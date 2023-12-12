// https://github.com/vercel/style-guide
module.exports = {
  root: true,
  extends: [
    '@vercel/style-guide/eslint/node',
    '@vercel/style-guide/eslint/typescript',
    '@vercel/style-guide/eslint/browser',
    // 'prettier',
  ].map(require.resolve),
  parserOptions: {
    project: `${__dirname}/tsconfig.json`,
  },
  globals: {
    JSX: true,
  },
  settings: {
    'import/resolver': {
      typescript: {
        project: `${__dirname}/tsconfig.json`,
      },
    },
  },
  ignorePatterns: ['node_modules/', 'dist/'],
  rules: {
    'import/no-default-export': 'off',
  },
};
