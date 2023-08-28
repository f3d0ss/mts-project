"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var readline = require("readline");
var child_process = require("child_process");
var dotenv = require("dotenv");
var generateTsAbis_js_1 = require("./utils/generateTsAbis.js");
dotenv.config(); // Load environment variables from .env file
var rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});
function executeCommand(command) {
    var _a, _b;
    var result = child_process.spawnSync(command, { shell: true });
    if (result.error) {
        console.error("Error executing command: ".concat(command));
        console.error((_a = result.stderr) === null || _a === void 0 ? void 0 : _a.toString());
    }
    else {
        console.log((_b = result.stdout) === null || _b === void 0 ? void 0 : _b.toString());
    }
}
function askQuestion(question) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            return [2 /*return*/, new Promise(function (resolve) {
                    rl.question(question, function (answer) {
                        resolve(answer.trim());
                    });
                })];
        });
    });
}
function deployTestScenario(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var command;
        return __generator(this, function (_a) {
            command = "forge script script/SetUpTestScenario.s.sol --broadcast --rpc-url ".concat(network, " ").concat(verify ? "--verify" : "");
            console.log(command);
            executeCommand(command);
            return [2 /*return*/];
        });
    });
}
function deploySafe(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var addressesStr, thresholdStr, addresses, threshold, command;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, askQuestion("Please enter an array of addresses separated by commas (e.g., address1,address2): ")];
                case 1:
                    addressesStr = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the threshold: ")];
                case 2:
                    thresholdStr = _a.sent();
                    addresses = addressesStr.split(",").map(function (addr) { return addr.trim(); });
                    threshold = parseInt(thresholdStr.trim(), 10);
                    command = "forge script script/DeploySafe.s.sol --broadcast --sig \"run(address[],uint256)\" \"[".concat(addresses.join(","), "]\" ").concat(threshold, " --rpc-url ").concat(network, " ").concat(verify ? "--verify" : "");
                    executeCommand(command);
                    return [2 /*return*/];
            }
        });
    });
}
function deployMTSController(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var ownerAddress, command;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, askQuestion("Please enter the OWNER_ADDRESS: ")];
                case 1:
                    ownerAddress = _a.sent();
                    command = "forge script script/DeployMTSController.s.sol --broadcast --sig \"run(address)\" ".concat(ownerAddress, " --rpc-url ").concat(network, " ").concat(verify ? "--verify" : "");
                    executeCommand(command);
                    return [2 /*return*/];
            }
        });
    });
}
function deployMTSControllerWithRestaurant(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var restaurantOwner, tokenName, tokenSymbol, command;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!process.env.MTS_OWNER) {
                        console.error('ERROR: You need to set the private key of the owner in the enviroment variable "MTS_OWNER".');
                        rl.close();
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_OWNER: ")];
                case 1:
                    restaurantOwner = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_NAME: ")];
                case 2:
                    tokenName = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_SYMBOL: ")];
                case 3:
                    tokenSymbol = _a.sent();
                    command = "forge script script/DeployMinimalScenarioWithPkOwner.s.sol --broadcast --sig \"run(address,string,string)\" ".concat(restaurantOwner, " \"").concat(tokenName, "\" \"").concat(tokenSymbol, "\" --rpc-url ").concat(network, " ").concat(verify ? "--verify" : "");
                    executeCommand(command);
                    return [2 /*return*/];
            }
        });
    });
}
function deploySafeMTSControllerResturant(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var thresholdStr, restaurantOwner, tokenName, tokenSymbol, threshold, command;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!process.env.SAFE_OWNERS_PKS) {
                        console.error('ERROR: You need to set the private keys of the Safe owners in the enviroment variable "SAFE_OWNERS_PKS".');
                        rl.close();
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, askQuestion("Please enter the THRESHOLD: ")];
                case 1:
                    thresholdStr = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_OWNER: ")];
                case 2:
                    restaurantOwner = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_NAME: ")];
                case 3:
                    tokenName = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_SYMBOL: ")];
                case 4:
                    tokenSymbol = _a.sent();
                    threshold = parseInt(thresholdStr.trim(), 10);
                    command = "forge script script/DeployMinimalScenarioWithSafeOwner.s.sol --broadcast --sig \"run(uint256,address,string,string)\" ".concat(threshold, " ").concat(restaurantOwner, " \"").concat(tokenName, "\" \"").concat(tokenSymbol, "\" --rpc-url ").concat(network, " ").concat(verify ? "--verify" : "");
                    executeCommand(command);
                    return [2 /*return*/];
            }
        });
    });
}
function deployAResturantWithEOA(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var controllerAddress, restaurantOwner, tokenName, tokenSymbol, command;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!process.env.MTS_OWNER) {
                        console.error('ERROR: You need to set the private key of the owner in the enviroment variable "MTS_OWNER".');
                        rl.close();
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, askQuestion("Please enter the CONTROLLER_ADDRESS: ")];
                case 1:
                    controllerAddress = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_OWNER: ")];
                case 2:
                    restaurantOwner = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_NAME: ")];
                case 3:
                    tokenName = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_SYMBOL: ")];
                case 4:
                    tokenSymbol = _a.sent();
                    command = "forge script script/DeployResturantWithPkOwner.s.sol --broadcast --sig \"run(address,address,string,string)\" ".concat(controllerAddress, " ").concat(restaurantOwner, " \"").concat(tokenName, "\" \"").concat(tokenSymbol, "\" --rpc-url ").concat(network, " ").concat(verify ? "--verify" : "");
                    executeCommand(command);
                    return [2 /*return*/];
            }
        });
    });
}
function deployAResturantWithSafe(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var controllerAddress, safeAddress, restaurantOwner, tokenName, tokenSymbol, command;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!process.env.SAFE_OWNERS_PKS) {
                        console.error('ERROR: You need to set the private keys of the Safe owners in the enviroment variable "SAFE_OWNERS_PKS".');
                        rl.close();
                        return [2 /*return*/];
                    }
                    return [4 /*yield*/, askQuestion("Please enter the CONTROLLER_ADDRESS: ")];
                case 1:
                    controllerAddress = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the SAFE_ADDRESS: ")];
                case 2:
                    safeAddress = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_OWNER: ")];
                case 3:
                    restaurantOwner = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_NAME: ")];
                case 4:
                    tokenName = _a.sent();
                    return [4 /*yield*/, askQuestion("Please enter the RESTAURANT_TOKEN_SYMBOL: ")];
                case 5:
                    tokenSymbol = _a.sent();
                    command = "forge script script/DeployResturantWithSafeOwner.s.sol --broadcast --sig \"run(address,string,string,address,address)\" ".concat(restaurantOwner, " \"").concat(tokenName, "\" \"").concat(tokenSymbol, "\" ").concat(controllerAddress, " ").concat(safeAddress, " --rpc-url ").concat(network, " ").concat(verify ? "--verify" : "");
                    executeCommand(command);
                    return [2 /*return*/];
            }
        });
    });
}
function deployAMockERC20(network, verify) {
    return __awaiter(this, void 0, void 0, function () {
        var command;
        return __generator(this, function (_a) {
            command = "forge script script/DeployMockErc20.s.sol --broadcast --rpc-url ".concat(network, " ").concat(verify ? "--verify" : "");
            executeCommand(command);
            return [2 /*return*/];
        });
    });
}
function main() {
    return __awaiter(this, void 0, void 0, function () {
        var networkChoice, network, scenarioChoice, scenario, verifyChoice, verify, _a;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0: return [4 /*yield*/, askQuestion("Enter the network you want to deploy on (default: localhost): ")];
                case 1:
                    networkChoice = _b.sent();
                    network = networkChoice.trim() || "localhost";
                    console.log("What do you want to deploy?");
                    console.log("0) TestScenario");
                    console.log("1) Only a Safe");
                    console.log("2) Only MTSController");
                    console.log("3) MTSController with one restaurant");
                    console.log("4) A Safe, MTSController, and one restaurant");
                    console.log("5) A Restaurant with the MTSController owned by an EOA");
                    console.log("6) A Restaurant with the MTSController owned by a Safe");
                    console.log("7) A Mock ERC20 token");
                    return [4 /*yield*/, askQuestion("Enter your choice (0/1/2/3/4/5/6/7) (default: 0): ")];
                case 2:
                    scenarioChoice = _b.sent();
                    scenario = scenarioChoice.trim() || "0";
                    return [4 /*yield*/, askQuestion("Do you want to verify the contracts? (y/N): ")];
                case 3:
                    verifyChoice = _b.sent();
                    verify = /^y(es)?$/i.test(verifyChoice.trim());
                    _a = scenario;
                    switch (_a) {
                        case "0": return [3 /*break*/, 4];
                        case "1": return [3 /*break*/, 6];
                        case "2": return [3 /*break*/, 8];
                        case "3": return [3 /*break*/, 10];
                        case "4": return [3 /*break*/, 12];
                        case "5": return [3 /*break*/, 14];
                        case "6": return [3 /*break*/, 16];
                        case "7": return [3 /*break*/, 18];
                    }
                    return [3 /*break*/, 20];
                case 4: return [4 /*yield*/, deployTestScenario(network, verify)];
                case 5:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("SetUpTestScenario.s.sol", true);
                    return [3 /*break*/, 21];
                case 6: return [4 /*yield*/, deploySafe(network, verify)];
                case 7:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeploySafe.s.sol", false);
                    return [3 /*break*/, 21];
                case 8: return [4 /*yield*/, deployMTSController(network, verify)];
                case 9:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeployMTSController.s.sol", true);
                    return [3 /*break*/, 21];
                case 10: return [4 /*yield*/, deployMTSControllerWithRestaurant(network, verify)];
                case 11:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeployMinimalScenarioWithPkOwner.s.sol", true);
                    return [3 /*break*/, 21];
                case 12: return [4 /*yield*/, deploySafeMTSControllerResturant(network, verify)];
                case 13:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeployMinimalScenarioWithSafeOwner.s.sol", true);
                    return [3 /*break*/, 21];
                case 14: return [4 /*yield*/, deployAResturantWithEOA(network, verify)];
                case 15:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeployResturantWithPkOwner.s.sol", false);
                    return [3 /*break*/, 21];
                case 16: return [4 /*yield*/, deployAResturantWithSafe(network, verify)];
                case 17:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeployResturantWithSafeOwner.s.sol", false);
                    return [3 /*break*/, 21];
                case 18: return [4 /*yield*/, deployAMockERC20(network, verify)];
                case 19:
                    _b.sent();
                    (0, generateTsAbis_js_1.generateTsAbis)("DeployMockErc20.s.sol", false);
                    return [3 /*break*/, 21];
                case 20:
                    console.log("Invalid choice. Exiting...");
                    return [3 /*break*/, 21];
                case 21:
                    rl.close();
                    return [2 /*return*/];
            }
        });
    });
}
main();
