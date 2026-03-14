/**
 * Lightweight keepalive module for the prefigure shared worker.
 *
 * Import this module from a same-origin page that does not render prefigure
 * diagrams directly but wants to keep the shared worker alive across page
 * navigations:
 *
 * ```ts
 * import { connectPrefigureKeepAlive } from "@doenet/doenetml/prefigure-keepalive";
 *
 * const handle = await connectPrefigureKeepAlive();
 * // Later, when the page is done:
 * handle.disconnect();
 * ```
 *
 * This does **not** initialize Pyodide; it only establishes a port connection
 * to the shared worker so the browser keeps it alive.  The worker will be
 * initialized the first time a prefigure diagram is rendered on any
 * same-origin page that uses `@doenet/doenetml`.
 */

import {
    PREFIGURE_MODULE_URL,
    PREFIGURE_INDEX_URL,
} from "./Viewer/renderers/utils/prefigureConfig";

type PrefigureModule = typeof import("@doenet/prefigure");

let prefigureModulePromise: Promise<PrefigureModule> | null = null;

async function getPrefigureModule(): Promise<PrefigureModule> {
    if (!prefigureModulePromise) {
        prefigureModulePromise = import(
            /* @vite-ignore */ PREFIGURE_MODULE_URL
        ) as Promise<PrefigureModule>;
    }
    return prefigureModulePromise;
}

/**
 * Connect to the prefigure shared worker and return a keepalive handle.
 *
 * The returned `disconnect()` method can be called to explicitly release this
 * page's hold on the shared worker.  Under normal circumstances you do not
 * need to call it — the browser will close the port automatically when the
 * page unloads.
 */
export async function connectPrefigureKeepAlive(): Promise<{
    disconnect(): void;
}> {
    const mod = await getPrefigureModule();
    return mod.connectPrefigureSharedWorker();
}

export { PREFIGURE_MODULE_URL, PREFIGURE_INDEX_URL };
