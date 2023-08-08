const { generateTsAbis } = require("./generateTsAbis.js");

try {
  scriptName = "SetUpTestScenario.s.sol";
  if (process.argv[2]) {
    scriptName = process.argv[2];
  }
  generateTsAbis(scriptName, true);
} catch (error) {
  console.error(error);
  process.exitCode = 1;
}
