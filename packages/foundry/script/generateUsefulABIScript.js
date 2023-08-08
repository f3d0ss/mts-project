const { generateUsefulAbis } = require("./generateUsefulAbis.js");

try {
  generateUsefulAbis();
} catch (error) {
  console.error(error);
  process.exitCode = 1;
}
