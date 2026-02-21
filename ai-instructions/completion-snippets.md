# Task: add autocompletions based on snippets data

## Goal

The goal is to add the snippets from packages/static-assets/src/completion-snippets.ts 
to the autocompletion in the code mirror editor of packages/codemirror/src/extensions/lsp.
Currently, we have autocompletions based on the schema working properly.
The idea is to include additional autocompletions from the snippets data.

## Background: project structure

The project is set up as a monorepo with all workspaces in the packages directory. The relevant workspaces
for this task are

- static-assets: where the schema data and completion snippets are stored
- lsp: the language server that generates the completions
- lsp-tools: tools needed by the language server
- codemirror: the code mirror editor

## Background: autocompletions based on schema

The way that current autocompletions work is based on the schema is detailed in the file `summarize-autocomplete.md`.

## New features of autocompletions based on snippets data

### Feature 1: the replacement text will be the snippet

One new feature that we need with snippets is that the text that will replace what the user typed will different from what is typed and selected.

In the completion snippets data structure, the key of each entry is the label, or what will be compared with what the user types.
When the autocompletion is selected, the text that will be inserted is the string from the field `snippet`.

### Feature 2: the snippets available is based on the element field

In the previous autocomplete scheme, the autocompleter allows element names based on the schema.
- When there is no containing element, then the elements from `schemaTopAllowedElements` are used
- When there is a containing element, then `_getAllowedChildren` is used with that element's name to determine the list of elements to include.

To determine how snippets are allowed, two steps are required
1. Determine the list of elements allowed according to the schema, i.e., get the same list of allowed elements from the current algorithm using the schema.
2. Look up all snippets where the `element` field is one of the element names allowed in step 1. Those are the snippets that will be added to autocomplete list.

Note: these new autocomplete snippet options will be added to the list of options. The previous autocomplete suggestions from the schema will still be included as well.

## The task

Implement the feature of snippet autocompletions so that when a user types characters after a `<`, the list of autocompletions includes the snippet keys. If one of those keys is selected, replace the text, including the starting `<` with the string from `snippet` field.