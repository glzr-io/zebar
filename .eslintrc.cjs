// https://github.com/vercel/style-guide
module.exports = {
  root: true,
  extends: [
    '@vercel/style-guide/eslint/node',
    '@vercel/style-guide/eslint/typescript',
    '@vercel/style-guide/eslint/browser',
  ].map(require.resolve),
  settings: {
    'import/resolver': {
      typescript: {
        project: ['packages/**/tsconfig.json'],
        extensions: ['.js', '.jsx', '.ts', '.tsx'],
      },
    },
  },
  ignorePatterns: ['node_modules/', 'dist/'],
  rules: {
    'import/no-default-export': 'off',
  },
};
