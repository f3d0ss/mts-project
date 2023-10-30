## Deploy a ResturantToken contract

There two way of deploying the `ResturantToken` and add it to the system/controller

### Method 1

The resturant owner deply the his `ResturantToken` and ask the `MTSController` owner (future DAO) to add it to the
system registry

- PRO: More flexible (resturant can create any contract)
- CON: resturant owner may create a `RestrantToken` that will never be added to the MTSController (waste of gas)

### Method 2

The resturant owner ask the `MTSController` owner (future DAO) to execute a factory method of the `MTSController` that
create a `ResturantToken`

## MACRO TODO

- [x] Scripts for contract deployment
- [x] Resturant get money if token not used after time
- [-] Scripts for interaction
- [x] Experience reviews
- [~] GnosisSafe
- [x] UI
- [] DAO
- [] Test
  - [~] ResturantToken 
  - [] MTSController

TOTHINK:

- [x] Use factory for ResturantToken
  [link](https://consensys.net/diligence/blog/2019/09/factories-improve-smart-contract-security/) drawback: much more
  gas

- [x] Factory of proxy (save gas) EIP 1167

- Open dispute if client find resturant closed

- Way of discount experience for certain user (eg. a user that bougth 10 exp can buy the 11th at a discount (no fee))

- Add versioning of ResturantToken contract

- Dao should not need timelock since there is no meaning in exiting the DAO since you cannot transfer voting power

- Mint with signed message from Resturant

- Enforce token URI to be ipfs URI