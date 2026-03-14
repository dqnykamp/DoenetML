import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => {
    return {
        initSpy: vi.fn(async () => {}),
        compileSpy: vi.fn(async () => ({
            svg: "<svg/>",
            annotations: "<annotations/>",
        })),
        wrapSpy: vi.fn(),
    };
});

vi.mock("comlink", () => {
    mocks.wrapSpy.mockImplementation(() => ({
        init: mocks.initSpy,
        compile: mocks.compileSpy,
    }));

    return {
        wrap: mocks.wrapSpy,
        expose: vi.fn(),
    };
});

vi.mock("../src/worker?worker&inline", () => ({
    default: class MockWorker {
        // Mock constructor intentionally empty; Comlink.wrap is mocked.
    },
}));

vi.mock("../src/shared-worker?sharedworker", () => ({
    default: class MockSharedWorkerFactory {
        port: MessagePort;
        constructor() {
            if ((globalThis as any).__PREFIGURE_FORCE_SHAREDWORKER_FAIL__) {
                throw new Error("shared worker disabled for test");
            }
            const channel = new MessageChannel();
            this.port = channel.port1;
        }
    },
}));

vi.mock("../src/worker/compiler", () => ({
    PREFIG_WHEEL_FILENAME: "prefig-test.whl",
}));

describe("@doenet/prefigure API", () => {
    beforeEach(() => {
        vi.resetModules();
        mocks.initSpy.mockClear();
        mocks.compileSpy.mockClear();
        mocks.wrapSpy.mockClear();
        (globalThis as any).__PREFIGURE_FORCE_SHAREDWORKER_FAIL__ = false;
    });

    it("returns an assets URL for default index path", async () => {
        const mod = await import("../src/index");
        const url = mod.defaultPrefigureIndexUrl();

        expect(url.endsWith("/assets/")).toBe(true);
    });

    it("initializes at most once for repeated same URL", async () => {
        const mod = await import("../src/index");

        await mod.initPrefigure("https://cdn.example.com/prefigure-assets/");
        await mod.initPrefigure("https://cdn.example.com/prefigure-assets/");

        expect(mocks.wrapSpy).toHaveBeenCalledTimes(1);
        expect(mocks.initSpy).toHaveBeenCalledTimes(1);
    });

    it("throws when reinitialized with a different indexURL", async () => {
        const mod = await import("../src/index");

        await mod.initPrefigure("https://cdn.example.com/assets-a/");

        await expect(
            mod.initPrefigure("https://cdn.example.com/assets-b/"),
        ).rejects.toThrow(/already initialized with a different indexURL/i);
    });

    it("allows retry after transient init failure", async () => {
        const mod = await import("../src/index");

        mocks.initSpy.mockReset();
        mocks.initSpy
            .mockRejectedValueOnce(new Error("transient init failure"))
            .mockResolvedValueOnce(undefined);

        await expect(
            mod.initPrefigure("https://cdn.example.com/retry-assets/"),
        ).rejects.toThrow(/transient init failure/);

        await expect(
            mod.initPrefigure("https://cdn.example.com/retry-assets/"),
        ).resolves.toBeUndefined();

        expect(mocks.initSpy).toHaveBeenCalledTimes(2);
    });

    it("compilePrefigure delegates to worker compile", async () => {
        const mod = await import("../src/index");

        const result = await mod.compilePrefigure("<diagram />");

        expect(mocks.compileSpy).toHaveBeenCalledWith("svg", "<diagram />");
        expect(result).toEqual({
            svg: "<svg/>",
            annotationsXml: "<annotations/>",
        });
    });

    it("coalesces concurrent init/compile calls into one worker init", async () => {
        const mod = await import("../src/index");

        const calls = await Promise.all([
            mod.initPrefigure("https://cdn.example.com/race-assets/"),
            mod.compilePrefigure("<diagram id='a' />"),
            mod.compilePrefigure("<diagram id='b' />"),
        ]);

        expect(calls[1]).toEqual({
            svg: "<svg/>",
            annotationsXml: "<annotations/>",
        });
        expect(calls[2]).toEqual({
            svg: "<svg/>",
            annotationsXml: "<annotations/>",
        });

        expect(mocks.wrapSpy).toHaveBeenCalledTimes(1);
        expect(mocks.initSpy).toHaveBeenCalledTimes(1);
        expect(mocks.compileSpy).toHaveBeenCalledTimes(2);
    });

    it("connectPrefigureSharedWorker does not call init", async () => {
        const mod = await import("../src/index");

        await mod.connectPrefigureSharedWorker();

        expect(mocks.initSpy).not.toHaveBeenCalled();
    });

    it("connectPrefigureSharedWorker returns a handle with disconnect", async () => {
        const mod = await import("../src/index");

        const handle = await mod.connectPrefigureSharedWorker();

        expect(typeof handle.disconnect).toBe("function");
    });

    it("multiple connectPrefigureSharedWorker calls share one connection", async () => {
        const mod = await import("../src/index");

        await mod.connectPrefigureSharedWorker();
        await mod.connectPrefigureSharedWorker();

        // Only one Comlink.wrap call should have been made.
        expect(mocks.wrapSpy).toHaveBeenCalledTimes(1);
    });

    it("initPrefigure after connectPrefigureSharedWorker reuses the existing connection", async () => {
        const mod = await import("../src/index");

        await mod.connectPrefigureSharedWorker();
        await mod.initPrefigure("https://cdn.example.com/shared-assets/");

        // Comlink.wrap should still have been called at most once.
        expect(mocks.wrapSpy).toHaveBeenCalledTimes(1);
        expect(mocks.initSpy).toHaveBeenCalledTimes(1);
    });

    it("URL mismatch guard still throws after init via shared worker path", async () => {
        const mod = await import("../src/index");

        await mod.connectPrefigureSharedWorker();
        await mod.initPrefigure("https://cdn.example.com/url-guard-a/");

        await expect(
            mod.initPrefigure("https://cdn.example.com/url-guard-b/"),
        ).rejects.toThrow(/already initialized with a different indexURL/i);
    });

    it("getPrefigureRuntimeStatus reflects connected and initialized state", async () => {
        const mod = await import("../src/index");

        const before = mod.getPrefigureRuntimeStatus();
        expect(before.connected).toBe(false);
        expect(before.initialized).toBe(false);

        await mod.connectPrefigureSharedWorker();

        const afterConnect = mod.getPrefigureRuntimeStatus();
        expect(afterConnect.connected).toBe(true);
        expect(afterConnect.initialized).toBe(false);

        await mod.initPrefigure("https://cdn.example.com/status-assets/");

        const afterInit = mod.getPrefigureRuntimeStatus();
        expect(afterInit.connected).toBe(true);
        expect(afterInit.initialized).toBe(true);
    });
});

// Tests for the dedicated-worker fallback path (no SharedWorker available).
// When SharedWorker is absent, the module should create a DedicatedWorker using
// the Vite-generated inline worker factory. The factory calls
// `new (globalThis.Worker || Worker)(blobURL, {type:'module'})` so that it does
// not throw ReferenceError in contexts (e.g. Chrome module SharedWorkers) where
// `Worker` is not a bare-identifier global.
//
// NOTE: The `new Worker(...)` call inside the Vite-generated factory cannot be
// fully unit-tested here because the factory is replaced by `MockWorker` via
// `vi.mock("../src/worker?worker&inline")`. What we can test is that
// initPrefigure / compilePrefigure complete correctly when SharedWorker is
// unavailable so the fallback code path is exercised end-to-end.
describe("@doenet/prefigure – dedicated worker fallback (no SharedWorker)", () => {
    beforeEach(() => {
        vi.resetModules();
        mocks.initSpy.mockClear();
        mocks.compileSpy.mockClear();
        mocks.wrapSpy.mockClear();
        (globalThis as any).__PREFIGURE_FORCE_SHAREDWORKER_FAIL__ = true;
    });

    it("initPrefigure succeeds without SharedWorker", async () => {
        const mod = await import("../src/index");
        await mod.initPrefigure("https://cdn.example.com/fallback-assets/");
        expect(mocks.initSpy).toHaveBeenCalledTimes(1);
    });

    it("compilePrefigure succeeds without SharedWorker", async () => {
        const mod = await import("../src/index");
        const result = await mod.compilePrefigure("<diagram />");
        expect(result.svg).toBe("<svg/>");
    });

    it("getPrefigureRuntimeStatus reports dedicated transport", async () => {
        const mod = await import("../src/index");
        // Before any connection, transport defaults to "dedicated".
        const before = mod.getPrefigureRuntimeStatus();
        expect(before.transport).toBe("dedicated");

        await mod.initPrefigure("https://cdn.example.com/transport-assets/");
        const after = mod.getPrefigureRuntimeStatus();
        expect(after.transport).toBe("dedicated");
        expect(after.initialized).toBe(true);
    });
});
