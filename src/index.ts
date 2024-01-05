import { PortNumber, ConsoleMessage } from "./types";

const detectPort = require("detect-port") as (
  port: PortNumber,
) => Promise<number>;
const killPort = require("kill-port") as (port: PortNumber) => Promise<void>;

let verbose = false;

export default async function portClaim(
  port: PortNumber,
  _verbose = false,
): Promise<void> {
  if (!port) {
    throw new Error("Port is required");
  }
  verbose = _verbose;

  logger(`Checking port ${port} is available...`);

  const isPortTaken = (await detectPort(port)).toString() !== port.toString();
  if (isPortTaken) {
    logger(`Port ${port} is taken`);
    logger(`Killing port ${port}...`);
    await killPort(port);
  } else {
    logger(`Port ${port} is available`);
  }
}

function logger(message: ConsoleMessage) {
  if (verbose) {
    console.log(message);
  }
}
