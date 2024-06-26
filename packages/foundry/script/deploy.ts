import * as readline from "readline";
import * as child_process from "child_process";
import * as dotenv from "dotenv";
import { generateTsAbis } from "./utils/generateTsAbis.js";

dotenv.config(); // Load environment variables from .env file

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function executeCommand(command: string) {
  const result = child_process.spawnSync(command, { shell: true });
  if (result.error) {
    console.error(`Error executing command: ${command}`);
    console.error(result.stderr?.toString());
  } else {
    console.log(result.stdout?.toString());
  }
}

async function askQuestion(question: string): Promise<string> {
  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      resolve(answer.trim());
    });
  });
}

async function deployTestScenario(network: string, verify: boolean) {
  const command = `forge script script/SetUpTestScenario.s.sol --broadcast --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  console.log(command);
  executeCommand(command);
}

async function deploySafe(network: string, verify: boolean) {
  const addressesStr = await askQuestion(
    "Please enter an array of addresses separated by commas (e.g., address1,address2): "
  );
  const thresholdStr = await askQuestion("Please enter the threshold: ");

  const addresses = addressesStr.split(",").map((addr) => addr.trim());
  const threshold = parseInt(thresholdStr.trim(), 10);
  const command = `forge script script/DeploySafe.s.sol --broadcast --sig "run(address[],uint256)" "[${addresses.join(
    ","
  )}]" ${threshold} --rpc-url ${network} ${verify ? "--verify" : ""}`;
  executeCommand(command);
}

async function deployMTSController(network: string, verify: boolean) {
  const ownerAddress = await askQuestion("Please enter the OWNER_ADDRESS: ");
  const command = `forge script script/DeployMTSController.s.sol --broadcast --sig "run(address)" ${ownerAddress} --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deployMTSControllerWithRestaurant(
  network: string,
  verify: boolean
) {
  if (!process.env.MTS_OWNER) {
    console.error(
      'ERROR: You need to set the private key of the owner in the enviroment variable "MTS_OWNER".'
    );
    rl.close();
    return;
  }

  const restaurantOwner = await askQuestion(
    "Please enter the RESTAURANT_OWNER: "
  );
  const tokenName = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_NAME: "
  );
  const tokenSymbol = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_SYMBOL: "
  );

  const command = `forge script script/DeployMinimalScenarioWithPkOwner.s.sol --broadcast --sig "run(address,string,string)" ${restaurantOwner} "${tokenName}" "${tokenSymbol}" --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deploySafeMTSControllerResturant(
  network: string,
  verify: boolean
) {
  if (!process.env.SAFE_OWNERS_PKS) {
    console.error(
      'ERROR: You need to set the private keys of the Safe owners in the enviroment variable "SAFE_OWNERS_PKS".'
    );
    rl.close();
    return;
  }

  const thresholdStr = await askQuestion("Please enter the THRESHOLD: ");
  const restaurantOwner = await askQuestion(
    "Please enter the RESTAURANT_OWNER: "
  );
  const tokenName = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_NAME: "
  );
  const tokenSymbol = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_SYMBOL: "
  );

  const threshold = parseInt(thresholdStr.trim(), 10);
  const command = `forge script script/DeployMinimalScenarioWithSafeOwner.s.sol --broadcast --sig "run(uint256,address,string,string)" ${threshold} ${restaurantOwner} "${tokenName}" "${tokenSymbol}" --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deployAResturantWithEOA(network: string, verify: boolean) {
  if (!process.env.MTS_OWNER) {
    console.error(
      'ERROR: You need to set the private key of the owner in the enviroment variable "MTS_OWNER".'
    );
    rl.close();
    return;
  }

  const controllerAddress = await askQuestion(
    "Please enter the CONTROLLER_ADDRESS: "
  );
  const restaurantOwner = await askQuestion(
    "Please enter the RESTAURANT_OWNER: "
  );
  const tokenName = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_NAME: "
  );
  const tokenSymbol = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_SYMBOL: "
  );

  const command = `forge script script/DeployResturantWithPkOwner.s.sol --broadcast --sig "run(address,address,string,string)" ${controllerAddress} ${restaurantOwner} "${tokenName}" "${tokenSymbol}" --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deployAResturantWithSafe(network: string, verify: boolean) {
  if (!process.env.SAFE_OWNERS_PKS) {
    console.error(
      'ERROR: You need to set the private keys of the Safe owners in the enviroment variable "SAFE_OWNERS_PKS".'
    );
    rl.close();
    return;
  }

  const controllerAddress = await askQuestion(
    "Please enter the CONTROLLER_ADDRESS: "
  );
  const safeAddress = await askQuestion("Please enter the SAFE_ADDRESS: ");
  const restaurantOwner = await askQuestion(
    "Please enter the RESTAURANT_OWNER: "
  );
  const tokenName = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_NAME: "
  );
  const tokenSymbol = await askQuestion(
    "Please enter the RESTAURANT_TOKEN_SYMBOL: "
  );

  const command = `forge script script/DeployResturantWithSafeOwner.s.sol --broadcast --sig "run(address,string,string,address,address)" ${restaurantOwner} "${tokenName}" "${tokenSymbol}" ${controllerAddress} ${safeAddress} --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deployAMockERC20(network: string, verify: boolean) {
  const command = `forge script script/DeployMockErc20.s.sol --broadcast --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deployDAO(network: string, verify: boolean) {
  const controllerAddress = await askQuestion(
    "Please enter the CONTROLLER_ADDRESS: "
  );

  const command = `forge script script/dao/DeployDAO.s.sol --broadcast --sig "run(address)" ${controllerAddress} --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function deployDAOFreeMint(network: string, verify: boolean) {
  const controllerAddress = await askQuestion(
    "Please enter the CONTROLLER_ADDRESS: "
  );

  const command = `forge script script/dao/DeployDAOFreeMint.s.sol --broadcast --sig "run(address)" ${controllerAddress} --rpc-url ${network} ${
    verify ? "--verify" : ""
  }`;
  executeCommand(command);
}

async function main() {
  const networkChoice = await askQuestion(
    "Enter the network you want to deploy on (default: localhost): "
  );
  const network = networkChoice.trim() || "localhost";

  console.log("What do you want to deploy?");
  console.log("0) TestScenario");
  console.log("1) Only a Safe");
  console.log("2) Only MTSController");
  console.log("3) MTSController with one restaurant");
  console.log("4) A Safe, MTSController, and one restaurant");
  console.log("5) A Restaurant with the MTSController owned by an EOA");
  console.log("6) A Restaurant with the MTSController owned by a Safe");
  console.log("7) A Mock ERC20 token");
  console.log("8) MTS DAO");
  console.log("9) MTS DAO with mintable power");

  const scenarioChoice = await askQuestion(
    "Enter your choice (0/1/2/3/4/5/6/7) (default: 0): "
  );

  const scenario = scenarioChoice.trim() || "0";

  let verify = false;
  if (network !== "localhost") {
    const verifyChoice = await askQuestion(
      "Do you want to verify the contracts? (y/N): "
    );
    verify = /^y(es)?$/i.test(verifyChoice.trim());
  }

  switch (scenario) {
    case "0":
      await deployTestScenario(network, verify);
      generateTsAbis("SetUpTestScenario.s.sol", true);
      break;
    case "1":
      await deploySafe(network, verify);
      generateTsAbis("DeploySafe.s.sol", false);
      break;
    case "2":
      await deployMTSController(network, verify);
      generateTsAbis("DeployMTSController.s.sol", true);
      break;
    case "3":
      await deployMTSControllerWithRestaurant(network, verify);
      generateTsAbis("DeployMinimalScenarioWithPkOwner.s.sol", true);
      break;
    case "4":
      await deploySafeMTSControllerResturant(network, verify);
      generateTsAbis("DeployMinimalScenarioWithSafeOwner.s.sol", true);
      break;
    case "5":
      await deployAResturantWithEOA(network, verify);
      generateTsAbis("DeployResturantWithPkOwner.s.sol", false);
      break;
    case "6":
      await deployAResturantWithSafe(network, verify);
      generateTsAbis("DeployResturantWithSafeOwner.s.sol", false);
      break;
    case "7":
      await deployAMockERC20(network, verify);
      generateTsAbis("DeployMockErc20.s.sol", false);
      break;
    case "8":
      await deployDAO(network, verify);
      generateTsAbis("DeployDAO.s.sol", false);
      break;
    case "9":
      await deployDAOFreeMint(network, verify);
      generateTsAbis("DeployDAOFreeMint.s.sol", false);
      break;
    default:
      console.log("Invalid choice. Exiting...");
      break;
  }
  rl.close();
}

main();
