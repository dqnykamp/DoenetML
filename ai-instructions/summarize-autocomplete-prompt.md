
# Detail how autocompletion works in this codebase

## Goal

The goal is to detail how the code mirror autocompletion works based on the relationships in the schema.
This information will be later used to add new features to the autocompletion.

## Background: project structure

The project is set up as a monorepo with all workspaces in the packages directory. The relevant workspaces
for this task are

- static-assets: where the schema data and completion snippets are stored
- lsp: the language server that generates the completions
- lsp-tools: tools needed by the language server
- codemirror: the code mirror editor

## The schema data

The schema data can be found in static-assets/src/schema.ts.

For this task, we are interested in the autocompletions of schema elements names. (We can ignore autocompletions of property names and values.)

One important source of information will be the AutoCompleter class in lsp-tools/src/auto-completer/index.ts,
including
- the AutoCompleter's getCompletionItems method will be important
- the AutoCompleter's parentChildMap, which will determine what children to autocomplete inside parent elements
- the AutoCompleter's schemaTopAllowedElements, which will determine what elements are allowed in the root

The other important source of information is the codemirror extension in codemirror/src/extensions/lsp/plugin.ts.

## The task

Your task is to, starting with this information, thoroughly investigate how the autocompletion in code mirror plugin. Detail how it uses information and structures from the lsp and the schema to determine what elements to suggest as autocompletion.

Your report should reference line numbers of specific files to detail how information from the schema gets transformed into the right form that is used by the autocompleter, and how that information is used in code mirror.