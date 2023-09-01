const path = require("path");

const buildNextEslintCommand = (filenames) =>
  `yarn next:lint --fix --file ${filenames
    .map((f) => path.relative(path.join("packages", "nextjs"), f))
    .join(" --file ")}`;

const checkTypesNextCommand = () => "yarn next:check-types";

const buildFoundryLintCommand = (filenames) =>
  `yarn foundry:lint ${filenames
    .map((f) => path.relative(path.join("packages", "foundry"), f))
    .join(" ")}`;

const buildPinnerLintCommand = () =>
  `cargo check --manifest-path packages/pinner/Cargo.toml`;

module.exports = {
  "packages/nextjs/**/*.{ts,tsx}": [
    buildNextEslintCommand,
    checkTypesNextCommand,
  ],
  "packages/foundry/**/*.sol": [buildFoundryLintCommand],
  "packages/pinner/**/*.rs": [buildPinnerLintCommand],
};
