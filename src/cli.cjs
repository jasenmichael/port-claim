#!/usr/bin/env node

const portClaim = require("../dist/index.cjs");

function showUsage() {
  console.log(`Usage: port-claim <port> [--verbose]`);
  console.log(`Options:`);
  console.log(`  -h, --help     Show this help message`);
  console.log(`  -v, --verbose  Print verbose output`);
}

const port = process.argv[2];
const verbose =
  process.argv.includes("--verbose") || process.argv.includes("-v");

if (!port) {
  console.log("Error: Missing port argument");
  showUsage();
  process.exit(0);
}

if (process.argv.includes("-h") || process.argv.includes("--help")) {
  showUsage();
  process.exit(0);
}

try {
  portClaim(port, verbose);
} catch (error) {
  console.error(error);
  process.exit(1);
}
