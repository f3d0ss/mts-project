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
- [] Scripts for interaction
- [] Experience reviews
- [] GnosisSafe
- [] UI
- [] DAO

TOTHINK:

- Use factory for ResturantToken
  [link](https://consensys.net/diligence/blog/2019/09/factories-improve-smart-contract-security/) drawback: much more
  gas
