const generateTsAbisExtension = require("./generateTsAbisExtension.js");
const fs = require("fs");
const path = require("path");

//@ts-expect-error  This script runs after `forge deploy` therefore its deterministic that it will present
// const deployments = require("../deployments.json");
const prettier = require("prettier");

function getDirectories(path) {
  return fs.readdirSync(path).filter(function (file) {
    return fs.statSync(path + "/" + file).isDirectory();
  });
}
function getFiles(path) {
  return fs.readdirSync(path).filter(function (file) {
    return fs.statSync(path + "/" + file).isFile();
  });
}
function getAbiOfContract(contractName) {
  const current_path_to_artifacts = path.join(
    __dirname,
    "..",
    `out/${contractName}.sol`
  );
  const artifactJson = JSON.parse(
    fs.readFileSync(`${current_path_to_artifacts}/${contractName}.json`)
  );

  return artifactJson.abi;
}

function getPreviousAllGeneratedContracts(filePath) {
  const fileContent = fs.readFileSync(filePath, "utf8");

  // Extract the content between the curly braces { ... }
  const matches = fileContent.match(/\{([\s\S]*)\}/);

  if (!matches || matches.length < 2) {
    throw new Error("Unable to find contracts object in the file.");
  }

  const contractsContent = `{${matches[1]}}`;

  // Parse the content as JSON to get the contracts object
  const contractsModule = new Function(`return ${contractsContent}`)();
  const allGeneratedContracts = contractsModule || {};

  return allGeneratedContracts;
}

function generateTsAbis(scriptName, clearAllContracts) {
  const current_path_to_broadcast = path.join(
    __dirname,
    "..",
    "broadcast",
    scriptName
  );
  const current_path_to_deployments = path.join(__dirname, "..", "deployments");

  const chains = getDirectories(current_path_to_broadcast);
  const Deploymentchains = getFiles(current_path_to_deployments);

  let deployments = {};

  Deploymentchains.forEach((chain) => {
    if (!chain.endsWith(".json")) return;
    chain = chain.slice(0, -5);
    let deploymentObject = JSON.parse(
      fs.readFileSync(`${current_path_to_deployments}/${chain}.json`)
    );
    deployments[chain] = deploymentObject;
  });

  const TARGET_DIR = "../nextjs/generated/";

  let allGeneratedContracts = {};
  if (!clearAllContracts && fs.existsSync(TARGET_DIR)) {
    allGeneratedContracts = getPreviousAllGeneratedContracts(
      `${TARGET_DIR}deployedContracts.ts`
    );
  } else {
    chains.forEach((chain) => {
      allGeneratedContracts[chain] = [];
      allGeneratedContracts[chain].push({
        name: deployments[chain].networkName || "hardhat",
        chainId: chain,
        contracts: {},
      });
    });
  }

  chains.forEach((chain) => {
    let broadCastObject = JSON.parse(
      fs.readFileSync(`${current_path_to_broadcast}/${chain}/run-latest.json`)
    );
    let transactionsCreate = broadCastObject.transactions.filter(
      (transaction) => transaction.transactionType == "CREATE"
    );

    /* ========================================================================== */
    /*                  ADD CONTRACT CREATE BY FACTORY CONTRACTS                  */
    /* ========================================================================== */
    transactionsCreate =
      generateTsAbisExtension.filterContracts(transactionsCreate);

    let newSafeContract =
      generateTsAbisExtension.getNewSafeContract(broadCastObject);
    if (newSafeContract) {
      transactionsCreate.push(newSafeContract);
    }
    const contractsDeployedWithFactory =
      generateTsAbisExtension.getContractsDeployedWithFactory(broadCastObject);
    transactionsCreate = [
      ...transactionsCreate,
      ...contractsDeployedWithFactory,
    ];

    /* ========================================================================== */
    /*                END ADD CONTRACT CREATE BY FACTORY CONTRACTS                */
    /* ========================================================================== */
    transactionsCreate.forEach((transaction) => {
      allGeneratedContracts[chain][0]["contracts"][
        deployments[chain][transaction.contractAddress] ||
          transaction.contractName
      ] = {
        address: transaction.contractAddress,
        abi: getAbiOfContract(transaction.contractName),
      };
    });
  });

  const fileContent = Object.entries(allGeneratedContracts).reduce(
    (content, [chainId, chainConfig]) => {
      return `${content}${parseInt(chainId).toFixed(0)}:${JSON.stringify(
        chainConfig,
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
    `${TARGET_DIR}deployedContracts.ts`,
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
  generateTsAbis,
};
