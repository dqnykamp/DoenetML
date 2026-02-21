# DoenetML element-name autocomplete: end-to-end report

## Scope

This report focuses on **element name** completion only (tag names and closing tags), and traces the full flow:

1. schema generation in `static-assets`
2. schema indexing in `lsp-tools` (`AutoCompleter`)
3. completion decision logic in `getCompletionItems`
4. LSP request wiring in `lsp`
5. request/response and client-side filtering in CodeMirror

Attribute-name/value completion branches are mentioned only when needed for context.

---

## 1) Where schema data comes from, and what fields matter for element-name autocomplete

### 1.1 Schema build pipeline

- Schema generation entry point: `packages/static-assets/scripts/generate-schema.ts` calls `getSchema()` and writes JSON to `src/generated/doenet-schema.json` (`generate-schema.ts:1-13`).
- Build scripts invoke schema generation: `packages/static-assets/package.json` includes `build:assets` and `build:schema` scripts that run `scripts/generate-schema.ts` (`package.json:36-41`).
- Schema is exported for consumers by `packages/static-assets/src/schema.ts` via `doenetSchema` import/export (`schema.ts:1-11`).

### 1.2 How `getSchema()` constructs relationships used for element suggestions

Core logic in `packages/static-assets/scripts/get-schema.ts`:

- `getSchema()` starts by collecting component class metadata and removing classes marked `excludeFromSchema` (`get-schema.ts:162-174`).
- It builds `inheritedOrAdaptedTypes` so that child relationships include inheritance/adaptation expansions (`get-schema.ts:176-244`, plus helper logic in `checkIfInheritOrAdapt`/`checkIfInherit` at `get-schema.ts:572-665`).
- For each concrete component type, it computes:
	- `children` from child groups (+ inherited/adapted expansion)
	- optional `additionalSchemaChildren` handling
	- de-duplication of children
	(`get-schema.ts:290-337`).
- It sets `top` for each schema element as `!cClass.inSchemaOnlyInheritAs` when pushing the final element record (`get-schema.ts:425-432`).

For element-name completion, the most important schema fields are:

- `name`
- `children`
- `top`

These are exactly what later become the top-level and parent/child completion sets.

---

## 2) How schema is transformed for fast completion lookup in `AutoCompleter`

File: `packages/lsp-tools/src/auto-completer/index.ts`

### 2.1 Schema source used by default

- `AutoCompleter` imports `doenetSchema` from `@doenet/static-assets/schema` and defaults constructor schema to `doenetSchema.elements` (`index.ts:2`, `index.ts:38-41`).

### 2.2 `setSchema` creates completion-oriented indices

`setSchema(schema)` builds the structures used by element-name completion (`index.ts:56-93`):

- `schemaElementsByName`: `name -> full element schema` (`index.ts:66-68`)
- `schemaTopAllowedElements`: `schema.filter(e => e.top).map(e => e.name)` (`index.ts:69-71`)
- `parentChildMap`: `parentName -> Set(children)` (`index.ts:72-74`)
- `schemaLowerToUpper`: lowercase normalization map for element names (`index.ts:58-60`)

Supporting helpers used by completion logic:

- `_getAllowedChildren(elementName)` normalizes case and returns `children` (`index.ts:115-118`)
- `normalizeElementName(name)` provides case-insensitive canonicalization (`index.ts:165-167`)

---

## 3) Element-name completion logic in `getCompletionItems`

File: `packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts`

### 3.1 Cursor context and AST context extraction

At the top of `getCompletionItems`:

- converts row/col to offset when needed (`get-completion-items.ts:14-16`)
- inspects previous characters (`prevChar`, `prevPrevChar`, `prevNonWhitespaceChar`) (`get-completion-items.ts:18-30`)
- queries parse context via `nodeAtOffset` and `elementAtOffsetWithContext` to determine cursor position (`get-completion-items.ts:32-35`)

This cursor context controls all completion branches.

### 3.2 Top-level element suggestions (`schemaTopAllowedElements`)

Top-level elements are suggested in two root-like scenarios:

1. No containing node, unknown cursor, previous char is `<`:
	 - returns `schemaTopAllowedElements` as completion items (`get-completion-items.ts:37-42`)
2. In root text node and previous char is `<`:
	 - also returns `schemaTopAllowedElements` (`get-completion-items.ts:44-54`)

So the `top` field from schema flows directly to root element suggestions via `schemaTopAllowedElements`.

### 3.3 Child element suggestions inside parent elements (`children` relationships)

When cursor is in element body and user typed `<`:

- It computes `allowedChildren` from `this._getAllowedChildren(containingElement.node.name)` (`get-completion-items.ts:75-85`).
- If current element is closed, returns only allowed children (`get-completion-items.ts:86-89`).
- If parent is not closed, returns both close-tag suggestion and allowed children (`get-completion-items.ts:90-97`).

`_getAllowedChildren` resolves through schema-derived children maps (`index.ts:115-118`, backed by `setSchema` at `index.ts:72-74`).

### 3.4 Open-tag-name completion while user is typing partial name

When `cursorPosition === "openTagName"`:

- It takes the partial current text (`currentText = element.name.toLowerCase()`) (`get-completion-items.ts:118-121`).
- If tag is root-level (`!parent || parent.type === "root"`), candidates are `schemaTopAllowedElements` prefix-filtered by `currentText` (`get-completion-items.ts:122-129`).
- Otherwise, candidates are `_getAllowedChildren(parent.name)` prefix-filtered (`get-completion-items.ts:131-137`).

This is the key branch where **schema relationships + typed prefix** jointly determine element-name completions.

### 3.5 Close-tag suggestions (also element-name related)

Element-name completion also includes close-tag insertion:

- If cursor is in close tag name, suggests `/${element.name}>` (`get-completion-items.ts:63-71`).
- If user typed `</` in suitable context, suggests close tag similarly (`get-completion-items.ts:100-116`).

---

## 4) How LSP wires editor requests to `AutoCompleter`

### 4.1 LSP capabilities and trigger characters

File: `packages/lsp/src/index.ts`

- Server advertises completion provider and trigger characters `"<", ".", "$", "/", '"', "'"` (`index.ts:64-68`).

### 4.2 Completion request handler

File: `packages/lsp/src/features/completions.ts`

- `connection.onCompletion` looks up document info and calls `info.autoCompleter.getCompletionItems(params.position)` (`completions.ts:9-17`).
- Returned value is passed back directly as LSP completion response (`completions.ts:17`).

### 4.3 Per-document `AutoCompleter` lifecycle

File: `packages/lsp/src/features/validate.ts`

- On document content change, if doc has no info entry, a new `AutoCompleter` is created and stored (`validate.ts:54-60`).
- The latest text is loaded via `info.autoCompleter.setSource(change.document.getText())` (`validate.ts:61`).

This guarantees completion runs against the current parsed document state.

---

## 5) How CodeMirror invokes LSP completion and post-processes element suggestions

### 5.1 Worker bridge and trigger discovery

File: `packages/codemirror/src/extensions/lsp/worker.ts`

- During LSP init, worker stores server-provided completion trigger characters in `this.completionTriggers` (`worker.ts:31-42`).
- Completion requests call `lspConn.getCompletion({ textDocument, position, context })` (`worker.ts:114-129`).

### 5.2 Plugin request logic and gating

File: `packages/codemirror/src/extensions/lsp/plugin.ts`

- `getCompletions(context)` checks the char before cursor against `uniqueLanguageServerInstance.completionTriggers` (`plugin.ts:165-174`).
- If implicit trigger and preceding char is trigger-char, uses `TriggerCharacter`; otherwise `Invoked` (`plugin.ts:175-178`).
- It suppresses completion when invocation is implicit and there is neither word prefix nor trigger character (`plugin.ts:179-186`).

### 5.3 Request + response conversion

- Sends request via `uniqueLanguageServerInstance.getCompletionItems(...)` with computed position/context (`plugin.ts:187-195`).
- Converts LSP items to CodeMirror options (label/apply/type/sort/filter fields) (`plugin.ts:200-229`).

### 5.4 Client-side filtering and ordering

- Plugin computes a token match (`prefixMatch`) and if token is word-like, applies client prefix filter:
	- `filterText.toLowerCase().startsWith(word)` (`plugin.ts:231-241`)
- Then sorts to prioritize options whose `apply` starts with the exact typed case (`plugin.ts:242-252`).

This means final visible element-name suggestions are:

1. server candidates (already constrained by schema relationships),
2. then possibly further narrowed and re-ordered by CodeMirror token filtering.

---

## 6) Schema field → runtime structure → completion behavior mapping

| Schema field | Built in schema | Indexed in `AutoCompleter` | Used in completion branch |
|---|---|---|---|
| `top` | `elements.push(... top: !cClass.inSchemaOnlyInheritAs ...)` (`get-schema.ts:425-432`) | `schemaTopAllowedElements = schema.filter(e => e.top).map(...)` (`index.ts:69-71`) | Root `<` suggestions and root open-tag-name suggestions (`get-completion-items.ts:37-54`, `get-completion-items.ts:122-129`) |
| `children` | child groups + inheritance/adaptation + additional children (`get-schema.ts:290-337`) | `parentChildMap` and `_getAllowedChildren` (`index.ts:72-74`, `index.ts:115-118`) | Body `<` child suggestions and nested open-tag-name suggestions (`get-completion-items.ts:75-97`, `get-completion-items.ts:131-137`) |
| `name` | schema element identity (`get-schema.ts:425-427`) | `schemaElementsByName` + normalization maps (`index.ts:58-68`) | Parent/child lookup, case-insensitive matching (`get-completion-items.ts:118-137`) |

---

## 7) End-to-end sequence for element-name autocompletion

1. Build step generates `doenet-schema.json` from component metadata (`generate-schema.ts:1-13`, `get-schema.ts:162-435`).
2. `@doenet/static-assets/schema` exports this as `doenetSchema` (`schema.ts:1-11`).
3. `AutoCompleter` loads `doenetSchema.elements` and builds top/children lookup structures (`index.ts:38-41`, `index.ts:56-74`).
4. CodeMirror plugin updates LSP document text, then requests completions (`plugin.ts:103-110`, `plugin.ts:187-195`; `worker.ts:80-97`, `worker.ts:114-129`).
5. LSP `onCompletion` delegates to `autoCompleter.getCompletionItems(position)` (`completions.ts:9-17`).
6. `getCompletionItems` chooses element candidates based on cursor context + schema relationships (`get-completion-items.ts:37-137`).
7. CodeMirror applies additional token filtering/sorting before showing final options (`plugin.ts:231-258`).

---

## 8) Important implementation nuances for future feature work

- Element-name logic is strictly context-branch based (root/body/openTagName/closeTag), so new behaviors should be added to specific branches in `get-completion-items.ts` rather than globally.
- Parent-child legality is schema-driven at completion time via `_getAllowedChildren`; modifying schema generation rules (inheritance/adaptation/additional children) directly changes autocomplete candidates.
- Trigger characters are broader than element-name-only needs (`index.ts:66`), but element-name proposals are still controlled server-side by cursor context checks in `get-completion-items.ts`.
- Final UI list is not purely server output: CodeMirror performs additional token filtering/ranking (`plugin.ts:231-252`), which can hide/reorder server candidates.