#!/usr/bin/env node

import { Binary } from "binary-install";
import { readFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const { name, version, repository } = JSON.parse(readFileSync(path.join(__dirname, "package.json"), "utf8"));

const repoUrl = (repository.url || repository)
  .replace("git+", "")
  .replace(".git", "")

function getBinUrl() {
  const platform = getPlatform();
  const arch = getArch();

  return `${repoUrl}/releases/download/v${version}/port-claim-${platform}-${arch}.tar.gz`;
}

function getPlatform() {
  const platform = os.platform();
  switch (platform) {
    case 'win32':
      return 'windows';
    case 'darwin':
      return 'mac';
    case 'linux':
      return 'linux';
    default:
      throw new Error(`Unsupported platform: ${platform}`);
  }
}

function getArch() {
  const arch = os.arch();
  switch (arch) {
    case 'x64':
      return 'x64';
    case 'arm64':
      return 'arm64';
    case 'ia32':
      return 'x86';
    case 'x32':
      return 'x86';
    case 'x86':
      return 'x86';
    default:
      throw new Error(`Unsupported architecture: ${arch}`);
  }
}

function getBinary() {
  const url = getBinUrl();
  const installDirectory = path.join(__dirname, ".bin");
  return new Binary(name, url, { installDirectory });
}

try {
  if (process.argv.includes("uninstall")) {
    try {
      const binary = getBinary();
      binary.uninstall();
      console.log("Previous version uninstalled");
    } catch (err) {
      console.log("No previous version to uninstall");
    }
  } else if (process.argv.includes("install")) {
    const binary = getBinary();
    binary.install();
  } else {
    const binary = getBinary();
    binary.run();
  }
} catch (error) {
  console.error("Error:", error.message);
  process.exit(1);
}
