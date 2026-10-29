// Compila coverwin-cli e copia para src-tauri/binaries com o sufixo de target que o Tauri exige em `externalBin`.
import { execSync } from "node:child_process";
import { copyFileSync, mkdirSync } from "node:fs";

const root = new URL("../../", import.meta.url);
const triple = execSync("rustc --print host-tuple").toString().trim();

execSync("cargo build --release -p coverwin-cli", { cwd: root, stdio: "inherit" });
mkdirSync(new URL("app/src-tauri/binaries/", root), { recursive: true });
copyFileSync(
	new URL("target/release/coverwin-cli.exe", root),
	new URL(`app/src-tauri/binaries/coverwin-cli-${triple}.exe`, root),
);
