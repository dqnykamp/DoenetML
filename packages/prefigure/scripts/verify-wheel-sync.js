#!/usr/bin/env node

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export function verifyWheelSync({ compilerPath, pyodidePackagesDir }) {
    if (!fs.existsSync(compilerPath)) {
        throw new Error(`Missing compiler file: ${compilerPath}`);
    }

    if (!fs.existsSync(pyodidePackagesDir)) {
        throw new Error(
            `Missing pyodide packages directory: ${pyodidePackagesDir}`,
        );
    }

    const compilerContent = fs.readFileSync(compilerPath, "utf8");
    const constantMatch = compilerContent.match(
        /export const PREFIG_WHEEL_FILENAME = "([^"]+)";/,
    );

    let configuredWheel = constantMatch?.[1];

    if (!configuredWheel) {
        const wheelFilenamePath = path.join(
            path.dirname(compilerPath),
            "wheel-filename.ts",
        );
        if (!fs.existsSync(wheelFilenamePath)) {
            throw new Error(
                "Could not parse PREFIG_WHEEL_FILENAME from compiler.ts and wheel-filename.ts was not found",
            );
        }

        const wheelContent = fs.readFileSync(wheelFilenamePath, "utf8");
        const wheelMatch = wheelContent.match(
            /export const PREFIG_WHEEL_FILENAME = "([^"]+)";/,
        );

        if (!wheelMatch) {
            throw new Error(
                "Could not parse PREFIG_WHEEL_FILENAME from wheel-filename.ts",
            );
        }

        configuredWheel = wheelMatch[1];
    }
    if (
        !(
            configuredWheel.startsWith("prefig-") &&
            configuredWheel.endsWith(".whl")
        )
    ) {
        throw new Error(
            `PREFIG_WHEEL_FILENAME is not a prefig wheel filename: ${configuredWheel}`,
        );
    }

    const prefigWheels = fs
        .readdirSync(pyodidePackagesDir)
        .filter((f) => f.startsWith("prefig-") && f.endsWith(".whl"));

    if (prefigWheels.length === 0) {
        throw new Error("No prefig wheel found in pyodide_packages/");
    }

    if (prefigWheels.length > 1) {
        throw new Error(
            `Multiple prefig wheels found in pyodide_packages/: ${prefigWheels.join(", ")}`,
        );
    }

    const packagedWheel = prefigWheels[0];
    if (configuredWheel !== packagedWheel) {
        throw new Error(
            `Wheel mismatch: compiler.ts uses ${configuredWheel}, but pyodide_packages contains ${packagedWheel}`,
        );
    }

    return configuredWheel;
}

function runCli() {
    const compilerPath = path.join(
        __dirname,
        "..",
        "src",
        "worker",
        "compiler.ts",
    );
    const pyodidePackagesDir = path.join(__dirname, "..", "pyodide_packages");

    try {
        const configuredWheel = verifyWheelSync({
            compilerPath,
            pyodidePackagesDir,
        });
        console.log(`✅ Prefigure wheel sync OK: ${configuredWheel}`);
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        console.error(`❌ ${message}`);
        process.exit(1);
    }
}

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
    runCli();
}
