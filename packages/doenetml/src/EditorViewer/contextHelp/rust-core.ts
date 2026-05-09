import init, { PublicDoenetMLCore } from "@doenet/doenetml-worker-rust";
// @ts-expect-error — Vite ?url resolves to a string at build time
import WASM_URL from "@doenet/doenetml-worker-rust/lib_doenetml_worker_bg.wasm?url";

/**
 * Browser-only WASM init for the editor's context-sensitive help resolver.
 * The LSP has its own (more elaborate) variant in `packages/lsp/src/rust-core.ts`
 * that handles vitest/Node fallback paths; the editor only runs in browsers,
 * so this stays minimal.
 */

let wasmInitPromise: Promise<unknown> | null = null;

async function ensureWasmInitialized(): Promise<void> {
    if (!wasmInitPromise) {
        wasmInitPromise = init(WASM_URL);
    }
    await wasmInitPromise;
}

/**
 * Lazily initialize the WASM module and return a fresh `PublicDoenetMLCore`.
 *
 * One core is created per AutoCompleter instance — the resolver adapter
 * keeps Rust-side source state and JS-side index mappings aligned, and
 * sharing a core across documents would muddle that.
 */
export async function getRustCoreForEditor(): Promise<PublicDoenetMLCore> {
    await ensureWasmInitialized();
    const core = PublicDoenetMLCore.new();
    // Flags must be set before the core can process source. An empty flags
    // object is sufficient for path resolution (the only thing we use the
    // core for here).
    core.set_flags("{}");
    return core;
}
