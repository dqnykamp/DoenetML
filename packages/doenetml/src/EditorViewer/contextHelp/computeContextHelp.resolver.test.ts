/**
 * Integration tests that exercise `computeContextHelp` with a real
 * `RustResolverAdapter` so multi-part property refs (`$a.b.c`) resolve
 * through the actual reference graph rather than the first-segment-only
 * JS fallback.
 *
 * Skipped when the WASM module can't load (e.g. when worker-rust is
 * not built).
 */
import { describe, expect, it } from "vitest";
import {
    AutoCompleter,
    RustResolverAdapter,
    type RustResolverCore,
} from "@doenet/lsp-tools";
import { doenetSchema } from "@doenet/static-assets/schema";
import {
    buildSchemaElementsByName,
    computeContextHelp,
} from "./computeContextHelp";
import type { HelpContent } from "./types";

let PublicDoenetMLCore: any;
let wasmAvailable = false;

try {
    const mod = await import("@doenet/doenetml-worker-rust");
    const fs = await import("node:fs");
    const path = await import("node:path");
    const wasmPath = path.resolve(
        import.meta.dirname,
        "../../../../doenetml-worker-rust/dist/lib_doenetml_worker_bg.wasm",
    );
    const wasmBytes = fs.readFileSync(wasmPath);
    mod.initSync(wasmBytes);
    PublicDoenetMLCore = mod.PublicDoenetMLCore;
    wasmAvailable = true;
} catch (e) {
    console.warn(
        "Skipping context-help resolver integration tests — could not load WASM:",
        (e as Error).message,
    );
}

const SCHEMA_MAP = buildSchemaElementsByName(
    doenetSchema.elements,
    doenetSchema.aliasedElements,
);

const TAKES_INDEX_COMPONENT_TYPES = new Set(
    doenetSchema.elements.filter((el) => el.takesIndex).map((el) => el.name),
);

function helpAtWithResolver(source: string, offset: number): HelpContent {
    const completer = new AutoCompleter(source);
    const adapter = new RustResolverAdapter(completer.sourceObj, {
        core: PublicDoenetMLCore.new() as RustResolverCore,
        takesIndexComponentTypes: TAKES_INDEX_COMPONENT_TYPES,
    });
    const wired = new AutoCompleter(undefined, undefined, {
        sourceObj: completer.sourceObj,
        rustResolverAdapter: adapter,
    });
    return computeContextHelp(wired, offset, SCHEMA_MAP);
}

describe.skipIf(!wasmAvailable)(
    "computeContextHelp — multi-part refs (with resolver)",
    () => {
        it("resolves a chained ref `$repeat.element.property`", () => {
            // `$rep.point1.x` walks: rep → point1 → x. The fallback path
            // (no resolver) would mis-resolve to a property of `rep`.
            const source = `<repeat for="1 2 3" name="rep" valueName="i">
                <point name="point1">($i, 0)</point>
            </repeat>
            $rep[1].point1.x`;
            // Cursor at end of "x".
            const help = helpAtWithResolver(source, source.length);
            expect(help.kind).toBe("property");
            if (help.kind === "property") {
                expect(help.elementName).toBe("point");
                expect(help.propertyName.toLowerCase()).toBe("x");
            }
        });
    },
);
