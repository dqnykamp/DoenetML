import { DastMacroPathPart } from "@doenet/parser";

/**
 * The path parts an author wrote as a *prop access* (`$a.b`) rather than as a namespace
 * path segment (`$(a/b)`).
 *
 * v0.6 kept the two apart in the DAST: slashes became `path` parts, dots became a chain of
 * `accessedProp` macros. `upgradePathSlashesToDots` flattens both into one v0.7 path, and
 * after that nothing in the tree distinguishes them — but the distinction matters here. A
 * namespace segment can name a component that `assignNames` created, so rewriting it is
 * right; a prop access never can, because v0.6 dot notation reached public state
 * variables and nothing else. `applyRefRenames` therefore leaves a prop access alone —
 * rewriting one would turn `$p.y`, the point's y-coordinate, into `$p.x[2]` whenever `x`
 * and `y` happened to be assigned somewhere else in the document.
 *
 * Held in a `WeakSet` keyed on the part itself, so there is nothing to clear between
 * documents and no converter-only field on a shared `@doenet/parser` type.
 */
const propAccessParts = new WeakSet<DastMacroPathPart>();

/** Record that `part` was written after a `.`, so it names a prop and not a component. */
export function markAsPropAccess(part: DastMacroPathPart) {
    propAccessParts.add(part);
}

/** Whether `part` was written after a `.`. See {@link markAsPropAccess}. */
export function isPropAccess(part: DastMacroPathPart): boolean {
    return propAccessParts.has(part);
}
