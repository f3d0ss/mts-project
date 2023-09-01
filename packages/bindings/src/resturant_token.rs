pub use resturant_token::*;
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
pub mod resturant_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_mtsController"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EXPIRATION_RANGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EXPIRATION_RANGE"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("burnNftNotSold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnNftNotSold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burnTicketNotConsumed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "burnTicketNotConsumed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("buyNFT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("buyNFT"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getControllerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getControllerAddress",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCounter"),
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
                    ::std::borrow::ToOwned::to_owned("getNft"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNft"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ResturantToken.NFT",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("locked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refund"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("refund"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reservationDate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uri"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendReview"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendReview"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uri"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("tokenOfOwnerByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tokenOfOwnerByIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("useTicket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("useTicket"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Locked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReviewPosted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReviewPosted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reviewURI"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unlocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unlocked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ResturantToken__InvalidSignatureFromClient",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__InvalidSignatureFromClient",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ResturantToken__PriceNotAcceptable",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__PriceNotAcceptable",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ResturantToken__SenderIsNotNftOwner",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__SenderIsNotNftOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ResturantToken__SenderIsNotTheController",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__SenderIsNotTheController",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ResturantToken__TokenAlreadyUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__TokenAlreadyUsed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ResturantToken__TokenNotForSale"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__TokenNotForSale",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ResturantToken__TokenNotSold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__TokenNotSold",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ResturantToken__TokenNotUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__TokenNotUsed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ResturantToken__TokenNotYetBurnable",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__TokenNotYetBurnable",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ResturantToken__TransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResturantToken__TransferFailed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RESTURANTTOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0<\x0C8\x03\x80b\0<\x0C\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\xBEV[\x81\x81`\0b\0\0D\x83\x82b\0\x03\xDDV[P`\x01b\0\0S\x82\x82b\0\x03\xDDV[PP`\x06\x80T`\xFF\x19\x16\x90UPb\0\0k3b\0\0\x9BV[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ub\0\0\x91\x84b\0\0\xF5V[PPPPb\0\x04\xA9V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16a\x01\0\x81\x81\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x85\x16\x17\x90\x94U`@Q\x93\x90\x92\x04\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\0\0\xFFb\0\x01xV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x01jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[b\0\x01u\x81b\0\0\x9BV[PV[`\x06T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x163\x14b\0\x01\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x01aV[V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF4W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x02!W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02>Wb\0\x02>b\0\x01\xF9V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x02iWb\0\x02ib\0\x01\xF9V[\x81`@R\x83\x81R` \x92P\x86\x83\x85\x88\x01\x01\x11\x15b\0\x02\x86W`\0\x80\xFD[`\0\x91P[\x83\x82\x10\x15b\0\x02\xAAW\x85\x82\x01\x83\x01Q\x81\x83\x01\x84\x01R\x90\x82\x01\x90b\0\x02\x8BV[`\0\x93\x81\x01\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02\xD5W`\0\x80\xFD[b\0\x02\xE0\x85b\0\x01\xDCV[\x93Pb\0\x02\xF0` \x86\x01b\0\x01\xDCV[`@\x86\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03\x0EW`\0\x80\xFD[b\0\x03\x1C\x88\x83\x89\x01b\0\x02\x0FV[\x93P``\x87\x01Q\x91P\x80\x82\x11\x15b\0\x033W`\0\x80\xFD[Pb\0\x03B\x87\x82\x88\x01b\0\x02\x0FV[\x91PP\x92\x95\x91\x94P\x92PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x03cW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x03\x84WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x03\xD8W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x03\xB3WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x03\xD4W\x82\x81U`\x01\x01b\0\x03\xBFV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\xF9Wb\0\x03\xF9b\0\x01\xF9V[b\0\x04\x11\x81b\0\x04\n\x84Tb\0\x03NV[\x84b\0\x03\x8AV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x04IW`\0\x84\x15b\0\x040WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x03\xD4V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x04zW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x04YV[P\x85\x82\x10\x15b\0\x04\x99W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a7S\x80b\0\x04\xB9`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x024W`\x005`\xE0\x1C\x80cYj\xC5\xBD\x11a\x018W\x80c\x95\xD8\x9BA\x11a\0\xB0W\x80c\xC5s\x80\xA2\x11a\0\x7FW\x80c\xC9\xE7\xF4\xB7\x11a\0dW\x80c\xC9\xE7\xF4\xB7\x14a\x06\xA7W\x80c\xE9\x85\xE9\xC5\x14a\x06\xC7W\x80c\xF2\xFD\xE3\x8B\x14a\x07\x10W`\0\x80\xFD[\x80c\xC5s\x80\xA2\x14a\x06iW\x80c\xC8{V\xDD\x14a\x06\x87W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x14a\x05\xC5W\x80c\xA2,\xB4e\x14a\x05\xDAW\x80c\xB4Z<\x0E\x14a\x05\xFAW\x80c\xB8\x8DO\xDE\x14a\x06IW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\x01\x07W\x80c\x85\xB3\xA7\xDB\x11a\0\xECW\x80c\x85\xB3\xA7\xDB\x14a\x05mW\x80c\x8A\xDA\x06n\x14a\x05\x8DW\x80c\x8D\xA5\xCB[\x14a\x05\xA2W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x05CW\x80c\x84V\xCBY\x14a\x05XW`\0\x80\xFD[\x80cYj\xC5\xBD\x14a\x04\xD4W\x80c\\\x97Z\xBB\x14a\x04\xEBW\x80ccR!\x1E\x14a\x05\x03W\x80cp\xA0\x821\x14a\x05#W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\xCBW\x80cA\x98\x1A\xB1\x11a\x01\x9AW\x80cB\x84.\x0E\x11a\x01\x7FW\x80cB\x84.\x0E\x14a\x04\x81W\x80cOl\xCC\xE7\x14a\x04\xA1W\x80cQ\xED\x82\x88\x14a\x04\xC1W`\0\x80\xFD[\x80cA\x98\x1A\xB1\x14a\x044W\x80cA\xF6;\xFD\x14a\x04TW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x03\xBFW\x80c'\x8E\xCD\xE1\x14a\x03\xDFW\x80c/t\\Y\x14a\x03\xFFW\x80c?K\xA8:\x14a\x04\x1FW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x02\x07W\x80c\t^\xA7\xB3\x14a\x02\xEAW\x80c\x15\x0Bz\x02\x14a\x03\nW\x80c\x18\x16\r\xDD\x14a\x03\x80W\x80c\x1A4\x1D\xDC\x14a\x03\x9FW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x029W\x80c\x05\x1FWX\x14a\x02nW\x80c\x06\xFD\xDE\x03\x14a\x02\x90W\x80c\x08\x18\x12\xFC\x14a\x02\xB2W[`\0\x80\xFD[4\x80\x15a\x02EW`\0\x80\xFD[Pa\x02Ya\x02T6`\x04a.\x85V[a\x070V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02zW`\0\x80\xFD[Pa\x02\x8Ea\x02\x896`\x04a.\xA9V[a\x07AV[\0[4\x80\x15a\x02\x9CW`\0\x80\xFD[Pa\x02\xA5a\x07\xB7V[`@Qa\x02e\x91\x90a/\x08V[4\x80\x15a\x02\xBEW`\0\x80\xFD[Pa\x02\xD2a\x02\xCD6`\x04a.\xA9V[a\x08IV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02eV[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x02\x8Ea\x03\x056`\x04a/7V[a\x08pV[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x03Oa\x03%6`\x04a/\xAAV[\x7F\x15\x0Bz\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x02eV[4\x80\x15a\x03\x8CW`\0\x80\xFD[P`\tT[`@Q\x90\x81R` \x01a\x02eV[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x02\x8Ea\x03\xBA6`\x04a0\xBEV[a\t\xA6V[4\x80\x15a\x03\xCBW`\0\x80\xFD[Pa\x02\x8Ea\x03\xDA6`\x04a1EV[a\x0B\xC4V[4\x80\x15a\x03\xEBW`\0\x80\xFD[Pa\x02\x8Ea\x03\xFA6`\x04a.\xA9V[a\x0CKV[4\x80\x15a\x04\x0BW`\0\x80\xFD[Pa\x03\x91a\x04\x1A6`\x04a/7V[a\r\xA3V[4\x80\x15a\x04+W`\0\x80\xFD[Pa\x02\x8Ea\x0EKV[4\x80\x15a\x04@W`\0\x80\xFD[Pa\x02\x8Ea\x04O6`\x04a.\xA9V[a\x0E\x99V[4\x80\x15a\x04`W`\0\x80\xFD[Pa\x04ta\x04o6`\x04a.\xA9V[a\x0F\x95V[`@Qa\x02e\x91\x90a1\x81V[4\x80\x15a\x04\x8DW`\0\x80\xFD[Pa\x02\x8Ea\x04\x9C6`\x04a1EV[a\x11\x8FV[4\x80\x15a\x04\xADW`\0\x80\xFD[Pa\x03\x91a\x04\xBC6`\x04a.\xA9V[a\x11\xAAV[a\x02\x8Ea\x04\xCF6`\x04a.\xA9V[a\x12NV[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x03\x91b\x01Q\x80\x81V[4\x80\x15a\x04\xF7W`\0\x80\xFD[P`\x06T`\xFF\x16a\x02YV[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x02\xD2a\x05\x1E6`\x04a.\xA9V[a\x13\xA8V[4\x80\x15a\x05/W`\0\x80\xFD[Pa\x03\x91a\x05>6`\x04a1\xFCV[a\x14\rV[4\x80\x15a\x05OW`\0\x80\xFD[Pa\x02\x8Ea\x14\xA7V[4\x80\x15a\x05dW`\0\x80\xFD[Pa\x02\x8Ea\x14\xB9V[4\x80\x15a\x05yW`\0\x80\xFD[Pa\x02\x8Ea\x05\x886`\x04a2\x17V[a\x15\x05V[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x03\x91a\x16\x1EV[4\x80\x15a\x05\xAEW`\0\x80\xFD[P`\x06Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02\xD2V[4\x80\x15a\x05\xD1W`\0\x80\xFD[Pa\x02\xA5a\x16.V[4\x80\x15a\x05\xE6W`\0\x80\xFD[Pa\x02\x8Ea\x05\xF56`\x04a2qV[a\x16=V[4\x80\x15a\x06\x06W`\0\x80\xFD[Pa\x02Ya\x06\x156`\x04a.\xA9V[`\0\x90\x81R`\x0C` R`@\x90 `\x01\x01Tx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x90V[4\x80\x15a\x06UW`\0\x80\xFD[Pa\x02\x8Ea\x06d6`\x04a2\xA8V[a\x16HV[4\x80\x15a\x06uW`\0\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16a\x02\xD2V[4\x80\x15a\x06\x93W`\0\x80\xFD[Pa\x02\xA5a\x06\xA26`\x04a.\xA9V[a\x16\xD0V[4\x80\x15a\x06\xB3W`\0\x80\xFD[Pa\x02\x8Ea\x06\xC26`\x04a2\xF8V[a\x17uV[4\x80\x15a\x06\xD3W`\0\x80\xFD[Pa\x02Ya\x06\xE26`\x04a3DV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[4\x80\x15a\x07\x1CW`\0\x80\xFD[Pa\x02\x8Ea\x07+6`\x04a1\xFCV[a\x19ZV[`\0a\x07;\x82a\x19\xEAV[\x92\x91PPV[a\x07Ia\x1A@V[\x800a\x07j\x82`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xAAW`@Q\x7F\x87[\xC7j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xB3\x82a\x1A\xA0V[PPV[```\0\x80Ta\x07\xC6\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xF2\x90a3wV[\x80\x15a\x08?W\x80`\x1F\x10a\x08\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x08T\x82a\x1B[V[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x08{\x82a\x13\xA8V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\t%WPa\t%\x813a\x06\xE2V[a\t\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\t\xA1\x83\x83a\x1B\xBFV[PPPV[a\t\xAEa\x1A@V[`\rT`@Q\x7FZ\xE9\xCEu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x87\x90R\x90\x91\x16\x90cZ\xE9\xCEu\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n>\x91\x90a3\xCAV[a\ntW`@Q\x7F\x8B\x8DTM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\n\x7F`\x0BT\x90V[\x90Pa\n\x8F`\x0B\x80T`\x01\x01\x90UV[a\n\x990\x82a\x1CEV[`@\x80Q`\xC0\x81\x01\x82R\x86\x81R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16` \x80\x84\x01\x91\x82Rc\xFF\xFF\xFF\xFF\x80\x89\x16\x85\x87\x01\x90\x81R`\0``\x87\x01\x81\x81R\x88Q\x80\x86\x01\x8AR\x82\x81R`\x80\x89\x01\x90\x81R`\xA0\x89\x01\x8C\x90R\x8A\x83R`\x0C\x90\x95R\x97\x90 \x86Q\x81U\x93Q`\x01\x85\x01\x80T\x92Q\x98Q\x15\x15x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x99\x90\x94\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x91\x90\x96\x16\x17\x17\x95\x90\x95\x16\x94\x90\x94\x17\x90\x91U\x91Q\x90\x91\x90`\x02\x82\x01\x90a\x0B\xA5\x90\x82a4-V[P`\xA0\x82\x01Q`\x03\x82\x01\x90a\x0B\xBA\x90\x82a4-V[PPPPPPPPV[a\x0B\xCE3\x82a\x1C_V[a\x0C@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC721: caller is not token owne`D\x82\x01R\x7Fr or approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\t\xA1\x83\x83\x83a\x1C\xDEV[a\x0CSa\x1A@V[`\0\x81\x81R`\x02` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x0C\x8FWP`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x14[\x15a\x0C\xC6W`@Q\x7F1V\xEF\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80c\xA9\x05\x9C\xBBa\r\n\x86`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\roW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x93\x91\x90a3\xCAV[Pa\r\x9D\x84a\x1A\xA0V[PPPPV[`\0a\r\xAE\x83a\x14\rV[\x82\x10a\x0E\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FERC721Enumerable: owner index ou`D\x82\x01R\x7Ft of bounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x8FW`@Q\x7F\xD2U\xF1-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x97a\x1F,V[V[a\x0E\xA1a\x1A@V[`\0\x81\x81R`\x02` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x0E\xDDWP`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x14[\x15a\x0F\x14W`@Q\x7F1V\xEF\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x90 `\x01\x01TB\x90a\x0FT\x90b\x01Q\x80\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a5\x1CV[\x11\x15a\x0F\x8CW`@Q\x7F<\x84\xFA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xAA\x82a\x1F\x9CV[a\x0F\xDF`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x15\x15\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R`\x0C` \x90\x81R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x82\x01\x93\x90\x93Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x04c\xFF\xFF\xFF\xFF\x16\x93\x81\x01\x93\x90\x93Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15``\x83\x01R`\x02\x81\x01\x80T`\x80\x84\x01\x91\x90a\x10t\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xA0\x90a3wV[\x80\x15a\x10\xEDW\x80`\x1F\x10a\x10\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xEDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x11\x06\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x112\x90a3wV[\x80\x15a\x11\x7FW\x80`\x1F\x10a\x11TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\x7FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11bW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[a\t\xA1\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x16HV[`\0a\x11\xB5`\tT\x90V[\x82\x10a\x12)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FERC721Enumerable: global index o`D\x82\x01R\x7Fut of bounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\t\x82\x81T\x81\x10a\x12<Wa\x12<a5/V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x800a\x12o\x82`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xAFW`@Q\x7F\x87[\xC7j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x80\x82 \x80T`\x01\x90\x91\x01T\x91Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R\x90\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x82\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x137W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13[\x91\x90a3\xCAV[\x90P\x80a\x13\x94W`@Q\x7F\xFC\xED70\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x13\xA00\x82\x88a\x1C\xDEV[PPPPPPV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x07;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FERC721: invalid token ID\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01R\x7Flid owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x14\xAFa\x1A@V[a\x0E\x97`\0a!\xF7V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xFDW`@Q\x7F\xD2U\xF1-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x97a\"hV[`\0\x83\x81R`\x0C` R`@\x81 `\x01\x01T\x84\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15\x90\x03a\x15sW`@Q\x7F\xD2U9\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x833\x80a\x15\x7F\x83a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xBFW`@Q\x7F\xC9\xE1M\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x81R`\x0C` R`@\x90 `\x02\x01a\x15\xDB\x85\x87\x83a5^V[P\x85\x7F\xE477#\x90\x19G\xF3\xE6|\x8F\xE5\xCCA\xB9\x18dmm!EV\xE2\x93\xED\xCD\xB4\x83\xB0Q\x87\xA2\x86\x86`@Qa\x16\x0E\x92\x91\x90a6\x1EV[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0a\x16)`\x0BT\x90V[\x90P\x90V[```\x01\x80Ta\x07\xC6\x90a3wV[a\x07\xB33\x83\x83a\"\xC3V[a\x16R3\x83a\x1C_V[a\x16\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC721: caller is not token owne`D\x82\x01R\x7Fr or approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\r\x9D\x84\x84\x84\x84a#\xAFV[`\0\x81\x81R`\x0C` R`@\x90 `\x03\x01\x80T``\x91\x90a\x16\xF0\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\x1C\x90a3wV[\x80\x15a\x17iW\x80`\x1F\x10a\x17>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17iV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17LW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[a\x17}a\x1A@V[`\0\x81\x81R`\x02` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x17\xB9WP`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x14[\x15a\x17\xF0W`@Q\x7F1V\xEF\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x90 `\x01\x90\x81\x01T\x83\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15\x90\x03a\x18`W`@Q\x7F\x03\x8E\x9B7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0a\x18\xBE\x82\x860\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa$8\x92PPPV[\x90P\x80a\x18\xF7W`@Q\x7F>\\\x81\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\0\x85a\x1F\x9CV[PPP`\0\x91\x82RP`\x0C` R`@\x90 `\x01\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UPPV[a\x19ba\x1A@V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\x19\xE7\x81a!\xF7V[PV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fx\x0E\x9Dc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x07;WPa\x07;\x82a$\xCBV[`\x06T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x163\x14a\x0E\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\0V[`\0a\x1A\xAB\x82a\x13\xA8V[\x90Pa\x1A\xBB\x81`\0\x84`\x01a%\xAEV[a\x1A\xC4\x82a\x13\xA8V[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R`\x03\x84R\x82\x85 \x80T`\0\x19\x01\x90U\x87\x85R`\x02\x90\x93R\x81\x84 \x80T\x90\x91\x16\x90UQ\x92\x93P\x84\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x19\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FERC721: invalid token ID\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0\x81\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x1C\x0C\x82a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x07\xB3\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa%\xBAV[`\0\x80a\x1Ck\x83a\x13\xA8V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x1C\xB2WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x1C\xD6WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xCB\x84a\x08IV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xF1\x82a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1DmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC721: transfer from incorrect `D\x82\x01R\x7Fowner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01R\x7Fress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\x1D\xF5\x83\x83\x83`\x01a%\xAEV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x1E\x08\x82a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC721: transfer from incorrect `D\x82\x01R\x7Fowner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4PPPV[a\x1F4a&CV[`\x06\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\0\x81\x81R`\x0C` R`@\x80\x82 \x80T`\x01\x90\x91\x01T`\rT\x92Q\x7FOE\x98\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01\x81\x90R\x92\x94\x92\x93\x91\x90\x91\x16\x90cOE\x98\x94\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a @\x91\x90a6MV[\x90Pa'\x10\x81\x10a \x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Ffee can't be more than 100%\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0a'\x10a \xA2\x83\x86a6fV[a \xAC\x91\x90a6}V[`\rT`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92P\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!>\x91\x90a3\xCAV[P\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa!f`\x06T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x16\x90V[a!p\x84\x88a6\xB8V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA0\x91\x90a3\xCAV[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16a\x01\0\x81\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x85\x16\x17\x90\x94U`@Q\x93\x90\x92\x04\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\"pa&\x95V[`\x06\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x1F\x7F3\x90V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a#$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a#\xBA\x84\x84\x84a\x1C\xDEV[a#\xC6\x84\x84\x84\x84a&\xE8V[a\r\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01R\x7Fceiver implementer\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\0\x80a$E\x85\x85a(\x86V[\x90P`\0a$\xA0\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P\x86`\x01`\x01`\xA0\x1B\x03\x16a$\xB6\x82\x86a(\xE7V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x97\x96PPPPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x80\xACX\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a%^WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F[^\x13\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a\x07;WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14a\x07;V[a\r\x9D\x84\x84\x84\x84a)fV[a%\xC4\x83\x83a*\xA2V[a%\xD1`\0\x84\x84\x84a&\xE8V[a\t\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01R\x7Fceiver implementer\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\x06T`\xFF\x16a\x0E\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FPausable: not paused\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\x06T`\xFF\x16\x15a\x0E\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPausable: paused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a(~W`@Q\x7F\x15\x0Bz\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a'E\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a6\xCBV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a'\x80WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra'}\x91\x81\x01\x90a7\x07V[`\x01[a(3W=\x80\x80\x15a'\xAEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\xB3V[``\x91P[P\x80Q`\0\x03a(+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01R\x7Fceiver implementer\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[\x80Q\x81` \x01\xFD[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\x15\x0Bz\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x90Pa\x1C\xD6V[P`\x01a\x1C\xD6V[`\0\x82\x82`@Q` \x01a(\xC9\x92\x91\x90\x91\x82R``\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0\x80a(\xF6\x85a,SV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x83\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R\x92\x95P\x90\x93P\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a)QW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x97\x96PPPPPPPV[`\x01\x81\x11\x15a)\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FERC721Enumerable: consecutive tr`D\x82\x01R\x7Fansfers not supported\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[\x81`\x01`\x01`\xA0\x1B\x03\x85\x16a*9Wa*4\x81`\t\x80T`\0\x83\x81R`\n` R`@\x81 \x82\x90U`\x01\x82\x01\x83U\x91\x90\x91R\x7Fn\x15@\x17\x1Bl\x0C\x96\x0Bq\xA7\x02\r\x9F`\x07\x7Fj\xF91\xA8\xBB\xF5\x90\xDA\x02#\xDA\xCFu\xC7\xAF\x01UV[a*\\V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14a*\\Wa*\\\x85\x82a,\xC7V[`\x01`\x01`\xA0\x1B\x03\x84\x16a*xWa*s\x81a-dV[a*\x9BV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14a*\x9BWa*\x9B\x84\x82a.\x13V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a*\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\t\0V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a+]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\t\0V[a+k`\0\x83\x83`\x01a%\xAEV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a+\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\0\x80`\0\x83Q`A\x14a,\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Finvalid signature length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q\x90\x92`\0\x91\x90\x91\x1A\x90V[`\0`\x01a,\xD4\x84a\x14\rV[a,\xDE\x91\x90a6\xB8V[`\0\x83\x81R`\x08` R`@\x90 T\x90\x91P\x80\x82\x14a-1W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 T\x84\x84R\x81\x84 \x81\x90U\x83R`\x08\x90\x91R\x90 \x81\x90U[P`\0\x91\x82R`\x08` \x90\x81R`@\x80\x84 \x84\x90U`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R`\x07\x81R\x83\x83 \x91\x83RR\x90\x81 UV[`\tT`\0\x90a-v\x90`\x01\x90a6\xB8V[`\0\x83\x81R`\n` R`@\x81 T`\t\x80T\x93\x94P\x90\x92\x84\x90\x81\x10a-\x9EWa-\x9Ea5/V[\x90`\0R` `\0 \x01T\x90P\x80`\t\x83\x81T\x81\x10a-\xBFWa-\xBFa5/V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\n\x90\x91R`@\x80\x82 \x84\x90U\x85\x82R\x81 U`\t\x80T\x80a-\xF7Wa-\xF7a7$V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90UPPPPV[`\0a.\x1E\x83a\x14\rV[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x85\x90U\x93\x82R`\x08\x90R\x91\x90\x91 \x91\x90\x91UPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x19\xE7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a.\x97W`\0\x80\xFD[\x815a.\xA2\x81a.WV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a.\xBBW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a.\xE8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a.\xCCV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a.\xA2` \x83\x01\x84a.\xC2V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a/2W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/JW`\0\x80\xFD[a/S\x83a/\x1BV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12a/sW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x8BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a/\xA3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a/\xC2W`\0\x80\xFD[a/\xCB\x86a/\x1BV[\x94Pa/\xD9` \x87\x01a/\x1BV[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xFCW`\0\x80\xFD[a0\x08\x88\x82\x89\x01a/aV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a0cWa0ca0\x19V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a0\x8BWa0\x8Ba0\x19V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a0\xA4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0\xD4W`\0\x80\xFD[\x845\x93Pa0\xE4` \x86\x01a/\x1BV[\x92P`@\x85\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a0\xFDW`\0\x80\xFD[\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x19W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a1*W`\0\x80\xFD[a19\x87\x825` \x84\x01a0HV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1ZW`\0\x80\xFD[a1c\x84a/\x1BV[\x92Pa1q` \x85\x01a/\x1BV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[` \x81R\x81Q` \x82\x01R`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R``\x82\x01Q\x15\x15`\x80\x82\x01R`\0`\x80\x83\x01Q`\xC0`\xA0\x84\x01Ra1\xD6`\xE0\x84\x01\x82a.\xC2V[\x90P`\xA0\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xC0\x85\x01Ra1\xF3\x82\x82a.\xC2V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a2\x0EW`\0\x80\xFD[a.\xA2\x82a/\x1BV[`\0\x80`\0`@\x84\x86\x03\x12\x15a2,W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2JW`\0\x80\xFD[a2V\x86\x82\x87\x01a/aV[\x94\x97\x90\x96P\x93\x94PPPPV[\x80\x15\x15\x81\x14a\x19\xE7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a2\x84W`\0\x80\xFD[a2\x8D\x83a/\x1BV[\x91P` \x83\x015a2\x9D\x81a2cV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2\xBEW`\0\x80\xFD[a2\xC7\x85a/\x1BV[\x93Pa2\xD5` \x86\x01a/\x1BV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x19W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a3\rW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3$W`\0\x80\xFD[a30\x86\x82\x87\x01a/aV[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a3WW`\0\x80\xFD[a3`\x83a/\x1BV[\x91Pa3n` \x84\x01a/\x1BV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a3\x8BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a3\xC4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a3\xDCW`\0\x80\xFD[\x81Qa.\xA2\x81a2cV[`\x1F\x82\x11\x15a\t\xA1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a4\x0EWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x13\xA0W\x82\x81U`\x01\x01a4\x1AV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4GWa4Ga0\x19V[a4[\x81a4U\x84Ta3wV[\x84a3\xE7V[` \x80`\x1F\x83\x11`\x01\x81\x14a4\x90W`\0\x84\x15a4xWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x13\xA0V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a4\xBFW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a4\xA0V[P\x85\x82\x10\x15a4\xDDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07;Wa\x07;a4\xEDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a5vWa5va0\x19V[a5\x8A\x83a5\x84\x83Ta3wV[\x83a3\xE7V[`\0`\x1F\x84\x11`\x01\x81\x14a5\xBEW`\0\x85\x15a5\xA6WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua*\x9BV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a5\xEFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a5\xCFV[P\x86\x82\x10\x15a6\x0CW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15a6_W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07;Wa\x07;a4\xEDV[`\0\x82a6\xB3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x07;Wa\x07;a4\xEDV[`\0`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP\x83`@\x83\x01R`\x80``\x83\x01Ra6\xFD`\x80\x83\x01\x84a.\xC2V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a7\x19W`\0\x80\xFD[\x81Qa.\xA2\x81a.WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD";
    /// The bytecode of the contract.
    pub static RESTURANTTOKEN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x024W`\x005`\xE0\x1C\x80cYj\xC5\xBD\x11a\x018W\x80c\x95\xD8\x9BA\x11a\0\xB0W\x80c\xC5s\x80\xA2\x11a\0\x7FW\x80c\xC9\xE7\xF4\xB7\x11a\0dW\x80c\xC9\xE7\xF4\xB7\x14a\x06\xA7W\x80c\xE9\x85\xE9\xC5\x14a\x06\xC7W\x80c\xF2\xFD\xE3\x8B\x14a\x07\x10W`\0\x80\xFD[\x80c\xC5s\x80\xA2\x14a\x06iW\x80c\xC8{V\xDD\x14a\x06\x87W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x14a\x05\xC5W\x80c\xA2,\xB4e\x14a\x05\xDAW\x80c\xB4Z<\x0E\x14a\x05\xFAW\x80c\xB8\x8DO\xDE\x14a\x06IW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\x01\x07W\x80c\x85\xB3\xA7\xDB\x11a\0\xECW\x80c\x85\xB3\xA7\xDB\x14a\x05mW\x80c\x8A\xDA\x06n\x14a\x05\x8DW\x80c\x8D\xA5\xCB[\x14a\x05\xA2W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x05CW\x80c\x84V\xCBY\x14a\x05XW`\0\x80\xFD[\x80cYj\xC5\xBD\x14a\x04\xD4W\x80c\\\x97Z\xBB\x14a\x04\xEBW\x80ccR!\x1E\x14a\x05\x03W\x80cp\xA0\x821\x14a\x05#W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\xCBW\x80cA\x98\x1A\xB1\x11a\x01\x9AW\x80cB\x84.\x0E\x11a\x01\x7FW\x80cB\x84.\x0E\x14a\x04\x81W\x80cOl\xCC\xE7\x14a\x04\xA1W\x80cQ\xED\x82\x88\x14a\x04\xC1W`\0\x80\xFD[\x80cA\x98\x1A\xB1\x14a\x044W\x80cA\xF6;\xFD\x14a\x04TW`\0\x80\xFD[\x80c#\xB8r\xDD\x14a\x03\xBFW\x80c'\x8E\xCD\xE1\x14a\x03\xDFW\x80c/t\\Y\x14a\x03\xFFW\x80c?K\xA8:\x14a\x04\x1FW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x02\x07W\x80c\t^\xA7\xB3\x14a\x02\xEAW\x80c\x15\x0Bz\x02\x14a\x03\nW\x80c\x18\x16\r\xDD\x14a\x03\x80W\x80c\x1A4\x1D\xDC\x14a\x03\x9FW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x029W\x80c\x05\x1FWX\x14a\x02nW\x80c\x06\xFD\xDE\x03\x14a\x02\x90W\x80c\x08\x18\x12\xFC\x14a\x02\xB2W[`\0\x80\xFD[4\x80\x15a\x02EW`\0\x80\xFD[Pa\x02Ya\x02T6`\x04a.\x85V[a\x070V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02zW`\0\x80\xFD[Pa\x02\x8Ea\x02\x896`\x04a.\xA9V[a\x07AV[\0[4\x80\x15a\x02\x9CW`\0\x80\xFD[Pa\x02\xA5a\x07\xB7V[`@Qa\x02e\x91\x90a/\x08V[4\x80\x15a\x02\xBEW`\0\x80\xFD[Pa\x02\xD2a\x02\xCD6`\x04a.\xA9V[a\x08IV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02eV[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x02\x8Ea\x03\x056`\x04a/7V[a\x08pV[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x03Oa\x03%6`\x04a/\xAAV[\x7F\x15\x0Bz\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x02eV[4\x80\x15a\x03\x8CW`\0\x80\xFD[P`\tT[`@Q\x90\x81R` \x01a\x02eV[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x02\x8Ea\x03\xBA6`\x04a0\xBEV[a\t\xA6V[4\x80\x15a\x03\xCBW`\0\x80\xFD[Pa\x02\x8Ea\x03\xDA6`\x04a1EV[a\x0B\xC4V[4\x80\x15a\x03\xEBW`\0\x80\xFD[Pa\x02\x8Ea\x03\xFA6`\x04a.\xA9V[a\x0CKV[4\x80\x15a\x04\x0BW`\0\x80\xFD[Pa\x03\x91a\x04\x1A6`\x04a/7V[a\r\xA3V[4\x80\x15a\x04+W`\0\x80\xFD[Pa\x02\x8Ea\x0EKV[4\x80\x15a\x04@W`\0\x80\xFD[Pa\x02\x8Ea\x04O6`\x04a.\xA9V[a\x0E\x99V[4\x80\x15a\x04`W`\0\x80\xFD[Pa\x04ta\x04o6`\x04a.\xA9V[a\x0F\x95V[`@Qa\x02e\x91\x90a1\x81V[4\x80\x15a\x04\x8DW`\0\x80\xFD[Pa\x02\x8Ea\x04\x9C6`\x04a1EV[a\x11\x8FV[4\x80\x15a\x04\xADW`\0\x80\xFD[Pa\x03\x91a\x04\xBC6`\x04a.\xA9V[a\x11\xAAV[a\x02\x8Ea\x04\xCF6`\x04a.\xA9V[a\x12NV[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x03\x91b\x01Q\x80\x81V[4\x80\x15a\x04\xF7W`\0\x80\xFD[P`\x06T`\xFF\x16a\x02YV[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x02\xD2a\x05\x1E6`\x04a.\xA9V[a\x13\xA8V[4\x80\x15a\x05/W`\0\x80\xFD[Pa\x03\x91a\x05>6`\x04a1\xFCV[a\x14\rV[4\x80\x15a\x05OW`\0\x80\xFD[Pa\x02\x8Ea\x14\xA7V[4\x80\x15a\x05dW`\0\x80\xFD[Pa\x02\x8Ea\x14\xB9V[4\x80\x15a\x05yW`\0\x80\xFD[Pa\x02\x8Ea\x05\x886`\x04a2\x17V[a\x15\x05V[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x03\x91a\x16\x1EV[4\x80\x15a\x05\xAEW`\0\x80\xFD[P`\x06Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02\xD2V[4\x80\x15a\x05\xD1W`\0\x80\xFD[Pa\x02\xA5a\x16.V[4\x80\x15a\x05\xE6W`\0\x80\xFD[Pa\x02\x8Ea\x05\xF56`\x04a2qV[a\x16=V[4\x80\x15a\x06\x06W`\0\x80\xFD[Pa\x02Ya\x06\x156`\x04a.\xA9V[`\0\x90\x81R`\x0C` R`@\x90 `\x01\x01Tx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x90V[4\x80\x15a\x06UW`\0\x80\xFD[Pa\x02\x8Ea\x06d6`\x04a2\xA8V[a\x16HV[4\x80\x15a\x06uW`\0\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16a\x02\xD2V[4\x80\x15a\x06\x93W`\0\x80\xFD[Pa\x02\xA5a\x06\xA26`\x04a.\xA9V[a\x16\xD0V[4\x80\x15a\x06\xB3W`\0\x80\xFD[Pa\x02\x8Ea\x06\xC26`\x04a2\xF8V[a\x17uV[4\x80\x15a\x06\xD3W`\0\x80\xFD[Pa\x02Ya\x06\xE26`\x04a3DV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[4\x80\x15a\x07\x1CW`\0\x80\xFD[Pa\x02\x8Ea\x07+6`\x04a1\xFCV[a\x19ZV[`\0a\x07;\x82a\x19\xEAV[\x92\x91PPV[a\x07Ia\x1A@V[\x800a\x07j\x82`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xAAW`@Q\x7F\x87[\xC7j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xB3\x82a\x1A\xA0V[PPV[```\0\x80Ta\x07\xC6\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xF2\x90a3wV[\x80\x15a\x08?W\x80`\x1F\x10a\x08\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x08T\x82a\x1B[V[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x08{\x82a\x13\xA8V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\t%WPa\t%\x813a\x06\xE2V[a\t\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\t\xA1\x83\x83a\x1B\xBFV[PPPV[a\t\xAEa\x1A@V[`\rT`@Q\x7FZ\xE9\xCEu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x87\x90R\x90\x91\x16\x90cZ\xE9\xCEu\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n>\x91\x90a3\xCAV[a\ntW`@Q\x7F\x8B\x8DTM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\n\x7F`\x0BT\x90V[\x90Pa\n\x8F`\x0B\x80T`\x01\x01\x90UV[a\n\x990\x82a\x1CEV[`@\x80Q`\xC0\x81\x01\x82R\x86\x81R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16` \x80\x84\x01\x91\x82Rc\xFF\xFF\xFF\xFF\x80\x89\x16\x85\x87\x01\x90\x81R`\0``\x87\x01\x81\x81R\x88Q\x80\x86\x01\x8AR\x82\x81R`\x80\x89\x01\x90\x81R`\xA0\x89\x01\x8C\x90R\x8A\x83R`\x0C\x90\x95R\x97\x90 \x86Q\x81U\x93Q`\x01\x85\x01\x80T\x92Q\x98Q\x15\x15x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x99\x90\x94\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x91\x90\x96\x16\x17\x17\x95\x90\x95\x16\x94\x90\x94\x17\x90\x91U\x91Q\x90\x91\x90`\x02\x82\x01\x90a\x0B\xA5\x90\x82a4-V[P`\xA0\x82\x01Q`\x03\x82\x01\x90a\x0B\xBA\x90\x82a4-V[PPPPPPPPV[a\x0B\xCE3\x82a\x1C_V[a\x0C@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC721: caller is not token owne`D\x82\x01R\x7Fr or approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\t\xA1\x83\x83\x83a\x1C\xDEV[a\x0CSa\x1A@V[`\0\x81\x81R`\x02` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x0C\x8FWP`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x14[\x15a\x0C\xC6W`@Q\x7F1V\xEF\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80c\xA9\x05\x9C\xBBa\r\n\x86`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\roW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x93\x91\x90a3\xCAV[Pa\r\x9D\x84a\x1A\xA0V[PPPPV[`\0a\r\xAE\x83a\x14\rV[\x82\x10a\x0E\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FERC721Enumerable: owner index ou`D\x82\x01R\x7Ft of bounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x8FW`@Q\x7F\xD2U\xF1-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x97a\x1F,V[V[a\x0E\xA1a\x1A@V[`\0\x81\x81R`\x02` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x0E\xDDWP`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x14[\x15a\x0F\x14W`@Q\x7F1V\xEF\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x90 `\x01\x01TB\x90a\x0FT\x90b\x01Q\x80\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a5\x1CV[\x11\x15a\x0F\x8CW`@Q\x7F<\x84\xFA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xAA\x82a\x1F\x9CV[a\x0F\xDF`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x15\x15\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81R`\x0C` \x90\x81R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x82\x01\x93\x90\x93Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x04c\xFF\xFF\xFF\xFF\x16\x93\x81\x01\x93\x90\x93Rx\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15``\x83\x01R`\x02\x81\x01\x80T`\x80\x84\x01\x91\x90a\x10t\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xA0\x90a3wV[\x80\x15a\x10\xEDW\x80`\x1F\x10a\x10\xC2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xEDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x11\x06\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x112\x90a3wV[\x80\x15a\x11\x7FW\x80`\x1F\x10a\x11TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\x7FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11bW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[a\t\xA1\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x16HV[`\0a\x11\xB5`\tT\x90V[\x82\x10a\x12)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FERC721Enumerable: global index o`D\x82\x01R\x7Fut of bounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\t\x82\x81T\x81\x10a\x12<Wa\x12<a5/V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[\x800a\x12o\x82`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xAFW`@Q\x7F\x87[\xC7j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x80\x82 \x80T`\x01\x90\x91\x01T\x91Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R\x90\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x82\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x137W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13[\x91\x90a3\xCAV[\x90P\x80a\x13\x94W`@Q\x7F\xFC\xED70\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x13\xA00\x82\x88a\x1C\xDEV[PPPPPPV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x07;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FERC721: invalid token ID\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01R\x7Flid owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x14\xAFa\x1A@V[a\x0E\x97`\0a!\xF7V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xFDW`@Q\x7F\xD2U\xF1-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\x97a\"hV[`\0\x83\x81R`\x0C` R`@\x81 `\x01\x01T\x84\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15\x90\x03a\x15sW`@Q\x7F\xD2U9\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x833\x80a\x15\x7F\x83a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xBFW`@Q\x7F\xC9\xE1M\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x81R`\x0C` R`@\x90 `\x02\x01a\x15\xDB\x85\x87\x83a5^V[P\x85\x7F\xE477#\x90\x19G\xF3\xE6|\x8F\xE5\xCCA\xB9\x18dmm!EV\xE2\x93\xED\xCD\xB4\x83\xB0Q\x87\xA2\x86\x86`@Qa\x16\x0E\x92\x91\x90a6\x1EV[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0a\x16)`\x0BT\x90V[\x90P\x90V[```\x01\x80Ta\x07\xC6\x90a3wV[a\x07\xB33\x83\x83a\"\xC3V[a\x16R3\x83a\x1C_V[a\x16\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC721: caller is not token owne`D\x82\x01R\x7Fr or approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\r\x9D\x84\x84\x84\x84a#\xAFV[`\0\x81\x81R`\x0C` R`@\x90 `\x03\x01\x80T``\x91\x90a\x16\xF0\x90a3wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\x1C\x90a3wV[\x80\x15a\x17iW\x80`\x1F\x10a\x17>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17iV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17LW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[a\x17}a\x1A@V[`\0\x81\x81R`\x02` R`@\x90 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x17\xB9WP`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x160\x14[\x15a\x17\xF0W`@Q\x7F1V\xEF\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x0C` R`@\x90 `\x01\x90\x81\x01T\x83\x91x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15\x90\x03a\x18`W`@Q\x7F\x03\x8E\x9B7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0a\x18\xBE\x82\x860\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa$8\x92PPPV[\x90P\x80a\x18\xF7W`@Q\x7F>\\\x81\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\0\x85a\x1F\x9CV[PPP`\0\x91\x82RP`\x0C` R`@\x90 `\x01\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UPPV[a\x19ba\x1A@V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\x19\xE7\x81a!\xF7V[PV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7Fx\x0E\x9Dc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x07;WPa\x07;\x82a$\xCBV[`\x06T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x163\x14a\x0E\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\0V[`\0a\x1A\xAB\x82a\x13\xA8V[\x90Pa\x1A\xBB\x81`\0\x84`\x01a%\xAEV[a\x1A\xC4\x82a\x13\xA8V[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R`\x03\x84R\x82\x85 \x80T`\0\x19\x01\x90U\x87\x85R`\x02\x90\x93R\x81\x84 \x80T\x90\x91\x16\x90UQ\x92\x93P\x84\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x19\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FERC721: invalid token ID\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0\x81\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x1C\x0C\x82a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x07\xB3\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa%\xBAV[`\0\x80a\x1Ck\x83a\x13\xA8V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x1C\xB2WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x1C\xD6WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xCB\x84a\x08IV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xF1\x82a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1DmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC721: transfer from incorrect `D\x82\x01R\x7Fowner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01R\x7Fress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[a\x1D\xF5\x83\x83\x83`\x01a%\xAEV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x1E\x08\x82a\x13\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC721: transfer from incorrect `D\x82\x01R\x7Fowner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4PPPV[a\x1F4a&CV[`\x06\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\0\x81\x81R`\x0C` R`@\x80\x82 \x80T`\x01\x90\x91\x01T`\rT\x92Q\x7FOE\x98\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01\x81\x90R\x92\x94\x92\x93\x91\x90\x91\x16\x90cOE\x98\x94\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a @\x91\x90a6MV[\x90Pa'\x10\x81\x10a \x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Ffee can't be more than 100%\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0a'\x10a \xA2\x83\x86a6fV[a \xAC\x91\x90a6}V[`\rT`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x91\x92P\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!>\x91\x90a3\xCAV[P\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa!f`\x06T`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x16\x90V[a!p\x84\x88a6\xB8V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA0\x91\x90a3\xCAV[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16a\x01\0\x81\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\x85\x16\x17\x90\x94U`@Q\x93\x90\x92\x04\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\"pa&\x95V[`\x06\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x1F\x7F3\x90V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a#$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a#\xBA\x84\x84\x84a\x1C\xDEV[a#\xC6\x84\x84\x84\x84a&\xE8V[a\r\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01R\x7Fceiver implementer\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\0\x80a$E\x85\x85a(\x86V[\x90P`\0a$\xA0\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P\x86`\x01`\x01`\xA0\x1B\x03\x16a$\xB6\x82\x86a(\xE7V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x97\x96PPPPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x80\xACX\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a%^WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F[^\x13\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a\x07;WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14a\x07;V[a\r\x9D\x84\x84\x84\x84a)fV[a%\xC4\x83\x83a*\xA2V[a%\xD1`\0\x84\x84\x84a&\xE8V[a\t\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01R\x7Fceiver implementer\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[`\x06T`\xFF\x16a\x0E\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FPausable: not paused\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\x06T`\xFF\x16\x15a\x0E\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPausable: paused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a(~W`@Q\x7F\x15\x0Bz\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a'E\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a6\xCBV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a'\x80WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra'}\x91\x81\x01\x90a7\x07V[`\x01[a(3W=\x80\x80\x15a'\xAEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\xB3V[``\x91P[P\x80Q`\0\x03a(+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01R\x7Fceiver implementer\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[\x80Q\x81` \x01\xFD[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\x15\x0Bz\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x90Pa\x1C\xD6V[P`\x01a\x1C\xD6V[`\0\x82\x82`@Q` \x01a(\xC9\x92\x91\x90\x91\x82R``\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0\x80a(\xF6\x85a,SV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x8B\x90R`\xFF\x83\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R\x92\x95P\x90\x93P\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a)QW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x97\x96PPPPPPPV[`\x01\x81\x11\x15a)\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FERC721Enumerable: consecutive tr`D\x82\x01R\x7Fansfers not supported\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\0V[\x81`\x01`\x01`\xA0\x1B\x03\x85\x16a*9Wa*4\x81`\t\x80T`\0\x83\x81R`\n` R`@\x81 \x82\x90U`\x01\x82\x01\x83U\x91\x90\x91R\x7Fn\x15@\x17\x1Bl\x0C\x96\x0Bq\xA7\x02\r\x9F`\x07\x7Fj\xF91\xA8\xBB\xF5\x90\xDA\x02#\xDA\xCFu\xC7\xAF\x01UV[a*\\V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14a*\\Wa*\\\x85\x82a,\xC7V[`\x01`\x01`\xA0\x1B\x03\x84\x16a*xWa*s\x81a-dV[a*\x9BV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14a*\x9BWa*\x9B\x84\x82a.\x13V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a*\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\t\0V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a+]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\t\0V[a+k`\0\x83\x83`\x01a%\xAEV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a+\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\t\0V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\0\x80`\0\x83Q`A\x14a,\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Finvalid signature length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\0V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q\x90\x92`\0\x91\x90\x91\x1A\x90V[`\0`\x01a,\xD4\x84a\x14\rV[a,\xDE\x91\x90a6\xB8V[`\0\x83\x81R`\x08` R`@\x90 T\x90\x91P\x80\x82\x14a-1W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 T\x84\x84R\x81\x84 \x81\x90U\x83R`\x08\x90\x91R\x90 \x81\x90U[P`\0\x91\x82R`\x08` \x90\x81R`@\x80\x84 \x84\x90U`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R`\x07\x81R\x83\x83 \x91\x83RR\x90\x81 UV[`\tT`\0\x90a-v\x90`\x01\x90a6\xB8V[`\0\x83\x81R`\n` R`@\x81 T`\t\x80T\x93\x94P\x90\x92\x84\x90\x81\x10a-\x9EWa-\x9Ea5/V[\x90`\0R` `\0 \x01T\x90P\x80`\t\x83\x81T\x81\x10a-\xBFWa-\xBFa5/V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\n\x90\x91R`@\x80\x82 \x84\x90U\x85\x82R\x81 U`\t\x80T\x80a-\xF7Wa-\xF7a7$V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90UPPPPV[`\0a.\x1E\x83a\x14\rV[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x85\x90U\x93\x82R`\x08\x90R\x91\x90\x91 \x91\x90\x91UPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x19\xE7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a.\x97W`\0\x80\xFD[\x815a.\xA2\x81a.WV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a.\xBBW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a.\xE8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a.\xCCV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a.\xA2` \x83\x01\x84a.\xC2V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a/2W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a/JW`\0\x80\xFD[a/S\x83a/\x1BV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12a/sW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x8BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a/\xA3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a/\xC2W`\0\x80\xFD[a/\xCB\x86a/\x1BV[\x94Pa/\xD9` \x87\x01a/\x1BV[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xFCW`\0\x80\xFD[a0\x08\x88\x82\x89\x01a/aV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a0cWa0ca0\x19V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a0\x8BWa0\x8Ba0\x19V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a0\xA4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0\xD4W`\0\x80\xFD[\x845\x93Pa0\xE4` \x86\x01a/\x1BV[\x92P`@\x85\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a0\xFDW`\0\x80\xFD[\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x19W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a1*W`\0\x80\xFD[a19\x87\x825` \x84\x01a0HV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1ZW`\0\x80\xFD[a1c\x84a/\x1BV[\x92Pa1q` \x85\x01a/\x1BV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[` \x81R\x81Q` \x82\x01R`\x01`\x01`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R``\x82\x01Q\x15\x15`\x80\x82\x01R`\0`\x80\x83\x01Q`\xC0`\xA0\x84\x01Ra1\xD6`\xE0\x84\x01\x82a.\xC2V[\x90P`\xA0\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xC0\x85\x01Ra1\xF3\x82\x82a.\xC2V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a2\x0EW`\0\x80\xFD[a.\xA2\x82a/\x1BV[`\0\x80`\0`@\x84\x86\x03\x12\x15a2,W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2JW`\0\x80\xFD[a2V\x86\x82\x87\x01a/aV[\x94\x97\x90\x96P\x93\x94PPPPV[\x80\x15\x15\x81\x14a\x19\xE7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a2\x84W`\0\x80\xFD[a2\x8D\x83a/\x1BV[\x91P` \x83\x015a2\x9D\x81a2cV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2\xBEW`\0\x80\xFD[a2\xC7\x85a/\x1BV[\x93Pa2\xD5` \x86\x01a/\x1BV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x19W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a3\rW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3$W`\0\x80\xFD[a30\x86\x82\x87\x01a/aV[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a3WW`\0\x80\xFD[a3`\x83a/\x1BV[\x91Pa3n` \x84\x01a/\x1BV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a3\x8BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a3\xC4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a3\xDCW`\0\x80\xFD[\x81Qa.\xA2\x81a2cV[`\x1F\x82\x11\x15a\t\xA1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a4\x0EWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x13\xA0W\x82\x81U`\x01\x01a4\x1AV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4GWa4Ga0\x19V[a4[\x81a4U\x84Ta3wV[\x84a3\xE7V[` \x80`\x1F\x83\x11`\x01\x81\x14a4\x90W`\0\x84\x15a4xWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x13\xA0V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a4\xBFW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a4\xA0V[P\x85\x82\x10\x15a4\xDDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07;Wa\x07;a4\xEDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a5vWa5va0\x19V[a5\x8A\x83a5\x84\x83Ta3wV[\x83a3\xE7V[`\0`\x1F\x84\x11`\x01\x81\x14a5\xBEW`\0\x85\x15a5\xA6WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua*\x9BV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a5\xEFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a5\xCFV[P\x86\x82\x10\x15a6\x0CW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15a6_W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07;Wa\x07;a4\xEDV[`\0\x82a6\xB3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x07;Wa\x07;a4\xEDV[`\0`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x83R\x80\x86\x16` \x84\x01RP\x83`@\x83\x01R`\x80``\x83\x01Ra6\xFD`\x80\x83\x01\x84a.\xC2V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a7\x19W`\0\x80\xFD[\x81Qa.\xA2\x81a.WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`1`\x04R`$`\0\xFD";
    /// The deployed bytecode of the contract.
    pub static RESTURANTTOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ResturantToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ResturantToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ResturantToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ResturantToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ResturantToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ResturantToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ResturantToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESTURANTTOKEN_ABI.clone(),
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
                RESTURANTTOKEN_ABI.clone(),
                RESTURANTTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `EXPIRATION_RANGE` (0x596ac5bd) function
        pub fn expiration_range(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 106, 197, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnNftNotSold` (0x051f5758) function
        pub fn burn_nft_not_sold(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 31, 87, 88], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnTicketNotConsumed` (0x41981ab1) function
        pub fn burn_ticket_not_consumed(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 152, 26, 177], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `buyNFT` (0x51ed8288) function
        pub fn buy_nft(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 237, 130, 136], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getControllerAddress` (0xc57380a2) function
        pub fn get_controller_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 115, 128, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCounter` (0x8ada066e) function
        pub fn get_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([138, 218, 6, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNft` (0x41f63bfd) function
        pub fn get_nft(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Nft> {
            self.0
                .method_hash([65, 246, 59, 253], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xb45a3c0e) function
        pub fn locked(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([180, 90, 60, 14], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refund` (0x278ecde1) function
        pub fn refund(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 142, 205, 225], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeMint` (0x1a341ddc) function
        pub fn safe_mint(
            &self,
            price: ::ethers::core::types::U256,
            payment_token: ::ethers::core::types::Address,
            reservation_date: u32,
            uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [26, 52, 29, 220],
                    (price, payment_token, reservation_date, uri),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendReview` (0x85b3a7db) function
        pub fn send_review(
            &self,
            token_id: ::ethers::core::types::U256,
            uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 179, 167, 219], (token_id, uri))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenByIndex` (0x4f6ccce7) function
        pub fn token_by_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function
        pub fn token_of_owner_by_index(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `useTicket` (0xc9e7f4b7) function
        pub fn use_ticket(
            &self,
            signature: ::ethers::core::types::Bytes,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 231, 244, 183], (signature, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Locked` event
        pub fn locked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockedFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `ReviewPosted` event
        pub fn review_posted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReviewPostedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unlocked` event
        pub fn unlocked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnlockedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ResturantTokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ResturantToken<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ResturantToken__InvalidSignatureFromClient` with signature `ResturantToken__InvalidSignatureFromClient()` and selector `0x3e5c818f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__InvalidSignatureFromClient",
        abi = "ResturantToken__InvalidSignatureFromClient()"
    )]
    pub struct ResturantToken__InvalidSignatureFromClient;
    ///Custom Error type `ResturantToken__PriceNotAcceptable` with signature `ResturantToken__PriceNotAcceptable()` and selector `0x8b8d544d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__PriceNotAcceptable",
        abi = "ResturantToken__PriceNotAcceptable()"
    )]
    pub struct ResturantToken__PriceNotAcceptable;
    ///Custom Error type `ResturantToken__SenderIsNotNftOwner` with signature `ResturantToken__SenderIsNotNftOwner()` and selector `0xc9e14d9e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__SenderIsNotNftOwner",
        abi = "ResturantToken__SenderIsNotNftOwner()"
    )]
    pub struct ResturantToken__SenderIsNotNftOwner;
    ///Custom Error type `ResturantToken__SenderIsNotTheController` with signature `ResturantToken__SenderIsNotTheController()` and selector `0xd255f12d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__SenderIsNotTheController",
        abi = "ResturantToken__SenderIsNotTheController()"
    )]
    pub struct ResturantToken__SenderIsNotTheController;
    ///Custom Error type `ResturantToken__TokenAlreadyUsed` with signature `ResturantToken__TokenAlreadyUsed()` and selector `0x038e9b37`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__TokenAlreadyUsed",
        abi = "ResturantToken__TokenAlreadyUsed()"
    )]
    pub struct ResturantToken__TokenAlreadyUsed;
    ///Custom Error type `ResturantToken__TokenNotForSale` with signature `ResturantToken__TokenNotForSale()` and selector `0x875bc76a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__TokenNotForSale",
        abi = "ResturantToken__TokenNotForSale()"
    )]
    pub struct ResturantToken__TokenNotForSale;
    ///Custom Error type `ResturantToken__TokenNotSold` with signature `ResturantToken__TokenNotSold()` and selector `0x3156efb6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__TokenNotSold",
        abi = "ResturantToken__TokenNotSold()"
    )]
    pub struct ResturantToken__TokenNotSold;
    ///Custom Error type `ResturantToken__TokenNotUsed` with signature `ResturantToken__TokenNotUsed()` and selector `0xd25539c1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__TokenNotUsed",
        abi = "ResturantToken__TokenNotUsed()"
    )]
    pub struct ResturantToken__TokenNotUsed;
    ///Custom Error type `ResturantToken__TokenNotYetBurnable` with signature `ResturantToken__TokenNotYetBurnable()` and selector `0x3c84fa7f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__TokenNotYetBurnable",
        abi = "ResturantToken__TokenNotYetBurnable()"
    )]
    pub struct ResturantToken__TokenNotYetBurnable;
    ///Custom Error type `ResturantToken__TransferFailed` with signature `ResturantToken__TransferFailed()` and selector `0xfced3730`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ResturantToken__TransferFailed",
        abi = "ResturantToken__TransferFailed()"
    )]
    pub struct ResturantToken__TransferFailed;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResturantTokenErrors {
        ResturantToken__InvalidSignatureFromClient(
            ResturantToken__InvalidSignatureFromClient,
        ),
        ResturantToken__PriceNotAcceptable(ResturantToken__PriceNotAcceptable),
        ResturantToken__SenderIsNotNftOwner(ResturantToken__SenderIsNotNftOwner),
        ResturantToken__SenderIsNotTheController(
            ResturantToken__SenderIsNotTheController,
        ),
        ResturantToken__TokenAlreadyUsed(ResturantToken__TokenAlreadyUsed),
        ResturantToken__TokenNotForSale(ResturantToken__TokenNotForSale),
        ResturantToken__TokenNotSold(ResturantToken__TokenNotSold),
        ResturantToken__TokenNotUsed(ResturantToken__TokenNotUsed),
        ResturantToken__TokenNotYetBurnable(ResturantToken__TokenNotYetBurnable),
        ResturantToken__TransferFailed(ResturantToken__TransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ResturantTokenErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__InvalidSignatureFromClient as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__InvalidSignatureFromClient(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__PriceNotAcceptable as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__PriceNotAcceptable(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__SenderIsNotNftOwner as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__SenderIsNotNftOwner(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__SenderIsNotTheController as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__SenderIsNotTheController(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__TokenAlreadyUsed as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__TokenAlreadyUsed(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__TokenNotForSale as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__TokenNotForSale(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__TokenNotSold as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__TokenNotSold(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__TokenNotUsed as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__TokenNotUsed(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__TokenNotYetBurnable as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__TokenNotYetBurnable(decoded));
            }
            if let Ok(decoded)
                = <ResturantToken__TransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResturantToken__TransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ResturantTokenErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ResturantToken__InvalidSignatureFromClient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__PriceNotAcceptable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__SenderIsNotNftOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__SenderIsNotTheController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__TokenAlreadyUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__TokenNotForSale(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__TokenNotSold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__TokenNotUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__TokenNotYetBurnable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResturantToken__TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ResturantTokenErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ResturantToken__InvalidSignatureFromClient as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__PriceNotAcceptable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__SenderIsNotNftOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__SenderIsNotTheController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__TokenAlreadyUsed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__TokenNotForSale as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__TokenNotSold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__TokenNotUsed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__TokenNotYetBurnable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ResturantToken__TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ResturantTokenErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ResturantToken__InvalidSignatureFromClient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__PriceNotAcceptable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__SenderIsNotNftOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__SenderIsNotTheController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__TokenAlreadyUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__TokenNotForSale(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__TokenNotSold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__TokenNotUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__TokenNotYetBurnable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResturantToken__TransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ResturantTokenErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ResturantToken__InvalidSignatureFromClient>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__InvalidSignatureFromClient) -> Self {
            Self::ResturantToken__InvalidSignatureFromClient(value)
        }
    }
    impl ::core::convert::From<ResturantToken__PriceNotAcceptable>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__PriceNotAcceptable) -> Self {
            Self::ResturantToken__PriceNotAcceptable(value)
        }
    }
    impl ::core::convert::From<ResturantToken__SenderIsNotNftOwner>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__SenderIsNotNftOwner) -> Self {
            Self::ResturantToken__SenderIsNotNftOwner(value)
        }
    }
    impl ::core::convert::From<ResturantToken__SenderIsNotTheController>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__SenderIsNotTheController) -> Self {
            Self::ResturantToken__SenderIsNotTheController(value)
        }
    }
    impl ::core::convert::From<ResturantToken__TokenAlreadyUsed>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__TokenAlreadyUsed) -> Self {
            Self::ResturantToken__TokenAlreadyUsed(value)
        }
    }
    impl ::core::convert::From<ResturantToken__TokenNotForSale>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__TokenNotForSale) -> Self {
            Self::ResturantToken__TokenNotForSale(value)
        }
    }
    impl ::core::convert::From<ResturantToken__TokenNotSold> for ResturantTokenErrors {
        fn from(value: ResturantToken__TokenNotSold) -> Self {
            Self::ResturantToken__TokenNotSold(value)
        }
    }
    impl ::core::convert::From<ResturantToken__TokenNotUsed> for ResturantTokenErrors {
        fn from(value: ResturantToken__TokenNotUsed) -> Self {
            Self::ResturantToken__TokenNotUsed(value)
        }
    }
    impl ::core::convert::From<ResturantToken__TokenNotYetBurnable>
    for ResturantTokenErrors {
        fn from(value: ResturantToken__TokenNotYetBurnable) -> Self {
            Self::ResturantToken__TokenNotYetBurnable(value)
        }
    }
    impl ::core::convert::From<ResturantToken__TransferFailed> for ResturantTokenErrors {
        fn from(value: ResturantToken__TransferFailed) -> Self {
            Self::ResturantToken__TransferFailed(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "Locked", abi = "Locked(uint256)")]
    pub struct LockedFilter {
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "ReviewPosted", abi = "ReviewPosted(uint256,string)")]
    pub struct ReviewPostedFilter {
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
        pub review_uri: ::std::string::String,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Unlocked", abi = "Unlocked(uint256)")]
    pub struct UnlockedFilter {
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResturantTokenEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        LockedFilter(LockedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        ReviewPostedFilter(ReviewPostedFilter),
        TransferFilter(TransferFilter),
        UnlockedFilter(UnlockedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ResturantTokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = LockedFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::LockedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = ReviewPostedFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::ReviewPostedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnlockedFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::UnlockedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ResturantTokenEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ResturantTokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReviewPostedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ResturantTokenEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ResturantTokenEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<LockedFilter> for ResturantTokenEvents {
        fn from(value: LockedFilter) -> Self {
            Self::LockedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ResturantTokenEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for ResturantTokenEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<ReviewPostedFilter> for ResturantTokenEvents {
        fn from(value: ReviewPostedFilter) -> Self {
            Self::ReviewPostedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ResturantTokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UnlockedFilter> for ResturantTokenEvents {
        fn from(value: UnlockedFilter) -> Self {
            Self::UnlockedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for ResturantTokenEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `EXPIRATION_RANGE` function with signature `EXPIRATION_RANGE()` and selector `0x596ac5bd`
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
    #[ethcall(name = "EXPIRATION_RANGE", abi = "EXPIRATION_RANGE()")]
    pub struct ExpirationRangeCall;
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burnNftNotSold` function with signature `burnNftNotSold(uint256)` and selector `0x051f5758`
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
    #[ethcall(name = "burnNftNotSold", abi = "burnNftNotSold(uint256)")]
    pub struct BurnNftNotSoldCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `burnTicketNotConsumed` function with signature `burnTicketNotConsumed(uint256)` and selector `0x41981ab1`
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
    #[ethcall(name = "burnTicketNotConsumed", abi = "burnTicketNotConsumed(uint256)")]
    pub struct BurnTicketNotConsumedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `buyNFT` function with signature `buyNFT(uint256)` and selector `0x51ed8288`
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
    #[ethcall(name = "buyNFT", abi = "buyNFT(uint256)")]
    pub struct BuyNFTCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getControllerAddress` function with signature `getControllerAddress()` and selector `0xc57380a2`
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
    #[ethcall(name = "getControllerAddress", abi = "getControllerAddress()")]
    pub struct GetControllerAddressCall;
    ///Container type for all input parameters for the `getCounter` function with signature `getCounter()` and selector `0x8ada066e`
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
    #[ethcall(name = "getCounter", abi = "getCounter()")]
    pub struct GetCounterCall;
    ///Container type for all input parameters for the `getNft` function with signature `getNft(uint256)` and selector `0x41f63bfd`
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
    #[ethcall(name = "getNft", abi = "getNft(uint256)")]
    pub struct GetNftCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `locked` function with signature `locked(uint256)` and selector `0xb45a3c0e`
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
    #[ethcall(name = "locked", abi = "locked(uint256)")]
    pub struct LockedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `refund` function with signature `refund(uint256)` and selector `0x278ecde1`
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
    #[ethcall(name = "refund", abi = "refund(uint256)")]
    pub struct RefundCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `safeMint` function with signature `safeMint(uint256,address,uint32,string)` and selector `0x1a341ddc`
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
    #[ethcall(name = "safeMint", abi = "safeMint(uint256,address,uint32,string)")]
    pub struct SafeMintCall {
        pub price: ::ethers::core::types::U256,
        pub payment_token: ::ethers::core::types::Address,
        pub reservation_date: u32,
        pub uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendReview` function with signature `sendReview(uint256,string)` and selector `0x85b3a7db`
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
    #[ethcall(name = "sendReview", abi = "sendReview(uint256,string)")]
    pub struct SendReviewCall {
        pub token_id: ::ethers::core::types::U256,
        pub uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `useTicket` function with signature `useTicket(bytes,uint256)` and selector `0xc9e7f4b7`
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
    #[ethcall(name = "useTicket", abi = "useTicket(bytes,uint256)")]
    pub struct UseTicketCall {
        pub signature: ::ethers::core::types::Bytes,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResturantTokenCalls {
        ExpirationRange(ExpirationRangeCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BurnNftNotSold(BurnNftNotSoldCall),
        BurnTicketNotConsumed(BurnTicketNotConsumedCall),
        BuyNFT(BuyNFTCall),
        GetApproved(GetApprovedCall),
        GetControllerAddress(GetControllerAddressCall),
        GetCounter(GetCounterCall),
        GetNft(GetNftCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Locked(LockedCall),
        Name(NameCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Refund(RefundCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeMint(SafeMintCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SendReview(SendReviewCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UseTicket(UseTicketCall),
    }
    impl ::ethers::core::abi::AbiDecode for ResturantTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ExpirationRangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExpirationRange(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BurnNftNotSoldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BurnNftNotSold(decoded));
            }
            if let Ok(decoded)
                = <BurnTicketNotConsumedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BurnTicketNotConsumed(decoded));
            }
            if let Ok(decoded)
                = <BuyNFTCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BuyNFT(decoded));
            }
            if let Ok(decoded)
                = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded)
                = <GetControllerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetControllerAddress(decoded));
            }
            if let Ok(decoded)
                = <GetCounterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCounter(decoded));
            }
            if let Ok(decoded)
                = <GetNftCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNft(decoded));
            }
            if let Ok(decoded)
                = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded)
                = <LockedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded)
                = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded)
                = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded)
                = <RefundCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Refund(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SafeMintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeMint(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded)
                = <SendReviewCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendReview(decoded));
            }
            if let Ok(decoded)
                = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TokenByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenByIndex(decoded));
            }
            if let Ok(decoded)
                = <TokenOfOwnerByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded)
                = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded)
                = <UseTicketCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UseTicket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ResturantTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExpirationRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnNftNotSold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnTicketNotConsumed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BuyNFT(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetControllerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNft(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Refund(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendReview(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenOfOwnerByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UseTicket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ResturantTokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExpirationRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnNftNotSold(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnTicketNotConsumed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BuyNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetControllerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNft(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::Refund(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendReview(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenOfOwnerByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UseTicket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExpirationRangeCall> for ResturantTokenCalls {
        fn from(value: ExpirationRangeCall) -> Self {
            Self::ExpirationRange(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ResturantTokenCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ResturantTokenCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnNftNotSoldCall> for ResturantTokenCalls {
        fn from(value: BurnNftNotSoldCall) -> Self {
            Self::BurnNftNotSold(value)
        }
    }
    impl ::core::convert::From<BurnTicketNotConsumedCall> for ResturantTokenCalls {
        fn from(value: BurnTicketNotConsumedCall) -> Self {
            Self::BurnTicketNotConsumed(value)
        }
    }
    impl ::core::convert::From<BuyNFTCall> for ResturantTokenCalls {
        fn from(value: BuyNFTCall) -> Self {
            Self::BuyNFT(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for ResturantTokenCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<GetControllerAddressCall> for ResturantTokenCalls {
        fn from(value: GetControllerAddressCall) -> Self {
            Self::GetControllerAddress(value)
        }
    }
    impl ::core::convert::From<GetCounterCall> for ResturantTokenCalls {
        fn from(value: GetCounterCall) -> Self {
            Self::GetCounter(value)
        }
    }
    impl ::core::convert::From<GetNftCall> for ResturantTokenCalls {
        fn from(value: GetNftCall) -> Self {
            Self::GetNft(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ResturantTokenCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<LockedCall> for ResturantTokenCalls {
        fn from(value: LockedCall) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<NameCall> for ResturantTokenCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for ResturantTokenCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ResturantTokenCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for ResturantTokenCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<PauseCall> for ResturantTokenCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for ResturantTokenCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<RefundCall> for ResturantTokenCalls {
        fn from(value: RefundCall) -> Self {
            Self::Refund(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ResturantTokenCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SafeMintCall> for ResturantTokenCalls {
        fn from(value: SafeMintCall) -> Self {
            Self::SafeMint(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for ResturantTokenCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for ResturantTokenCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SendReviewCall> for ResturantTokenCalls {
        fn from(value: SendReviewCall) -> Self {
            Self::SendReview(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ResturantTokenCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ResturantTokenCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ResturantTokenCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenByIndexCall> for ResturantTokenCalls {
        fn from(value: TokenByIndexCall) -> Self {
            Self::TokenByIndex(value)
        }
    }
    impl ::core::convert::From<TokenOfOwnerByIndexCall> for ResturantTokenCalls {
        fn from(value: TokenOfOwnerByIndexCall) -> Self {
            Self::TokenOfOwnerByIndex(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for ResturantTokenCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ResturantTokenCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ResturantTokenCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ResturantTokenCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for ResturantTokenCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UseTicketCall> for ResturantTokenCalls {
        fn from(value: UseTicketCall) -> Self {
            Self::UseTicket(value)
        }
    }
    ///Container type for all return fields from the `EXPIRATION_RANGE` function with signature `EXPIRATION_RANGE()` and selector `0x596ac5bd`
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
    pub struct ExpirationRangeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getControllerAddress` function with signature `getControllerAddress()` and selector `0xc57380a2`
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
    pub struct GetControllerAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getCounter` function with signature `getCounter()` and selector `0x8ada066e`
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
    pub struct GetCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNft` function with signature `getNft(uint256)` and selector `0x41f63bfd`
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
    pub struct GetNftReturn(pub Nft);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `locked` function with signature `locked(uint256)` and selector `0xb45a3c0e`
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
    pub struct LockedReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
    pub struct TokenByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
    pub struct TokenOfOwnerByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///`Nft(uint256,address,uint32,bool,string,string)`
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
    pub struct Nft {
        pub price: ::ethers::core::types::U256,
        pub payment_token: ::ethers::core::types::Address,
        pub reservation_date: u32,
        pub locked: bool,
        pub review_uri: ::std::string::String,
        pub uri: ::std::string::String,
    }
}
