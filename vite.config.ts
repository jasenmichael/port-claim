import { resolve } from "node:path"
import { defineConfig } from "vite"
import { glob } from "glob"

export default defineConfig({
  build: {
    lib: {
      entry: glob.sync(resolve(__dirname, 'src/*.ts')),
      formats: ["es", "cjs"],
      fileName: (format, name) => `${name}.${format === 'es' ? 'mjs' : 'cjs'}`,
    },
  }
});
