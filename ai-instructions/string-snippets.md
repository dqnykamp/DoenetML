# String Snippets Implementation Plan

## Goal

Support snippets with `element: "string"` that insert plain text without XML tags. These snippets should be available when string content is allowed in the current context.

## Background

Currently, snippets are matched against schema element names. String snippets need different logic since "string" isn't a real element name. Instead, each schema element has an `acceptsStringChildren` boolean property that determines if text content is allowed.

## Rules for String Snippet Availability

- **At root level (or no parent)**: String snippets are always accepted
- **Inside an element**: String snippets are accepted only if the parent element's schema has `acceptsStringChildren: true`

## Implementation Changes

### 1. In `packages/lsp-tools/src/auto-completer/index.ts`

#### Modify `_initializeSnippets()` method

Add special handling for `element === "string"` before the normal element validation:

```typescript
private _initializeSnippets() {
    this.snippetsByNormalizedElement.clear();

    Object.entries(COMPLETION_SNIPPETS).forEach(([key, snippet]) => {
        const rawSnippet = snippet.snippet ?? "";
        const trimmedSnippet = rawSnippet.trimStart();
        
        // Check for string element (special case - not a real element)
        if (snippet.element === "string") {
            const processed: ProcessedSnippet = {
                key,
                element: "string",
                normalizedElement: "string",
                snippet: trimmedSnippet,
                description: snippet.description,
            };
            if (!this.snippetsByNormalizedElement.has("string")) {
                this.snippetsByNormalizedElement.set("string", []);
            }
            this.snippetsByNormalizedElement.get("string")!.push(processed);
            return; // Skip normal element validation
        }
        
        // Continue with normal element normalization...
        const normalizedElement = this.normalizeElementName(snippet.element);
        // ... rest of existing code
    });
}
```

#### Add new helper methods

```typescript
/**
 * Check if string snippets should be included for the given parent element.
 * String snippets are allowed at root level or when the parent accepts string children.
 * 
 * @param parentElement - The parent element to check, or undefined for root level
 * @returns true if string snippets should be included
 */
_shouldIncludeStringSnippets(parentElement?: DastElementV6): boolean {
    if (!parentElement) {
        return true; // Root level accepts strings
    }
    const normalizedParent = this.normalizeElementName(parentElement.name);
    return this.schemaElementsByName[normalizedParent]?.acceptsStringChildren ?? false;
}

/**
 * Get string snippets, optionally filtered by typed prefix.
 * 
 * @param typedPrefix - The text typed after `<` (used for prefix filtering)
 * @returns Array of ProcessedSnippets with element "string"
 */
_getStringSnippets(typedPrefix: string = ""): ProcessedSnippet[] {
    const snippets = this.snippetsByNormalizedElement.get("string") || [];
    if (typedPrefix) {
        const prefixLower = typedPrefix.toLowerCase();
        return snippets.filter((s) => s.key.toLowerCase().startsWith(prefixLower));
    }
    return snippets;
}
```

### 2. In `packages/lsp-tools/src/auto-completer/methods/get-completion-items.ts`

Update three contexts where snippets are retrieved. **Do not update** the `openTagName` context (when user is typing an element name).

#### Context 1: Fresh `<` with no containing node (around line 170)

```typescript
if (!containingNode && cursorPosition === "unknown" && prevChar === "<") {
    const schemaItems = this.schemaTopAllowedElements.map((name) => ({
        label: name,
        kind: CompletionItemKind.Property,
    }));

    const allowedElementsSet = new Set(this.schemaTopAllowedElements);
    const snippets = this._getSnippetsForElements(allowedElementsSet);
    const stringSnippets = this._shouldIncludeStringSnippets() 
        ? this._getStringSnippets() 
        : [];
    const allSnippets = [...snippets, ...stringSnippets];
    
    const snippetItems = createSnippetCompletionItems(
        this.sourceObj,
        allSnippets,  // Changed from snippets
        offset - 1,
        offset,
    );

    return [...schemaItems, ...snippetItems];
}
```

#### Context 2: Fresh `<` at root in text node (around line 185)

```typescript
if (!element && containingNode && containingNode.type === "text") {
    if (prevChar === "<") {
        const schemaItems = this.schemaTopAllowedElements.map((name) => ({
            label: name,
            kind: CompletionItemKind.Property,
        }));

        const allowedElementsSet = new Set(this.schemaTopAllowedElements);
        const snippets = this._getSnippetsForElements(allowedElementsSet);
        const stringSnippets = this._shouldIncludeStringSnippets()
            ? this._getStringSnippets()
            : [];
        const allSnippets = [...snippets, ...stringSnippets];
        
        const snippetItems = createSnippetCompletionItems(
            this.sourceObj,
            allSnippets,  // Changed from snippets
            offset - 1,
            offset,
        );

        return [...schemaItems, ...snippetItems];
    }
    return [];
}
```

#### Context 3: Fresh `<` inside element body (around line 220)

```typescript
if (
    cursorPosition === "body" &&
    containingElement.node &&
    prevChar === "<"
) {
    const allowedChildrenNames = this._getAllowedChildren(
        containingElement.node.name,
    );
    const allowedChildrenSet = new Set(allowedChildrenNames);

    const schemaItems = allowedChildrenNames.map((name) => ({
        label: name,
        kind: CompletionItemKind.Property,
    }));

    const snippets = this._getSnippetsForElements(allowedChildrenSet);
    const stringSnippets = this._shouldIncludeStringSnippets(containingElement.node)
        ? this._getStringSnippets()
        : [];
    const allSnippets = [...snippets, ...stringSnippets];
    
    const snippetItems = createSnippetCompletionItems(
        this.sourceObj,
        allSnippets,  // Changed from snippets
        offset - 1,
        offset,
    );

    const completionItems = [...schemaItems, ...snippetItems];
    // ... rest of the code
}
```

#### Context 4: `openTagName` - DO NOT MODIFY

When the user is already typing an element name (`<gra|`), **do not** include string snippets. They are looking for element completions, not text snippets.

## Key Design Decisions

1. **Storage**: String snippets are stored under the key `"string"` in `snippetsByNormalizedElement` map
2. **Separate retrieval**: New `_getStringSnippets()` method for clarity and to avoid confusion with element snippets
3. **Limited context**: Only show string snippets immediately after typing `<`, not while typing element names
4. **Parent checking**: Use `_shouldIncludeStringSnippets()` to check if the parent element allows string children via `acceptsStringChildren` property

## Testing Considerations

1. Test string snippet at root level (should appear)
2. Test string snippet inside element with `acceptsStringChildren: true` (should appear)
3. Test string snippet inside element with `acceptsStringChildren: false` (should NOT appear)
4. Test that string snippets don't appear while typing element names (e.g., `<gra|`)
5. Test prefix filtering works for string snippets

## Example Snippet

```typescript
"just-some-text": {
    element: "string",
    snippet: `Hello, this is just some text with no DoenetML elements.`,
    description: "Just some text",
}
```

When triggered after `<`, this will replace the `<` with the plain text (no tags).
