pub use mts_controller_mock::*;
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
pub mod mts_controller_mock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getFeeInBasePoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFeeInBasePoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
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
                    ::std::borrow::ToOwned::to_owned("isPriceAcceptable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPriceAcceptable"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("setAcceptableMinPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setAcceptableMinPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumPrice"),
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
                    ::std::borrow::ToOwned::to_owned("setBasePintFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBasePintFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("basePointFees"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MTSCONTROLLERMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\x07\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cOE\x98\x94\x14a\0QW\x80cZ\xE9\xCEu\x14a\0\x9AW\x80c\xAD\xBD\x92\xCA\x14a\0\xBDW\x80c\xDE\x95\xCE\xFF\x14a\0\xF6W[`\0\x80\xFD[a\0\x87a\0_6`\x04a\x01\xBBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xADa\0\xA86`\x04a\x01\xDDV[a\x01-V[`@Q\x90\x15\x15\x81R` \x01a\0\x91V[a\0\xF4a\0\xCB6`\x04a\x01\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 UV[\0[a\0\xF4a\x01\x046`\x04a\x01\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 T\x81\x03a\x01aWP`\0a\x01\x8CV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x10\x15[\x92\x91PPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xB6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xCDW`\0\x80\xFD[a\x01\xD6\x82a\x01\x92V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xF0W`\0\x80\xFD[a\x01\xF9\x83a\x01\x92V[\x94` \x93\x90\x93\x015\x93PPPV";
    /// The bytecode of the contract.
    pub static MTSCONTROLLERMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cOE\x98\x94\x14a\0QW\x80cZ\xE9\xCEu\x14a\0\x9AW\x80c\xAD\xBD\x92\xCA\x14a\0\xBDW\x80c\xDE\x95\xCE\xFF\x14a\0\xF6W[`\0\x80\xFD[a\0\x87a\0_6`\x04a\x01\xBBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xADa\0\xA86`\x04a\x01\xDDV[a\x01-V[`@Q\x90\x15\x15\x81R` \x01a\0\x91V[a\0\xF4a\0\xCB6`\x04a\x01\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 UV[\0[a\0\xF4a\x01\x046`\x04a\x01\xDDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 T\x81\x03a\x01aWP`\0a\x01\x8CV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x10\x15[\x92\x91PPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xB6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xCDW`\0\x80\xFD[a\x01\xD6\x82a\x01\x92V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xF0W`\0\x80\xFD[a\x01\xF9\x83a\x01\x92V[\x94` \x93\x90\x93\x015\x93PPPV";
    /// The deployed bytecode of the contract.
    pub static MTSCONTROLLERMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MTSControllerMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MTSControllerMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MTSControllerMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MTSControllerMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MTSControllerMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MTSControllerMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MTSControllerMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MTSCONTROLLERMOCK_ABI.clone(),
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
                MTSCONTROLLERMOCK_ABI.clone(),
                MTSCONTROLLERMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getFeeInBasePoint` (0x4f459894) function
        pub fn get_fee_in_base_point(
            &self,
            payment_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 69, 152, 148], payment_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPriceAcceptable` (0x5ae9ce75) function
        pub fn is_price_acceptable(
            &self,
            payment_token: ::ethers::core::types::Address,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 233, 206, 117], (payment_token, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAcceptableMinPrice` (0xde95ceff) function
        pub fn set_acceptable_min_price(
            &self,
            payment_token: ::ethers::core::types::Address,
            minimum_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 149, 206, 255], (payment_token, minimum_price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBasePintFees` (0xadbd92ca) function
        pub fn set_base_pint_fees(
            &self,
            payment_token: ::ethers::core::types::Address,
            base_point_fees: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 189, 146, 202], (payment_token, base_point_fees))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MTSControllerMock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getFeeInBasePoint` function with signature `getFeeInBasePoint(address)` and selector `0x4f459894`
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
    #[ethcall(name = "getFeeInBasePoint", abi = "getFeeInBasePoint(address)")]
    pub struct GetFeeInBasePointCall {
        pub payment_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPriceAcceptable` function with signature `isPriceAcceptable(address,uint256)` and selector `0x5ae9ce75`
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
    #[ethcall(name = "isPriceAcceptable", abi = "isPriceAcceptable(address,uint256)")]
    pub struct IsPriceAcceptableCall {
        pub payment_token: ::ethers::core::types::Address,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setAcceptableMinPrice` function with signature `setAcceptableMinPrice(address,uint256)` and selector `0xde95ceff`
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
        name = "setAcceptableMinPrice",
        abi = "setAcceptableMinPrice(address,uint256)"
    )]
    pub struct SetAcceptableMinPriceCall {
        pub payment_token: ::ethers::core::types::Address,
        pub minimum_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setBasePintFees` function with signature `setBasePintFees(address,uint256)` and selector `0xadbd92ca`
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
    #[ethcall(name = "setBasePintFees", abi = "setBasePintFees(address,uint256)")]
    pub struct SetBasePintFeesCall {
        pub payment_token: ::ethers::core::types::Address,
        pub base_point_fees: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MTSControllerMockCalls {
        GetFeeInBasePoint(GetFeeInBasePointCall),
        IsPriceAcceptable(IsPriceAcceptableCall),
        SetAcceptableMinPrice(SetAcceptableMinPriceCall),
        SetBasePintFees(SetBasePintFeesCall),
    }
    impl ::ethers::core::abi::AbiDecode for MTSControllerMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetFeeInBasePointCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetFeeInBasePoint(decoded));
            }
            if let Ok(decoded)
                = <IsPriceAcceptableCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsPriceAcceptable(decoded));
            }
            if let Ok(decoded)
                = <SetAcceptableMinPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetAcceptableMinPrice(decoded));
            }
            if let Ok(decoded)
                = <SetBasePintFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBasePintFees(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MTSControllerMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetFeeInBasePoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPriceAcceptable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAcceptableMinPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBasePintFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MTSControllerMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetFeeInBasePoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPriceAcceptable(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAcceptableMinPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetBasePintFees(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetFeeInBasePointCall> for MTSControllerMockCalls {
        fn from(value: GetFeeInBasePointCall) -> Self {
            Self::GetFeeInBasePoint(value)
        }
    }
    impl ::core::convert::From<IsPriceAcceptableCall> for MTSControllerMockCalls {
        fn from(value: IsPriceAcceptableCall) -> Self {
            Self::IsPriceAcceptable(value)
        }
    }
    impl ::core::convert::From<SetAcceptableMinPriceCall> for MTSControllerMockCalls {
        fn from(value: SetAcceptableMinPriceCall) -> Self {
            Self::SetAcceptableMinPrice(value)
        }
    }
    impl ::core::convert::From<SetBasePintFeesCall> for MTSControllerMockCalls {
        fn from(value: SetBasePintFeesCall) -> Self {
            Self::SetBasePintFees(value)
        }
    }
    ///Container type for all return fields from the `getFeeInBasePoint` function with signature `getFeeInBasePoint(address)` and selector `0x4f459894`
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
    pub struct GetFeeInBasePointReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isPriceAcceptable` function with signature `isPriceAcceptable(address,uint256)` and selector `0x5ae9ce75`
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
    pub struct IsPriceAcceptableReturn(pub bool);
}
