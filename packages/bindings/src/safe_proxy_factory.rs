pub use safe_proxy_factory::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod safe_proxy_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "createChainSpecificProxyWithNonce",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createChainSpecificProxyWithNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("saltNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract SafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createProxyWithCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createProxyWithCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("saltNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callback"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IProxyCreationCallback",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract SafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createProxyWithNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createProxyWithNonce",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("saltNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract SafeProxy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxyCreationCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxyCreationCode"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ProxyCreation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProxyCreation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("singleton"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SAFEPROXYFACTORY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x08\xEE\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80cS\xE5\xD95\x11a\0PW\x80cS\xE5\xD95\x14a\0\xB7W\x80c\xD1\x8A\xF5M\x14a\0\xCCW\x80c\xEC\x9E\x80\xBB\x14a\0\xDFW`\0\x80\xFD[\x80c\x16\x88\xF0\xB9\x14a\0lW\x80c4\x08\xE4p\x14a\0\xA9W[`\0\x80\xFD[a\0\x7Fa\0z6`\x04a\x05\xD2V[a\0\xF2V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@QF\x81R` \x01a\0\xA0V[a\0\xBFa\x01\x94V[`@Qa\0\xA0\x91\x90a\x06\x99V[a\0\x7Fa\0\xDA6`\x04a\x06\xB3V[a\x01\xDCV[a\0\x7Fa\0\xED6`\x04a\x05\xD2V[a\x02\xF8V[`\0\x80\x83\x80Q\x90` \x01 \x83`@Q` \x01a\x01\x18\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x01;\x85\x85\x83a\x03*V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x91\x93P\x90\x83\x16\x90\x7FOQ\xFA\xF6\xC4V\x1F\xF9_\x06vW\xE449\xF0\xF8V\xD9|\x04\xD9\xEC\x90p\xA6\x19\x9A\xD4\x18\xE25\x90` \x01`@Q\x80\x91\x03\x90\xA2P\x93\x92PPPV[```@Q\x80` \x01a\x01\xA6\x90a\x04\xC6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x91\x90PV[`\0\x80\x83\x83`@Q` \x01a\x02 \x92\x91\x90\x91\x82R``\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1C\x90Pa\x02F\x86\x86\x83a\0\xF2V[\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15a\x02\xEFW`@Q\x7F\x1ER\xB5\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90c\x1ER\xB5\x18\x90a\x02\xBC\x90\x85\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01a\x07\x1FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xEAW=`\0\x80>=`\0\xFD[PPPP[P\x94\x93PPPPV[`\0\x80\x83\x80Q\x90` \x01 \x83a\x03\x0BF\x90V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01a\x01\x18V[`\0\x83;a\x03\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSingleton contract not deployed\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80` \x01a\x03\xAB\x90a\x04\xC6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x90Ra\x04\x03\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90` \x01a\x07iV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82\x81Q\x82` \x01`\0\xF5\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x04\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FCreate2 call failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x90V[\x83Q\x15a\x04\xBEW`\0\x80`\0\x86Q` \x88\x01`\0\x87Z\xF1\x03a\x04\xBEW`\0\x80\xFD[P\x93\x92PPPV[a\x01b\x80a\x07\x8C\x839\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xF5W`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x058W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05SWa\x05Sa\x04\xF8V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05\x99Wa\x05\x99a\x04\xF8V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\xB2W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\xE7W`\0\x80\xFD[\x835a\x05\xF2\x81a\x04\xD3V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x0EW`\0\x80\xFD[a\x06\x1A\x86\x82\x87\x01a\x05'V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x06FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06.V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06g\x81` \x86\x01` \x86\x01a\x06+V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06\xAC` \x83\x01\x84a\x06OV[\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x06\xC9W`\0\x80\xFD[\x845a\x06\xD4\x81a\x04\xD3V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF0W`\0\x80\xFD[a\x06\xFC\x87\x82\x88\x01a\x05'V[\x93PP`@\x85\x015\x91P``\x85\x015a\x07\x14\x81a\x04\xD3V[\x93\x96\x92\x95P\x90\x93PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra\x07X`\x80\x83\x01\x85a\x06OV[\x90P\x82``\x83\x01R\x95\x94PPPPPV[`\0\x83Qa\x07{\x81\x84` \x88\x01a\x06+V[\x91\x90\x91\x01\x91\x82RP` \x01\x91\x90PV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x01b8\x03\x80a\x01b\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xB9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FInvalid singleton address provid`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\xE9V[`\0` \x82\x84\x03\x12\x15a\0\xCBW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE2W`\0\x80\xFD[\x93\x92PPPV[`k\x80a\0\xF7`\09`\0\xF3\xFE`\x80`@R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16\x7F\xA6\x19Hn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x03`MW\x80\x82R` \x82\xF3[6\x82\x837\x81\x826\x84\x84Z\xF4\x90P=\x82\x83>\x80`fW=\x82\xFD[P=\x81\xF3";
    /// The bytecode of the contract.
    pub static SAFEPROXYFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80cS\xE5\xD95\x11a\0PW\x80cS\xE5\xD95\x14a\0\xB7W\x80c\xD1\x8A\xF5M\x14a\0\xCCW\x80c\xEC\x9E\x80\xBB\x14a\0\xDFW`\0\x80\xFD[\x80c\x16\x88\xF0\xB9\x14a\0lW\x80c4\x08\xE4p\x14a\0\xA9W[`\0\x80\xFD[a\0\x7Fa\0z6`\x04a\x05\xD2V[a\0\xF2V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@QF\x81R` \x01a\0\xA0V[a\0\xBFa\x01\x94V[`@Qa\0\xA0\x91\x90a\x06\x99V[a\0\x7Fa\0\xDA6`\x04a\x06\xB3V[a\x01\xDCV[a\0\x7Fa\0\xED6`\x04a\x05\xD2V[a\x02\xF8V[`\0\x80\x83\x80Q\x90` \x01 \x83`@Q` \x01a\x01\x18\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x01;\x85\x85\x83a\x03*V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x91\x93P\x90\x83\x16\x90\x7FOQ\xFA\xF6\xC4V\x1F\xF9_\x06vW\xE449\xF0\xF8V\xD9|\x04\xD9\xEC\x90p\xA6\x19\x9A\xD4\x18\xE25\x90` \x01`@Q\x80\x91\x03\x90\xA2P\x93\x92PPPV[```@Q\x80` \x01a\x01\xA6\x90a\x04\xC6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x91\x90PV[`\0\x80\x83\x83`@Q` \x01a\x02 \x92\x91\x90\x91\x82R``\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1C\x90Pa\x02F\x86\x86\x83a\0\xF2V[\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15a\x02\xEFW`@Q\x7F\x1ER\xB5\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90c\x1ER\xB5\x18\x90a\x02\xBC\x90\x85\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01a\x07\x1FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xEAW=`\0\x80>=`\0\xFD[PPPP[P\x94\x93PPPPV[`\0\x80\x83\x80Q\x90` \x01 \x83a\x03\x0BF\x90V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01a\x01\x18V[`\0\x83;a\x03\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FSingleton contract not deployed\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80` \x01a\x03\xAB\x90a\x04\xC6V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x90Ra\x04\x03\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90` \x01a\x07iV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82\x81Q\x82` \x01`\0\xF5\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x04\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FCreate2 call failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x90V[\x83Q\x15a\x04\xBEW`\0\x80`\0\x86Q` \x88\x01`\0\x87Z\xF1\x03a\x04\xBEW`\0\x80\xFD[P\x93\x92PPPV[a\x01b\x80a\x07\x8C\x839\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xF5W`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x058W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05SWa\x05Sa\x04\xF8V[`@Q`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05\x99Wa\x05\x99a\x04\xF8V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\xB2W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\xE7W`\0\x80\xFD[\x835a\x05\xF2\x81a\x04\xD3V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x0EW`\0\x80\xFD[a\x06\x1A\x86\x82\x87\x01a\x05'V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x06FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06.V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06g\x81` \x86\x01` \x86\x01a\x06+V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06\xAC` \x83\x01\x84a\x06OV[\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x06\xC9W`\0\x80\xFD[\x845a\x06\xD4\x81a\x04\xD3V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF0W`\0\x80\xFD[a\x06\xFC\x87\x82\x88\x01a\x05'V[\x93PP`@\x85\x015\x91P``\x85\x015a\x07\x14\x81a\x04\xD3V[\x93\x96\x92\x95P\x90\x93PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP`\x80`@\x83\x01Ra\x07X`\x80\x83\x01\x85a\x06OV[\x90P\x82``\x83\x01R\x95\x94PPPPPV[`\0\x83Qa\x07{\x81\x84` \x88\x01a\x06+V[\x91\x90\x91\x01\x91\x82RP` \x01\x91\x90PV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x01b8\x03\x80a\x01b\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xB9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FInvalid singleton address provid`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\xE9V[`\0` \x82\x84\x03\x12\x15a\0\xCBW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE2W`\0\x80\xFD[\x93\x92PPPV[`k\x80a\0\xF7`\09`\0\xF3\xFE`\x80`@R`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16\x7F\xA6\x19Hn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x03`MW\x80\x82R` \x82\xF3[6\x82\x837\x81\x826\x84\x84Z\xF4\x90P=\x82\x83>\x80`fW=\x82\xFD[P=\x81\xF3";
    /// The deployed bytecode of the contract.
    pub static SAFEPROXYFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SafeProxyFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SafeProxyFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SafeProxyFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SafeProxyFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SafeProxyFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SafeProxyFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SafeProxyFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SAFEPROXYFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SAFEPROXYFACTORY_ABI.clone(),
                SAFEPROXYFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `createChainSpecificProxyWithNonce` (0xec9e80bb) function
        pub fn create_chain_specific_proxy_with_nonce(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([236, 158, 128, 187], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithCallback` (0xd18af54d) function
        pub fn create_proxy_with_callback(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [209, 138, 245, 77],
                    (singleton, initializer, salt_nonce, callback),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithNonce` (0x1688f0b9) function
        pub fn create_proxy_with_nonce(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 136, 240, 185], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyCreationCode` (0x53e5d935) function
        pub fn proxy_creation_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([83, 229, 217, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProxyCreation` event
        pub fn proxy_creation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreationFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SafeProxyFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ProxyCreation", abi = "ProxyCreation(address,address)")]
    pub struct ProxyCreationFilter {
        #[ethevent(indexed)]
        pub proxy: ::ethers::core::types::Address,
        pub singleton: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createChainSpecificProxyWithNonce` function with signature `createChainSpecificProxyWithNonce(address,bytes,uint256)` and selector `0xec9e80bb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createChainSpecificProxyWithNonce",
        abi = "createChainSpecificProxyWithNonce(address,bytes,uint256)"
    )]
    pub struct CreateChainSpecificProxyWithNonceCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createProxyWithCallback",
        abi = "createProxyWithCallback(address,bytes,uint256,address)"
    )]
    pub struct CreateProxyWithCallbackCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createProxyWithNonce",
        abi = "createProxyWithNonce(address,bytes,uint256)"
    )]
    pub struct CreateProxyWithNonceCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proxyCreationCode", abi = "proxyCreationCode()")]
    pub struct ProxyCreationCodeCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SafeProxyFactoryCalls {
        CreateChainSpecificProxyWithNonce(CreateChainSpecificProxyWithNonceCall),
        CreateProxyWithCallback(CreateProxyWithCallbackCall),
        CreateProxyWithNonce(CreateProxyWithNonceCall),
        GetChainId(GetChainIdCall),
        ProxyCreationCode(ProxyCreationCodeCall),
    }
    impl ::ethers::core::abi::AbiDecode for SafeProxyFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CreateChainSpecificProxyWithNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CreateChainSpecificProxyWithNonce(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CreateProxyWithCallback(decoded));
            }
            if let Ok(decoded)
                = <CreateProxyWithNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CreateProxyWithNonce(decoded));
            }
            if let Ok(decoded)
                = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded)
                = <ProxyCreationCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProxyCreationCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SafeProxyFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreateChainSpecificProxyWithNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxyCreationCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SafeProxyFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreateChainSpecificProxyWithNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxyWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyCreationCode(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateChainSpecificProxyWithNonceCall>
    for SafeProxyFactoryCalls {
        fn from(value: CreateChainSpecificProxyWithNonceCall) -> Self {
            Self::CreateChainSpecificProxyWithNonce(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithCallbackCall> for SafeProxyFactoryCalls {
        fn from(value: CreateProxyWithCallbackCall) -> Self {
            Self::CreateProxyWithCallback(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithNonceCall> for SafeProxyFactoryCalls {
        fn from(value: CreateProxyWithNonceCall) -> Self {
            Self::CreateProxyWithNonce(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for SafeProxyFactoryCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<ProxyCreationCodeCall> for SafeProxyFactoryCalls {
        fn from(value: ProxyCreationCodeCall) -> Self {
            Self::ProxyCreationCode(value)
        }
    }
    ///Container type for all return fields from the `createChainSpecificProxyWithNonce` function with signature `createChainSpecificProxyWithNonce(address,bytes,uint256)` and selector `0xec9e80bb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateChainSpecificProxyWithNonceReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateProxyWithCallbackReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateProxyWithNonceReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProxyCreationCodeReturn(pub ::ethers::core::types::Bytes);
}
