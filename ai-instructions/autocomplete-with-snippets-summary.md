# CodeMirror autocomplete architecture (with snippets)

This document explains how DoenetML autocomplete works end-to-end, including the snippet feature, with references to concrete source locations.

## 1) High-level request flow

When a user types in the `CodeMirror` component, autocomplete flows through four layers:

1. **Editor wiring (React + CodeMirror extension setup)**
   - `CodeMirror` enables `lspPlugin(documentId)` for editable editors: [packages/codemirror/src/CodeMirror.tsx#L71-L86](packages/codemirror/src/CodeMirror.tsx#L71-L86)
   - It generates a per-editor document URI and closes it on unmount: [packages/codemirror/src/CodeMirror.tsx#L45-L59](packages/codemirror/src/CodeMirror.tsx#L45-L59)

2. **CodeMirror LSP plugin (client-side adapter)**
   - A shared LSP instance is created once for all editors: [packages/codemirror/src/extensions/lsp/plugin.ts#L89-L90](packages/codemirror/src/extensions/lsp/plugin.ts#L89-L90)
   - The plugin registers CodeMirror autocompletion override: [packages/codemirror/src/extensions/lsp/plugin.ts#L351-L366](packages/codemirror/src/extensions/lsp/plugin.ts#L351-L366)

3. **Web worker LSP bridge**
   - Worker initializes JSON-RPC/LSP connection and capabilities: [packages/codemirror/src/extensions/lsp/utils/init-message-connection.ts#L8-L100](packages/codemirror/src/extensions/lsp/utils/init-message-connection.ts#L8-L100)
   - LSP bridge forwards `textDocument/completion` requests: [packages/codemirror/src/extensions/lsp/worker.ts#L114-L129](packages/codemirror/src/extensions/lsp/worker.ts#L114-L129)

4. **Language server + AutoCompleter**
   - LSP advertises completion trigger characters and installs completion support: [packages/lsp/src/index.ts#L58-L68](packages/lsp/src/index.ts#L58-L68), [packages/lsp/src/index.ts#L113-L118](packages/lsp/src/index.ts#L113-L118)
   - Completion handler delegates to `info.autoCompleter.getCompletionItems(...)`: [packages/lsp/src/features/completions.ts#L9-L18](packages/lsp/src/features/completions.ts#L9-L18)

## 2) How the editor/plugin side works

## 2.1 Document sync and diagnostics

- On document changes, plugin calls `setValue`, which updates the LSP document and polls diagnostics: [packages/codemirror/src/extensions/lsp/plugin.ts#L105-L127](packages/codemirror/src/extensions/lsp/plugin.ts#L105-L127)
- LSP diagnostics are converted to CodeMirror diagnostics and rendered as markdown HTML via `micromark`: [packages/codemirror/src/extensions/lsp/plugin.ts#L129-L173](packages/codemirror/src/extensions/lsp/plugin.ts#L129-L173)

## 2.2 Completion request triggering

- Plugin checks whether completion was explicit or caused by an LSP trigger character: [packages/codemirror/src/extensions/lsp/plugin.ts#L174-L187](packages/codemirror/src/extensions/lsp/plugin.ts#L174-L187)
- Trigger character list is learned from server capabilities during worker init: [packages/codemirror/src/extensions/lsp/worker.ts#L35-L43](packages/codemirror/src/extensions/lsp/worker.ts#L35-L43)
- If invocation is implicit and there is no word/trigger context, plugin returns `null` (no suggestions): [packages/codemirror/src/extensions/lsp/plugin.ts#L188-L195](packages/codemirror/src/extensions/lsp/plugin.ts#L188-L195)

## 2.3 Completion shaping on the client

- LSP items are mapped to CodeMirror `Completion`s with `label/detail/type/filterText/sortText`: [packages/codemirror/src/extensions/lsp/plugin.ts#L210-L243](packages/codemirror/src/extensions/lsp/plugin.ts#L210-L243)
- If LSP item has `textEdit.range`, plugin stores it and preserves original insert text (`_originalApplyText`) for filtering/sorting: [packages/codemirror/src/extensions/lsp/plugin.ts#L233-L240](packages/codemirror/src/extensions/lsp/plugin.ts#L233-L240)
- Prefix filtering and ranking use `_originalApplyText` so snippets still sort/filter correctly even when custom `apply` logic is used: [packages/codemirror/src/extensions/lsp/plugin.ts#L260-L287](packages/codemirror/src/extensions/lsp/plugin.ts#L260-L287), [packages/codemirror/src/extensions/lsp/plugin.ts#L399-L418](packages/codemirror/src/extensions/lsp/plugin.ts#L399-L418)
- If a `textEdit.range` exists, plugin replaces the default `apply` with a custom dispatch that applies exactly to that range: [packages/codemirror/src/extensions/lsp/plugin.ts#L290-L339](packages/codemirror/src/extensions/lsp/plugin.ts#L290-L339)

This range-based apply logic is critical for snippets, because snippet insertion often needs to replace from the opening `<` up to cursor position, not just insert at cursor.

## 3) How the LSP server decides completion items

## 3.1 Document lifecycle and AutoCompleter ownership

- `documentInfo` stores one `AutoCompleter` per URI: [packages/lsp/src/globals.ts#L37-L51](packages/lsp/src/globals.ts#L37-L51)
- On every text change, server creates (if needed) and updates `autoCompleter` with latest source: [packages/lsp/src/features/validate.ts#L54-L64](packages/lsp/src/features/validate.ts#L54-L64)

So completions always run against the latest parsed DoenetML in memory.

## 3.2 Completion entrypoint

- Completion request handler is intentionally thin and synchronous from stored state:
  - look up `documentInfo`
  - return `autoCompleter.getCompletionItems(params.position)`
  - [packages/lsp/src/features/completions.ts#L9-L18](packages/lsp/src/features/completions.ts#L9-L18)

## 4) Snippet feature: source of truth and runtime behavior

## 4.1 Where snippets come from

- Snippets are authored in generator script as a typed object: [packages/static-assets/scripts/generate-completion-snippets.ts#L6-L457](packages/static-assets/scripts/generate-completion-snippets.ts#L6-L457)
- Script writes JSON artifact consumed at runtime: [packages/static-assets/scripts/generate-completion-snippets.ts#L461-L472](packages/static-assets/scripts/generate-completion-snippets.ts#L461-L472)
- Runtime export is `COMPLETION_SNIPPETS` from generated JSON: [packages/static-assets/src/completion-snippets.ts#L1-L10](packages/static-assets/src/completion-snippets.ts#L1-L10)
- Package export path and build pipeline include this asset generation step: [packages/static-assets/package.json#L26-L40](packages/static-assets/package.json#L26-L40)

## 4.2 How snippets are indexed in AutoCompleter

- `AutoCompleter` imports `COMPLETION_SNIPPETS`: [packages/lsp-tools/src/auto-completer/index.ts#L1-L7](packages/lsp-tools/src/auto-completer/index.ts#L1-L7)
- On schema setup, it builds canonical element/attribute maps and then initializes snippets: [packages/lsp-tools/src/auto-completer/index.ts#L69-L107](packages/lsp-tools/src/auto-completer/index.ts#L69-L107)
- `_initializeSnippets`:
  - trims leading whitespace from snippet body
  - normalizes snippet target element to schema capitalization
  - skips snippets tied to unknown elements
  - indexes by normalized element in `snippetsByNormalizedElement`
  - [packages/lsp-tools/src/auto-completer/index.ts#L246-L277](packages/lsp-tools/src/auto-completer/index.ts#L246-L277)
- `_getSnippetsForElements` returns snippets for allowed element set and optional key prefix filter: [packages/lsp-tools/src/auto-completer/index.ts#L290-L311](packages/lsp-tools/src/auto-completer/index.ts#L290-L311)

## 4.3 How snippets become LSP completion items

- `createSnippetCompletionItems(...)` creates `CompletionItemKind.Snippet` with `textEdit.range/newText` and `filterText = key`: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L82-L106](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L82-L106)
- `createTextEditRange(...)` converts Doenet source offsets (1-based row/col) to LSP 0-based line/character: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L19-L36](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L19-L36)
- `formatSnippetWithIndent(...)` indents multiline snippet lines based on insertion column: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L48-L69](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L48-L69)

## 5) Context-aware completion logic (schema + snippets + tags + attrs)

Main logic lives in: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L134-L353](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L134-L353)

Key branches:

- **Root-level `<`**: offer top-level schema elements + snippets for those elements: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L161-L177](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L161-L177), [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L183-L200](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L183-L200)
- **Inside element body after `<`**: offer allowed children + snippets; also closing tag for non-closed parent: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L221-L258](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L221-L258)
- **After `</`**: offer closing tag: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L260-L276](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L260-L276)
- **Typing open tag name**: filter allowed elements by prefix, and snippets by snippet key prefix; snippet replacement range is from tag start `<` to cursor: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L278-L317](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L278-L317)
- **In opening tag / attribute name**: return allowed attribute names from schema: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L319-L327](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L319-L327)
- **In attribute value**: return allowed value enums, optionally adding quotes if cursor is right after `=`: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L329-L351](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L329-L351)

## 6) Trigger characters and protocol capabilities

- Server advertises trigger characters as `['<', '.', '$', '/', '"', "'"]`: [packages/lsp/src/index.ts#L65-L67](packages/lsp/src/index.ts#L65-L67)
- Client reads these triggers from initialized LSP connection: [packages/codemirror/src/extensions/lsp/worker.ts#L41-L41](packages/codemirror/src/extensions/lsp/worker.ts#L41-L41)
- Plugin uses this list to determine `CompletionTriggerKind.TriggerCharacter` vs invoked: [packages/codemirror/src/extensions/lsp/plugin.ts#L177-L187](packages/codemirror/src/extensions/lsp/plugin.ts#L177-L187)
- Client initialize capabilities set snippet support (`completionItem.snippetSupport: true`): [packages/codemirror/src/extensions/lsp/utils/init-message-connection.ts#L21-L35](packages/codemirror/src/extensions/lsp/utils/init-message-connection.ts#L21-L35)

## 7) Tests that exercise this behavior

Component tests (CodeMirror + worker + LSP): [packages/codemirror/cypress/component/autocomplete.cy.tsx#L11-L135](packages/codemirror/cypress/component/autocomplete.cy.tsx#L11-L135)

Coverage in this spec:

- element names in blank doc: [packages/codemirror/cypress/component/autocomplete.cy.tsx#L39-L59](packages/codemirror/cypress/component/autocomplete.cy.tsx#L39-L59)
- element names inside parent: [packages/codemirror/cypress/component/autocomplete.cy.tsx#L61-L77](packages/codemirror/cypress/component/autocomplete.cy.tsx#L61-L77)
- closing tag completion: [packages/codemirror/cypress/component/autocomplete.cy.tsx#L79-L89](packages/codemirror/cypress/component/autocomplete.cy.tsx#L79-L89)
- snippet insertion: [packages/codemirror/cypress/component/autocomplete.cy.tsx#L91-L104](packages/codemirror/cypress/component/autocomplete.cy.tsx#L91-L104)
- attribute name/value completions: [packages/codemirror/cypress/component/autocomplete.cy.tsx#L106-L134](packages/codemirror/cypress/component/autocomplete.cy.tsx#L106-L134)

## 8) Maintenance guide: common changes

## 8.1 Add or edit a snippet

1. Edit snippet definitions in [packages/static-assets/scripts/generate-completion-snippets.ts#L6-L457](packages/static-assets/scripts/generate-completion-snippets.ts#L6-L457)
2. Regenerate assets so `src/generated/completion-snippets.json` is updated (script wiring: [packages/static-assets/package.json#L39-L40](packages/static-assets/package.json#L39-L40))
3. Ensure snippet `element` names correspond to schema-valid elements; invalid element names are ignored in `_initializeSnippets`: [packages/lsp-tools/src/auto-completer/index.ts#L252-L260](packages/lsp-tools/src/auto-completer/index.ts#L252-L260)
4. Add/adjust Cypress coverage in [packages/codemirror/cypress/component/autocomplete.cy.tsx#L91-L104](packages/codemirror/cypress/component/autocomplete.cy.tsx#L91-L104)

## 8.2 Change completion behavior by cursor context

- Modify branch logic in [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L134-L353](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L134-L353)
- If changing snippet insertion ranges or formatting, update:
  - range conversion: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L19-L36](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L19-L36)
  - indentation formatting: [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L48-L69](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L48-L69)
  - CodeMirror-side range application: [packages/codemirror/src/extensions/lsp/plugin.ts#L290-L339](packages/codemirror/src/extensions/lsp/plugin.ts#L290-L339)

## 8.3 Add a new trigger character

1. Update server capability list: [packages/lsp/src/index.ts#L65-L67](packages/lsp/src/index.ts#L65-L67)
2. Verify plugin trigger path in [packages/codemirror/src/extensions/lsp/plugin.ts#L177-L187](packages/codemirror/src/extensions/lsp/plugin.ts#L177-L187)
3. Add an integration test in [packages/codemirror/cypress/component/autocomplete.cy.tsx](packages/codemirror/cypress/component/autocomplete.cy.tsx)

## 9) Practical mental model for maintainers

- **Authority for what is allowed**: schema in `@doenet/static-assets/schema` used by `AutoCompleter`.
- **Authority for snippet templates**: generated completion snippets data from static-assets.
- **Authority for context decisions**: `getCompletionItems` in lsp-tools.
- **Authority for insertion behavior in editor**: CodeMirror plugin conversion from LSP item to CodeMirror completion `apply`.

If autocomplete looks wrong, debug in this order:

1. `AutoCompleter.getCompletionItems(...)` branch selection and returned items.
2. LSP completion transport (`features/completions.ts`, worker bridge).
3. CodeMirror plugin filtering/sorting and `textEdit` application.
4. Cypress component test reproduction.

## 10) Quick start checklist (first-time contributors)

Use this when making your first autocomplete/snippet change.

1. **Pick the change type**
   - New snippet template only -> edit [packages/static-assets/scripts/generate-completion-snippets.ts](packages/static-assets/scripts/generate-completion-snippets.ts)
   - Completion behavior change (context/rules) -> edit [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts)
   - Client-side filtering/apply behavior -> edit [packages/codemirror/src/extensions/lsp/plugin.ts](packages/codemirror/src/extensions/lsp/plugin.ts)

2. **Keep schema constraints in mind**
   - Element and attribute legality comes from schema-backed maps in [packages/lsp-tools/src/auto-completer/index.ts#L69-L107](packages/lsp-tools/src/auto-completer/index.ts#L69-L107)
   - Snippets with unknown elements are dropped during initialization: [packages/lsp-tools/src/auto-completer/index.ts#L252-L260](packages/lsp-tools/src/auto-completer/index.ts#L252-L260)

3. **Regenerate snippet assets if snippet data changed**
   - Snippet JSON is generated by [packages/static-assets/scripts/generate-completion-snippets.ts#L461-L472](packages/static-assets/scripts/generate-completion-snippets.ts#L461-L472)
   - Build wiring that runs generation is in [packages/static-assets/package.json#L39-L40](packages/static-assets/package.json#L39-L40)

4. **Validate insertion/range behavior**
   - Snippet ranges and indentation are built in [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L19-L106](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L19-L106)
   - Those ranges are applied in CodeMirror via custom `apply`: [packages/codemirror/src/extensions/lsp/plugin.ts#L290-L339](packages/codemirror/src/extensions/lsp/plugin.ts#L290-L339)

5. **Run/inspect integration tests**
   - Main autocomplete component tests: [packages/codemirror/cypress/component/autocomplete.cy.tsx](packages/codemirror/cypress/component/autocomplete.cy.tsx)
   - Verify at least: element name completion, close tag completion, snippet insertion, attribute name/value completion.

6. **When behavior is surprising, trace this sequence**
   - `CodeMirror` extension registration -> [packages/codemirror/src/CodeMirror.tsx#L71-L86](packages/codemirror/src/CodeMirror.tsx#L71-L86)
   - Plugin completion request/reshaping -> [packages/codemirror/src/extensions/lsp/plugin.ts#L174-L347](packages/codemirror/src/extensions/lsp/plugin.ts#L174-L347)
   - LSP completion handler -> [packages/lsp/src/features/completions.ts#L9-L18](packages/lsp/src/features/completions.ts#L9-L18)
   - AutoCompleter decision logic -> [packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L134-L353](packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts#L134-L353)
