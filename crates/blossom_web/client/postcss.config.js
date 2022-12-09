const autoprefixer = require("autoprefixer")

const mode = process.env.NODE_ENV
const dev = mode === "development"

const config = {
    plugins: [autoprefixer],
}

module.exports = config
