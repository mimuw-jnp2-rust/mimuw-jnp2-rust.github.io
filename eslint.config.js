import globals from "globals";

export default [
  {
    extends: ["eslint:recommended", "prettier"],
    globals: {
      ...globals.browser,
      ...globals.es2021,
    },
    ignorePatterns: ["public/", "target/"],
    parser: "@typescript-eslint/parser",
    parserOptions: {
      ecmaVersion: "latest",
    },

    files: ["**/*.js"],
    plugins: ["@typescript-eslint"],
    rules: {},
  },
];
