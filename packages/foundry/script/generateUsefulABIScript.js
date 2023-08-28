const { generateUsefulAbis } = require("./utils/generateUsefulAbis.js");

try {
  generateUsefulAbis();
} catch (error) {
  console.error(error);
  process.exitCode = 1;
}
