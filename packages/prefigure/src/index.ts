import * as Comlink from "comlink";
import Worker from "./worker?worker&inline";
import SharedWorkerFactory from "./shared-worker?sharedworker";
import type { api } from "./worker";
import { PREFIG_WHEEL_FILENAME } from "./worker/wheel-filename";

declare const PREFIGURE_VERSION: string;

export type PrefigureMode = "svg" | "tactile";

export type PrefigureCompileResult = {
    svg: string;
    annotationsXml: string;
};

export type PrefigureCompileOptions = {
    mode?: PrefigureMode;
    indexURL?: string;
};

export type PrefigureKeepaliveHandle = {
    disconnect(): void;
};

export type PrefigureRuntimeStatus = {
    transport: "shared" | "dedicated";
    connected: boolean;
    initialized: boolean;
};

type WorkerApi = typeof api;
type PrefigureWorkerApi = Comlink.Remote<WorkerApi>;
type WorkerTransport = "shared" | "dedicated";

type WorkerConnection = {
    api: PrefigureWorkerApi;
    transport: WorkerTransport;
    port?: MessagePort;
};

let workerConnectionPromise: Promise<WorkerConnection> | null = null;
let resolvedTransport: WorkerTransport | null = null;
let initPromise: Promise<void> | null = null;
let initializedIndexUrl: string | null = null;

export const version: string = PREFIGURE_VERSION;
export const prefigWheelFilename = PREFIG_WHEEL_FILENAME;
const GLOBAL_SCOPE = globalThis as typeof globalThis & {
    prefigure?: typeof prefigure;
    initPrefigure?: typeof initPrefigure;
};

async function createWorkerConnection(): Promise<WorkerConnection> {
    const forceDedicated = !!(GLOBAL_SCOPE as any)
        .__DOENET_PREFIGURE_FORCE_DEDICATED__;
    if (!forceDedicated) {
        try {
            // Use a dedicated shared-worker entrypoint instead of self-loading this
            // module to avoid nested worker creation in SharedWorker scope.
            const sw = new SharedWorkerFactory();
            const port = sw.port;
            port.start();
            const workerApi = Comlink.wrap<WorkerApi>(
                port,
            ) as PrefigureWorkerApi;
            return { api: workerApi, transport: "shared", port };
        } catch {
            // SharedWorker creation can fail for cross-origin or browser policy reasons.
            // Fall back to a dedicated worker.
        }
    }

    const workerApi = Comlink.wrap<WorkerApi>(
        new Worker(),
    ) as PrefigureWorkerApi;
    return { api: workerApi, transport: "dedicated" };
}

function ensureWorkerConnection(): Promise<WorkerConnection> {
    if (!workerConnectionPromise) {
        workerConnectionPromise = createWorkerConnection().then((conn) => {
            resolvedTransport = conn.transport;
            return conn;
        });
    }
    return workerConnectionPromise;
}

function ensureWorkerApi(): Promise<PrefigureWorkerApi> {
    return ensureWorkerConnection().then((conn) => conn.api);
}

/**
 * Connect to the prefigure runtime worker without initializing Pyodide.
 */
export async function connectPrefigureSharedWorker(): Promise<PrefigureKeepaliveHandle> {
    const conn = await ensureWorkerConnection();
    return {
        disconnect() {
            if (conn.port) {
                conn.port.close();
                workerConnectionPromise = null;
            }
        },
    };
}

/**
 * Return runtime transport/connection status without opening a new connection.
 */
export function getPrefigureRuntimeStatus(): PrefigureRuntimeStatus {
    return {
        transport: resolvedTransport ?? "dedicated",
        connected: workerConnectionPromise !== null,
        initialized: initializedIndexUrl !== null,
    };
}

export function defaultPrefigureIndexUrl(): string {
    // Default to sibling assets relative to the module URL.
    return new URL("./assets/", import.meta.url).toString();
}

/**
 * Initialize the prefigure worker runtime.
 */
export async function initPrefigure(indexURL?: string) {
    // If initialization is already in progress or complete and no explicit URL
    // was given, reuse the existing initialization without a URL conflict check.
    // This prevents false conflicts when compilePrefigure() calls initPrefigure()
    // with undefined and import.meta.url resolves differently from the URL that
    // was passed during warmup (e.g., explicit PREFIGURE_INDEX_URL vs. CDN default).
    if (initPromise && indexURL === undefined) {
        return initPromise;
    }

    const effectiveIndexUrl = indexURL ?? defaultPrefigureIndexUrl();

    const normalizedIndexUrl = effectiveIndexUrl.endsWith("/")
        ? effectiveIndexUrl
        : `${effectiveIndexUrl}/`;

    if (initializedIndexUrl && normalizedIndexUrl !== initializedIndexUrl) {
        throw new Error(
            `Prefigure is already initialized with a different indexURL (${initializedIndexUrl}).`,
        );
    }

    if (initPromise) {
        return initPromise;
    }

    initPromise = (async () => {
        const workerApi = await ensureWorkerApi();
        await workerApi.init({
            indexURL: normalizedIndexUrl,
        });
        initializedIndexUrl = normalizedIndexUrl;
    })().catch((error) => {
        // Allow retries after transient initialization failures.
        initPromise = null;
        throw error;
    });

    return initPromise;
}

/**
 * Compile PreFigure XML into SVG and annotations XML.
 */
export async function compilePrefigure(
    source: string,
    options: PrefigureCompileOptions = {},
): Promise<PrefigureCompileResult> {
    const mode = options.mode ?? "svg";
    const indexURL = options.indexURL;

    await initPrefigure(indexURL);
    const workerApi = await ensureWorkerApi();
    const output = await workerApi.compile(mode, source);

    return {
        svg: output.svg,
        annotationsXml: output.annotations,
    };
}

export async function prefigure(
    source: string,
    options: PrefigureCompileOptions = {},
): Promise<PrefigureCompileResult> {
    return compilePrefigure(source, options);
}

if (!GLOBAL_SCOPE.prefigure) {
    GLOBAL_SCOPE.prefigure = prefigure;
}

if (!GLOBAL_SCOPE.initPrefigure) {
    GLOBAL_SCOPE.initPrefigure = initPrefigure;
}
