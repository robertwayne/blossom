const esbuild = require("esbuild")
const autoprefixer = require("autoprefixer")
const postcss = require("@deanc/esbuild-plugin-postcss")
const cssnano = require("cssnano")

console.log("Minifying and bundling assets...")

esbuild
    .build({
        entryPoints: ["src/index.ts"],
        bundle: true,
        minify: true,
        outfile: "../dist/bundle.js",
        plugins: [
            postcss({
                plugins: [autoprefixer, cssnano],
            }),
        ],
    })
    .catch((e) => console.error(e.error))

console.log('Created "bundle.js" and "bundle.css" in "dist" directory.')
