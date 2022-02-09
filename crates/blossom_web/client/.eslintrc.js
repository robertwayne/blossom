module.exports = {
    env: {
        browser: true,
        node: true,
    },
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint'],
    overrides: [
        {
            files: ['*.ts'],
            extends: ['plugin:@typescript-eslint/recommended'],
        },
    ],
    rules: {},
}
