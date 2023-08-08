const fs = require("fs");
const path = require("path");

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

console.log(getContractBytecode("ResturantToken"));
