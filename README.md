# Code for the Thesis work "Study and implementation of a platform for restaurants selling personalized culinary experiences"

🧪 This repo contain all the code to deploy the project.

⚙️ Started from the Scaffold-ETH 2 template. Built using NextJS, RainbowKit, Foudnry, Wagmi, and Typescript.

## Contents

- [Requirements](#requirements)
- [Quickstart](#quickstart)
- [Deploying your Smart Contracts to a Live Network](#deploying-your-smart-contracts-to-a-live-network)

## Requirements

Before you begin, you need to install the following tools:

- [Node (v18 LTS)](https://nodejs.org/en/download/)
- Yarn ([v1](https://classic.yarnpkg.com/en/docs/install/) or [v2+](https://yarnpkg.com/getting-started/install))
- [Git](https://git-scm.com/downloads)
- [Foundryup](https://book.getfoundry.sh/getting-started/installation)

## Quickstart

To get started, follow the steps below:

1. Clone this repo & install dependencies

```
git clone https://github.com/scaffold-eth/scaffold-eth-2.git
cd scaffold-eth-2
yarn install
foundryup
```

2. Create your `.env` file inside `packages/foundry`, you can edit from `.env.example`:

3. Run a local network in the first terminal:

```
yarn chain
```

This command starts a local Ethereum network using Anvil in Foundry. The network runs on your local machine and can be used for testing and development. You can customize the network configuration in `foundry.toml`

4. On a second terminal, deploy the test contract:

```
yarn deploy
```

This command deploys run a script in typescript which ask what do you want to deploy. Based on your decision the correct script in `packages/foundry/script/` is used to deploy the contracts to the network.

5. On a third terminal, start your NextJS app:

```
yarn start
```

Visit your app on: `http://localhost:3000`. You can interact with ui in the frontend. 

Run smart contract test with `yarn foundry:test`

## Deploying your Smart Contracts to a Live Network

Once you are ready to deploy your smart contracts, there are a few things you need to adjust.

1. Select the network

By default, `yarn deploy` will ask on which network you want to deplot the contracts.

2. Generate a new account or add one to deploy the contract(s) from. Additionally you will need to add your Alchemy API key. Rename `.env.example` to `.env` and fill the required keys.

```
ALCHEMY_API_KEY="",
CHAIN_PRIVATE_KEY=""
```

3. Deploy your smart contract(s)

Run the command below to deploy the smart contract and select the target network. Make sure to have some funds in your deployer account to pay for the transaction.

```
yarn deploy
```

4. Deploy and verify your smart contract(s)

You can deploy & verify your smart contract on Etherscan by reply `y` when the script ask if you want to verify:

```
Do you want to verify the contracts? (y/N):
```

We welcome contributions to Scaffold-ETH 2!

Please see [CONTRIBUTING.MD](https://github.com/scaffold-eth/scaffold-eth-2/blob/main/CONTRIBUTING.md) for more information and guidelines for contributing to Scaffold-ETH 2.
