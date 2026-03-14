# Prefigure Shared Worker Keepalive: Detailed Implementation Plan

## Goal

Implement a two-stage prefigure runtime architecture that:

1. Connects to a same-origin `SharedWorker` without initializing Pyodide.
2. Initializes Pyodide only when a page actually needs to compile prefigure (for example, when `prefigure.tsx` renders).
3. Exposes a lightweight keepalive API from `@doenet/doenetml` so non-Doenet pages on the same origin can keep the shared worker alive.

This allows fast startup on later pages while avoiding heavy warmup on pages that do not use prefigure.

## Current State Summary

- `@doenet/prefigure` currently creates a dedicated worker via `./worker?worker&inline` and initializes it in `initPrefigure(...)`.
- `@doenet/doenetml` prefigure renderer dynamically imports prefigure from `PREFIGURE_MODULE_URL`, then calls `initPrefigure(...)`.
- No separate lightweight connect-only path exists.
- `@doenet/doenetml` root export (`src/index.ts`) is heavy and should not be imported by external keepalive callers.

Relevant files:
- `packages/prefigure/src/index.ts`
- `packages/prefigure/src/worker/index.ts`
- `packages/doenetml/src/Viewer/renderers/prefigure.tsx`
- `packages/doenetml/src/Viewer/renderers/utils/prefigureConfig.ts`
- `packages/doenetml/src/index.ts`
- `packages/doenetml/package.json`

## High-Level Design

### Runtime model

Use one shared runtime worker per origin and per worker script identity.

- `connectPrefigureSharedWorker()`
  - Connect only.
  - No call to `init`.
  - Returns a handle with `disconnect()`.
- `initPrefigure(indexURL?)`
  - Reuses existing connection.
  - Performs one-time heavy initialization in the shared worker.
- `compilePrefigure(...)`
  - Ensures initialized runtime, then compiles.

### Fallback model

If `SharedWorker` cannot be used, fallback to the existing dedicated `Worker` path.

- Connect API remains available and becomes a no-op handle on dedicated worker path.
- Init and compile semantics remain unchanged.

### Keepalive export from `@doenet/doenetml`

Add a small subpath module that:

- Uses the same `PREFIGURE_MODULE_URL` resolution strategy as renderer config.
- Dynamically imports prefigure module URL.
- Calls prefigure `connectPrefigureSharedWorker()`.

External pages can import this subpath and keep the worker alive without loading Doenet viewer/editor.

## API Contract Changes

## `@doenet/prefigure`

Add exports:

- `connectPrefigureSharedWorker(): Promise<PrefigureKeepaliveHandle>`
- `type PrefigureKeepaliveHandle = { disconnect(): void }`

Optional (recommended for diagnostics):

- `getPrefigureRuntimeStatus(): { transport: "shared" | "dedicated"; connected: boolean; initialized: boolean }`

No breaking changes to existing API:

- `initPrefigure(indexURL?)`
- `compilePrefigure(source, options)`
- `prefigure(source, options)`

## `@doenet/doenetml`

Add subpath export:

- `@doenet/doenetml/prefigure-keepalive`

Exported function from that subpath:

- `connectPrefigureKeepAlive(): Promise<{ disconnect(): void }>`

## Detailed File-by-File Plan

### 1) `packages/prefigure/src/shared-worker.ts` (new)

Create a dedicated SharedWorker entrypoint.

Responsibilities:
- Hold singleton compiler instance in worker global scope.
- Expose API over each `onconnect` port via Comlink.
- Track number of active ports.
- Keep `initializedIndexUrl` guard semantics.

Suggested API exposed from worker:
- `connect()` (optional, returns simple metadata)
- `init(options)`
- `compile(mode, source)`
- `status()` (optional diagnostics)

Implementation notes:
- Use `self.onconnect = (event) => { const port = event.ports[0]; ... }`.
- Call `Comlink.expose(api, port)` and `port.start()`.
- Track `port` close/disconnect by listening to `messageerror` and explicit client `disconnect` RPC if needed.

## 2) `packages/prefigure/src/index.ts` (update)

Refactor worker connection logic into transport-aware connector.

Add internal types:
- `type WorkerTransport = "shared" | "dedicated"`
- `type WorkerConnection = { api: PrefigureWorkerApi; transport: WorkerTransport; disconnect?: () => void }`

Add module-level state:
- `workerConnectionPromise`
- `workerTransport`
- Existing `initPromise`, `initializedIndexUrl` retained

Add `connectPrefigureSharedWorker()`:
- Attempt shared worker first.
- If shared unavailable/fails, establish dedicated worker and return a handle.
- Do not call `api.init`.

Update `ensureWorkerApi()`:
- Reuse connection established by connect function.
- Preserve one connection per page module instance.

Update `initPrefigure(...)`:
- Use `ensureWorkerApi()` and call `api.init(...)`.
- Preserve current idempotency and URL mismatch guard.

Keep existing global API registration:
- `window.prefigure`
- `window.initPrefigure`

Optional:
- Add `window.connectPrefigureSharedWorker` only if desired for parity.

## 3) `packages/prefigure/src/worker/index.ts` (maybe unchanged)

If dedicated fallback reuses this file, no major change needed.

If adding shared-compatible status helpers, ensure worker API shape can be wrapped consistently.

## 4) `packages/prefigure/README.md` (update)

Document:
- Shared worker behavior and origin constraints.
- New connect-only API and intended use.
- Lifecycle caveats: worker can still be evicted by browser.

Include short usage snippet:

```js
import { connectPrefigureSharedWorker } from "@doenet/prefigure";

const keepAlive = await connectPrefigureSharedWorker();
// Later
keepAlive.disconnect();
```

## 5) `packages/doenetml/src/prefigure-keepalive.ts` (new)

Create lightweight wrapper module.

Responsibilities:
- Resolve prefigure module URL in same way as renderer config.
- Dynamically import the module URL.
- Call `connectPrefigureSharedWorker()` and return handle.

Implementation detail:
- Reuse `PREFIGURE_MODULE_URL` from `Viewer/renderers/utils/prefigureConfig.ts`.
- To avoid accidentally pulling renderer tree, ensure import path does not import React components.

Potential helper extraction:
- Move prefigure module URL constants to a new shared utility file under `packages/doenetml/src/utils/prefigureRuntimeConfig.ts`.
- Update renderer and keepalive module to import from this shared utility.

## 6) `packages/doenetml/package.json` (update)

Add explicit subpath export for lightweight API:

- `"./prefigure-keepalive": { "import": "./dist/prefigure-keepalive.js", "require": "./dist/prefigure-keepalive.js" }`

Keep existing wildcard export, but explicit subpath improves discoverability and stability.

## 7) `packages/doenetml/src/index.ts` (optional)

Do not add keepalive export to root barrel by default.

Reason:
- External pages should import `@doenet/doenetml/prefigure-keepalive` to avoid large root bundle.

If needed for convenience, add root re-export with clear docs about bundle size tradeoff.

## 8) `packages/doenetml/src/Viewer/renderers/prefigure.tsx` (optional update)

No functional changes required for this task.

Optional enhancement:
- Replace local prefigure module import helper with shared utility used by `prefigure-keepalive.ts` for consistency.

## Build and Bundling Requirements

## Ensure shared worker bundling works in Vite

For `@doenet/prefigure`:
- Add import for shared worker entry in `index.ts` using Vite worker query.
- Example pattern: `import SharedWorkerEntry from "./shared-worker?sharedworker&inline";`
- Construct with `new SharedWorkerEntry()` where appropriate.

Keep dedicated fallback:
- Existing `import Worker from "./worker?worker&inline";`

Verify build output includes both worker bundles and that CDN usage still resolves correctly.

## Keep connect path lightweight

Avoid eager heavy initialization in connect path.

- `connectPrefigureSharedWorker()` must not call `init`.
- If worker module itself imports heavy runtime eagerly, this is still acceptable for first pass, but note it may reduce perceived benefit.
- Future optimization: lazy-import compiler internals inside shared worker `init` handler.

## Testing Plan

## Unit tests in `@doenet/prefigure`

Add or update tests to cover:

1. `connectPrefigureSharedWorker()` does not call `init`.
2. `initPrefigure()` after connect performs one-time init.
3. Multiple connect calls share same connection object/module state.
4. URL mismatch guard still throws after successful init with another URL.
5. Fallback to dedicated worker when shared worker unavailable.

## Integration behavior checks

Manual checks in browser (dev server):

1. Open page with prefigure diagram:
- Expect initial warmup and then fast compile.

2. Reload same page:
- Expect faster startup if shared worker persisted.

3. Open second same-origin page without prefigure but with keepalive import:
- Worker should remain active.

4. Close prefigure page, keep keepalive page open, then open prefigure page again:
- Should remain fast.

5. Cross-origin navigation and return:
- Fast only if same-origin keepalive page remained open.

6. Browser without `SharedWorker` support:
- Should fallback to dedicated worker and still function.

## Optional e2e/cypress scenarios

- Add scenario with two tabs/windows if framework permits.
- At minimum, verify that keepalive call succeeds and prefigure still compiles afterward.

## Rollout Strategy

1. Implement API and subpath export behind no feature flag (safe additive).
2. Land docs plus basic tests.
3. Add optional telemetry logs during development:
- transport selected
- connect timestamp
- init duration
4. After validation, remove verbose logs or gate behind debug flag.

## Risks and Mitigations

1. SharedWorker unsupported or blocked.
- Mitigation: dedicated worker fallback.

2. Browser eviction despite active pages.
- Mitigation: design assumes best-effort persistence; keep cache headers strong.

3. Bundle accidentally heavy for keepalive import.
- Mitigation: explicit subpath module and dynamic import by URL.

4. Importing renderer utility drags React code.
- Mitigation: isolate prefigure runtime config into non-React utility module.

5. API drift between shared and dedicated worker implementations.
- Mitigation: common `WorkerApi` interface and shared tests.

## Acceptance Criteria

1. External same-origin page can run:
- `import { connectPrefigureKeepAlive } from "@doenet/doenetml/prefigure-keepalive"`
- `await connectPrefigureKeepAlive()`

2. The above does not initialize Pyodide until `initPrefigure` is called elsewhere.

3. Prefigure renderer behavior remains unchanged and still falls back gracefully if warmup fails.

4. On environments with shared worker support, warm runtime is reusable across same-origin pages while at least one connection remains alive.

5. On environments without support, no regressions in compile functionality.

## Suggested Task Breakdown for Handoff Agent

1. Implement shared worker entrypoint and connect-only API in `@doenet/prefigure`.
2. Refactor `index.ts` transport selection and fallback.
3. Add/adjust tests in `packages/prefigure/test`.
4. Add `@doenet/doenetml/prefigure-keepalive` subpath module and package export.
5. Update docs in `packages/prefigure/README.md` and optionally top-level docs.
6. Run targeted builds/tests:
- `npm run build -w @doenet/prefigure`
- `npm run build -w @doenet/doenetml`
- `npm run test -w @doenet/prefigure`

## Notes for Reviewer

- Confirm no breaking changes in existing prefigure API.
- Confirm keepalive subpath does not depend on viewer/editor bundles.
- Confirm fallback path works when `SharedWorker` is unavailable.
- Confirm URL guard and idempotency behavior remain intact.
