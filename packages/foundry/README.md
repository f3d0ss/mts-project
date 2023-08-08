# Stuff
PK  - Address
0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80  - 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d  - 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a  - 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC

0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6  - 0x90F79bf6EB2c4f870365E785982E1f101E93b906

# Deploy

For the deploy you need to set the enviroment variables a with private keys for each chain where you want to deply your
contract (and fund those addresses) `ANVIL_PRIVATE_KEY`, `SEPOLIA_PRIVATE_KEY`, `GOERLI_PRIVATE_KEY`, etc.

## Deploy Safe

### Setup

First you need to set the enviroment variables:
- `CHAIN_CONTROLLER_OWNER` for the chain on which you want to deploy the
contract to the address of the owner of the MTSController
- `SAFE_VERSION` to the version of the safe you want to deploy (`1.3.0` or `1.4.1`, default:`1.3.0`) *note: The safe webapp recognise only v1.3.0 safes for now*
### Deploy on Anvil

`forge script script/DeploySafe.s.sol --broadcast --sig "run(address[],uint256)" "[ADDRESS1, ADDRESS2, ...]" THRESHOLD --rpc-url localhost`

### Deploy on Sepolia

`forge script script/DeploySafe.s.sol --broadcast --sig "run(address[],uint256)" "[ADDRESS1, ADDRESS2, ...]" THRESHOLD --rpc-url sepolia`

### Deploy on Goerli

`forge script script/DeploySafe.s.sol --broadcast --sig "run(address[],uint256)" "[ADDRESS1, ADDRESS2, ...]" THRESHOLD --rpc-url goerli`

## Deploy MTSController

### Setup

### Deploy on Anvil

`forge script script/DeployMTSController.s.sol --broadcast --sig "run(address)" OWNER_ADDRESS --rpc-url localhost`

### Deploy on Sepolia

`forge script script/DeployMTSController.s.sol --broadcast --sig "run(address)" OWNER_ADDRESS --rpc-url sepolia`

### Deploy on Goerli

`forge script script/DeployMTSController.s.sol --broadcast --sig "run(address)" OWNER_ADDRESS --rpc-url goerli`

## Deploy a ResturantToken

### Setup

### Deploy on Anvil

`forge script script/DeployResturantWithSafe.s.sol --broadcast --sig "run(address,string,string,address,address,uint256[])" RESTURANT_OWNER "TOKEN_NAME" "TOKEN_SYMBOL" CONTROLLER_ADDRESS SAFE_ADDRESS "[SAFE_PRIV_KEY,...]" --rpc-url localhost`

### Deploy on Sepolia

`forge script script/DeployResturantWithSafe.s.sol --broadcast --sig "run(address,string,string,address,address,uint256[])" RESTURANT_OWNER "TOKEN_NAME" "TOKEN_SYMBOL" CONTROLLER_ADDRESS SAFE_ADDRESS "[SAFE_PRIV_KEY,...]" --rpc-url sepolia`

### Deploy on Goerli

`forge script script/DeployResturantWithSafe.s.sol --broadcast --sig "run(address,string,string,address,address,uint256[])" RESTURANT_OWNER "TOKEN_NAME" "TOKEN_SYMBOL" CONTROLLER_ADDRESS SAFE_ADDRESS "[SAFE_PRIV_KEY,...]" --rpc-url goerli`



## Deploy Minimal Scenario With Safe

### Deploy on Anvil

`forge script script/DeployMinimalScenarioWithSafe.sol --broadcast --sig "run(uint256[],uint256,address,string,string)" "[SAFE_PRIV_KEY,...]" THRESHOLD RESTURANT_OWNER "TOKEN_NAME" "TOKEN_SYMBOL" --rpc-url localhost`

__________________________
`forge script script/DeploySafe.s.sol --broadcast --sig "run(address[],uint256)" "[0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266,0x70997970C51812dc3A010C7d01b50e0d17dc79C8,0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC]" 2 --rpc-url localhost`
Safe: 0xE3328B42382aED5C179a07d83527047E15DEd0B9
`forge script script/DeployMTSController.s.sol --broadcast --sig "run(address)" 0xE3328B42382aED5C179a07d83527047E15DEd0B9 --rpc-url localhost`
MTSController: 0x8476FC408B2dF4d03E9705FC2768d9179B62800c
`forge script script/DeployResturantWithSafe.s.sol --broadcast --sig "run(address,string,string,address,address,uint256[])" "0x90F79bf6EB2c4f870365E785982E1f101E93b906" "GoodResturant" "GR" 0x8476FC408B2dF4d03E9705FC2768d9179B62800c 0xE3328B42382aED5C179a07d83527047E15DEd0B9 "[42468054105998644681036035997014131563610289007175279352442773583210734106202,40606737760334725431406512677033654118342507952694270066784247067953537247501,77814517325470205911140941194401928579557062014761831930645393041380819009408]" --rpc-url localhost`


`forge script script/DeployResturantWithSafe.s.sol --broadcast --sig "run(address,string,string,address,address,uint256[])" "0x90F79bf6EB2c4f870365E785982E1f101E93b906" "GoodResturant" "GR" 0x8476FC408B2dF4d03E9705FC2768d9179B62800c 0xE3328B42382aED5C179a07d83527047E15DEd0B9 "[40606737760334725431406512677033654118342507952694270066784247067953537247501,42468054105998644681036035997014131563610289007175279352442773583210734106202,778145173 25470205911140941194401928579557062014761831930645393041380819009408]" --rpc-url localhost`


`forge script script/DeployMinimalScenarioWithSafe.s.sol --broadcast --sig "run(uint256,address,string,string)" 2 0x90F79bf6EB2c4f870365E785982E1f101E93b906 "NAME" "NM" --rpc-url localhost`
