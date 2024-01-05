import { execSync, spawn } from "node:child_process";
import { describe, expect, it } from "vitest";

const port = 3000;

describe("Check port availability and killing processes", () => {
  it("should start a dummy server", async () => {
    const dummyServer = spawn(
      "node",
      ["./tests/dummy-server.cjs", port.toString()],
      {
        detached: true,
        stdio: "overlapped",
      },
    );
    await new Promise((resolve) => setTimeout(resolve, 400));
    expect(dummyServer.pid).not.toBeUndefined();
  });

  it("should show usage", () => {
    const command = "./src/cli.cjs -h";
    const stdout = execSync(command).toString();
    expect(stdout).toContain("Usage: port-claim <port> [--verbose]");
  });

  it("should return error if no port is passed", () => {
    const command = "./src/cli.cjs";
    const stdout = execSync(command).toString();
    expect(stdout).toContain("Error: Missing port argument");
  });

  it("should report an available port", () => {
    const command = "./src/cli.cjs 3156 -v";
    const stdout = execSync(command).toString();
    expect(stdout).toContain(`Port 3156 is available`);
  });

  it("should report a taken port, and kill the process", () => {
    const command = "./src/cli.cjs " + port.toString() + " -v";
    const stdout = execSync(command).toString();
    expect(stdout).toContain(`Port ${port} is taken`);
  });

  it("should confirm the process is killed by reporting the port as available again", () => {
    const command = "./src/cli.cjs " + port.toString() + " -v";
    const stdout = execSync(command).toString();
    expect(stdout).toContain(`Port ${port} is available`);
  });
});
