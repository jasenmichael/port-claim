<h1 align="center">port-claim</h1>
<div align="center">
  <strong>If a port is in use, port-claim stops the process using it.</strong>
</div>
<br>
<div align="center">
  <a href="https://npmjs.org/package/port-claim">
    <img src="https://img.shields.io/npm/v/port-claim.svg?style=flat-square" alt="Package version" />
  </a>
  <a href="https://npmjs.org/package/port-claim">
    <img src="https://img.shields.io/npm/dm/port-claim.svg?style=flat-square" alt="Downloads" />
  </a>
  <a href="https://github.com/feross/standard">
    <img src="https://img.shields.io/badge/code%20style-standard-brightgreen.svg?style=flat-square" alt="Standard" />
  </a>
  <a href="https://github.com/jasenmichael/port-claim/blob/main/LICENSE">
    <img src="https://img.shields.io/npm/l/port-claim.svg?style=flat-square" alt="License" />
  </a>
  <a href="http://makeapullrequest.com">
    <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" alt="PRs" />
  </a>
</div>
<br>


## Table of Contents
- [Table of Contents](#table-of-contents)
- [Install](#install)
- [Usage](#usage)
- [API](#api)
- [CLI](#cli)
- [Contributing](#contributing)
- [License](#license)

## Install


With `npm`:
```sh
npm install --save port-claim
```

With `yarn`:
```sh
yarn add port-claim
```

With `pnpm`:
```sh
pnpm add port-claim
```

## Usage

```js


```

## API

**portClaim(portNumber: number): Promise<void>**

- Takes a port number as input.
- Checks if the port is in use (with [detect-port](https://www.npmjs.com/package/detect-port)).
- If the port is already in use, kills the process using it (with [kill-port](https://www.npmjs.com/package/kill-port)).
- Returns a promise that resolves when the port is successfully claimed, or rejects with an error if no port was passed, or the process fails.


## CLI

You can use `port-claim` as a global package.

Install the package globally:

```sh
$ npm install -g port-claim
# OR with yarn
$ yarn global add port-claim
# OR with pnpm
$ pnpm add -g port-claim
```

Then:

```sh
# Claim a port
$ port-claim 3000
```

You can also use [npx](https://nodejs.dev/learn/the-npx-nodejs-package-runner) to `port-claim` without installing:

```sh
# Claim a port
$ npx port-claim 3000
```

## Example
example usage in a package.json pre script
```json file="package.json"
  "scripts": {
    "predev": "port-claim 3000",
    "dev": "nuxt dev"
  },
```

## Contributing

Got an idea for a new feature? Found a bug? Contributions are welcome! Please [open up an issue](https://github.com/jasenmichael/port-claim/issues) or [make a pull request](https://github.com/jasenmichael/claim-port/pulls).

## License

[MIT © @jasenmichael](./LICENSE)
