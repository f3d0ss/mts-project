const fs = require("fs");
const path = require("path");

const deployableContractsWithFactories = [];
const contractToFilter = ["Safe", "CompatibilityFallbackHandler"];
const NO_CONTRACT_FOUND = "NAC";

function getContractBytecode(contract) {
  const current_path_to_out = path.join(
    __dirname,
    "..",
    "out",
    `${contract}.sol`
  );
  let contractObject = JSON.parse(
    fs.readFileSync(`${current_path_to_out}/${contract}.json`)
  );
  let bytecode = contractObject.bytecode.object;
  return bytecode.substring(2);
}

function getContractsDeployedWithFactory(broadCastObject) {
  const deployableContractsWithFactoriesWithBytecode =
    deployableContractsWithFactories.map((deployableContractWithFactories) => {
      return {
        name: deployableContractWithFactories,
        bytecode: getContractBytecode(deployableContractWithFactories),
      };
    });
  let newTransactionsThatDeployedContract = broadCastObject.transactions.filter(
    (transaction) =>
      transaction.transactionType == "CALL" &&
      transaction.additionalContracts.length > 0
  );
  newTransactionsThatDeployedContract = newTransactionsThatDeployedContract.map(
    (transaction) => {
      let newContractDeployedWithFactory = transaction.additionalContracts[0];
      const deployedContractNameWithBytecode =
        deployableContractsWithFactoriesWithBytecode.find(
          (deployableContractWithFactories) =>
            newContractDeployedWithFactory.initCode.startsWith(
              deployableContractWithFactories.bytecode
            )
        );

      let contractName = NO_CONTRACT_FOUND;
      if (deployedContractNameWithBytecode) {
        contractName = deployedContractNameWithBytecode.name;
      }
      newContractDeployedWithFactory.contractName = contractName;
      newContractDeployedWithFactory.contractAddress =
        newContractDeployedWithFactory.address;
      return newContractDeployedWithFactory;
    }
  );
  return newTransactionsThatDeployedContract.filter(
    (contract) => contract.contractName != NO_CONTRACT_FOUND
  );
}

function getNewSafeContract(broadCastObject) {
  let safeProxyFactoryTransactions = broadCastObject.transactions.filter(
    (transaction) =>
      transaction.transactionType == "CALL" &&
      transaction.contractName == "SafeProxyFactory" &&
      transaction.additionalContracts.length > 0
  );
  if (safeProxyFactoryTransactions.length == 0) return null;
  let newSafe = safeProxyFactoryTransactions[0].additionalContracts[0];
  newSafe = { contractName: "Safe", contractAddress: newSafe.address };
  return newSafe;
}

function filterContracts(transactions) {
  return transactions.filter(
    (transaction) => !contractToFilter.includes(transaction.contractName)
  );
}

module.exports = {
  getNewSafeContract,
  filterContracts,
  getContractsDeployedWithFactory,
};
