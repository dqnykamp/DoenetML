/*
 * SharedWorker entrypoint for the PreFigure runtime.
 *
 * One shared compiler instance is maintained per worker script identity.
 * Each connecting page receives its own MessagePort; when all ports are closed
 * (i.e. all connected pages have unloaded or explicitly disconnected) the
 * browser may evict the worker.
 */

import { expose } from "comlink";
import type { PreFigureCompiler } from "./worker/compiler";

// Some third-party libs (notably speech-rule-engine internals) detect workers
// by checking for DedicatedWorkerGlobalScope. In SharedWorker context that
// symbol is normally absent, which can incorrectly route code into Node paths.
// Define a compatibility alias before loading the heavy compiler module.
if (
    typeof (globalThis as any).DedicatedWorkerGlobalScope === "undefined" &&
    typeof (globalThis as any).SharedWorkerGlobalScope !== "undefined"
) {
    (globalThis as any).DedicatedWorkerGlobalScope = (
        globalThis as any
    ).SharedWorkerGlobalScope;
}

let compilerPromise: Promise<PreFigureCompiler> | null = null;

async function getCompiler(): Promise<PreFigureCompiler> {
    if (!compilerPromise) {
        compilerPromise = import("./worker/compiler").then(
            ({ PreFigureCompiler }) => new PreFigureCompiler(),
        );
    }
    return compilerPromise;
}

const api = {
    init: async (...args: Parameters<PreFigureCompiler["init"]>) => {
        const compiler = await getCompiler();
        return compiler.init(...args);
    },
    compile: async (...args: Parameters<PreFigureCompiler["compile"]>) => {
        const compiler = await getCompiler();
        return compiler.compile(...args);
    },
};

// `self` in a SharedWorker is SharedWorkerGlobalScope, which lives in
// lib.webworker.d.ts (incompatible with lib.dom.d.ts in the same compilation).
// We declare only the subset we need to avoid the lib conflict.
interface SharedWorkerSelf {
    onconnect: ((event: MessageEvent) => void) | null;
}
const sharedWorkerSelf = self as unknown as SharedWorkerSelf;

sharedWorkerSelf.onconnect = (event: MessageEvent) => {
    const port = event.ports[0];
    expose(api, port);
    port.start();
};
