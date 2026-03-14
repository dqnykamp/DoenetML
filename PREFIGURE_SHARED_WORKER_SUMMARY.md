# PreFigure SharedWorker Summary

## What Was Implemented

The `@doenet/prefigure` runtime now attempts to use a SharedWorker first, with fallback to a dedicated worker.

Implementation highlights:

- `packages/prefigure/src/index.ts`:
  - Tries `SharedWorkerFactory` (`./shared-worker?sharedworker`) first.
  - Falls back to dedicated worker (`./worker?worker&inline`) if SharedWorker creation fails or if forced dedicated mode is enabled.
  - Exposes runtime status and shared-worker keepalive entry points.
- `packages/prefigure/src/shared-worker.ts`:
  - Dedicated SharedWorker entrypoint.
  - Maintains one compiler/runtime instance per worker script identity.
  - Lazily imports compiler code after setting a worker-scope shim for compatibility with `speech-rule-engine` environment detection.
- `packages/doenetml/src/prefigure-keepalive.ts`:
  - Provides a light keepalive connection to keep the SharedWorker alive between page transitions.

## Design Decisions and Rationale

1. Shared-first, dedicated fallback
- Goal: get fast warm performance when SharedWorker is allowed.
- Reason: Pyodide + compiler initialization is expensive; reusing one runtime can remove repeated startup cost.

2. Keep behavior resilient
- SharedWorker startup is wrapped in `try/catch`.
- If browser policy/origin rules block SharedWorker, rendering still works via dedicated worker.

3. Runtime URL resolution relative to module URL
- Worker and asset URLs are resolved from `import.meta.url`.
- This makes npm/CDN packaging self-contained and relocatable.

4. Preserve compatibility with existing integrations
- Existing call sites can continue to call `initPrefigure`/`compilePrefigure` without needing to know transport internals.

## How It Performs When It Works

When SharedWorker is available (same-origin scenario):

- First load is faster than repeated dedicated-worker initialization.
- Additional diagrams/tabs can be much faster because the runtime is already initialized.
- Keepalive can reduce worker eviction risk between navigations.

## Key Limitation

SharedWorker is constrained by browser origin rules.

In practical deployments, Doenet is often embedded on arbitrary third-party websites, where:

- The embedding page/iframe document origin is the customer's domain.
- `@doenet/standalone` and `@doenet/prefigure` are loaded from `cdn.jsdelivr.net`.
- `@doenet/prefigure` resolves worker URLs from its CDN module URL.

Result:

- The app attempts to create a SharedWorker at a CDN URL from a non-CDN document origin.
- Browsers block that cross-origin SharedWorker creation.
- Code falls back to dedicated worker.

## Why This Matters for Realistic Usage

For the common zero-setup embed model (customer site + CDN packages), SharedWorker is not reliably available.

That means:

- Cross-frame/shared-runtime speedups are usually not realized.
- Each embedding context pays dedicated-worker startup costs.
- The shared-worker design improves controlled same-origin deployments, but does not solve performance for the dominant third-party embed scenario.

## Practical Conclusion

The SharedWorker architecture is technically correct and beneficial in same-origin deployments, but it is not a universal performance solution for real-world Doenet embedding patterns where customers host documents on their own domains and consume packages directly from a public CDN.
