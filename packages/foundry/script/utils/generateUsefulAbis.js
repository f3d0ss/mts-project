const fs = require("fs");
const path = require("path");

//@ts-expect-error  This script runs after `forge deploy` therefore its deterministic that it will present
// const deployments = require("../deployments.json");
const prettier = require("prettier");

function getAbiOfContract(contractName) {
  const current_path_to_artifacts = path.join(
    __dirname,
    "..",
    "..",
    `out/${contractName}.sol`
  );
  const artifactJson = JSON.parse(
    fs.readFileSync(`${current_path_to_artifacts}/${contractName}.json`)
  );

  return artifactJson.abi;
}

function generateUsefulAbis() {
  const TARGET_DIR = "../nextjs/generated/";
  contractNames = ["ResturantToken", "Safe", "ERC20"];

  let allGeneratedContracts = {};

  contractNames.forEach((contractName) => {
    allGeneratedContracts[`${contractName}`] = {
      abi: getAbiOfContract(contractName),
    };
  });

  const fileContent = Object.entries(allGeneratedContracts).reduce(
    (content, [contractName, contractABI]) => {
      return `${content}${contractName}:${JSON.stringify(
        contractABI,
        null,
        2
      )},`;
    },
    ""
  );

  if (!fs.existsSync(TARGET_DIR)) {
    fs.mkdirSync(TARGET_DIR);
  }

  fs.writeFileSync(
    `${TARGET_DIR}usefulAbis.ts`,
    prettier.format(
      `const contracts = {${fileContent}} as const; \n\n export default contracts`,
      {
        parser: "typescript",
      }
    )
  );
}

/* ========================================================================== */
/*                EXPORT generateTsAbis TO BE CALLABLE BY OTHER JS SCRIPT               */
/* ========================================================================== */
module.exports = {
  generateUsefulAbis,
};
