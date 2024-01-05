const detectPort = require('detect-port') as (port: number) => Promise<number>
const killPort = require('kill-port') as (port: number) => Promise<void>

let verbose = false

export default async function portClaim(port: number | string, _verbose = false) {
  if (!port) { throw new Error('Port is required') }

  verbose = _verbose

  logger(`Checking port ${port} is available...`)

  const isPortTaken = ((await detectPort(port as number))).toString() !== port.toString()
  if (isPortTaken) {
    logger(`Port ${port} is taken`)
    logger(`Killing port ${port}...`)
    await killPort(Number(port))
  } else {
    logger(`Port ${port} is available`)
  }
}

function logger(message: string) {
  if (verbose) { console.log(message) }
}
