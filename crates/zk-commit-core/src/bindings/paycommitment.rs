pub use pay_commitment::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod pay_commitment {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_groth16VerifierAddress",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("depositERC20"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositERC20"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_commitment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("groth16VerifierAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("groth16VerifierAddress",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nullifierHashes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nullifierHashes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pA"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pB"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                            2usize,
                                        ),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pC"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pubSignals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    4usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[4]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LogClaim"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogClaim"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("receipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commitment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nullifierHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogDeposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("depositor"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commitment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PAYCOMMITMENT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x17\x7F8\x03\x80b\0\x17\x7F\x839\x81\x81\x01`@R\x81\x01\x90b\0\x007\x91\x90b\0\x01*V[\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPb\0\x01qV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\0\xF2\x82b\0\0\xC5V[\x90P\x91\x90PV[b\0\x01\x04\x81b\0\0\xE5V[\x81\x14b\0\x01\x10W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x01$\x81b\0\0\xF9V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01DWb\0\x01Cb\0\0\xC0V[[`\0b\0\x01T\x85\x82\x86\x01b\0\x01\x13V[\x92PP` b\0\x01g\x85\x82\x86\x01b\0\x01\x13V[\x91PP\x92P\x92\x90PV[a\x15\xFE\x80b\0\x01\x81`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80c\x17\xCC\x91\\\x14a\0OW\x80cU\x1CO\xD2\x14a\0\x8CW\x80c\x9Dv\xEAX\x14a\0\xB5W\x80c\xA0\xE5\xF8\x90\x14a\0\xE0W\x80c\xA4\xED\x8Bp\x14a\x01\x0BW[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0v`\x04\x806\x03\x81\x01\x90a\0q\x91\x90a\n\xA1V[a\x01'V[`@Qa\0\x83\x91\x90a\n\xE9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x98W`\0\x80\xFD[Pa\0\xB3`\x04\x806\x03\x81\x01\x90a\0\xAE\x91\x90a\x0BoV[a\x01GV[\0[4\x80\x15a\0\xC1W`\0\x80\xFD[Pa\0\xCAa\x053V[`@Qa\0\xD7\x91\x90a\x0C\x19V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xECW`\0\x80\xFD[Pa\0\xF5a\x05WV[`@Qa\x01\x02\x91\x90a\x0C\x19V[`@Q\x80\x91\x03\x90\xF3[a\x01%`\x04\x806\x03\x81\x01\x90a\x01 \x91\x90a\x0CjV[a\x05}V[\0[`\x03` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x04\x80\x10\x15a\x01\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\x82\x90a\r\x07V[`@Q\x80\x91\x03\x90\xFD[`\0\x81`\0`\x04\x81\x10a\x01\xA1Wa\x01\xA0a\r'V[[` \x02\x015\x90P`\0\x82`\x01`\x04\x81\x10a\x01\xBEWa\x01\xBDa\r'V[[` \x02\x015`\0\x1B\x90P`\0\x83`\x02`\x04\x81\x10a\x01\xDEWa\x01\xDDa\r'V[[` \x02\x015`\0\x1B\x90P`\0\x84`\x03`\x04\x81\x10a\x01\xFEWa\x01\xFDa\r'V[[` \x02\x015\x90P`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x02fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02]\x90a\r\xA2V[`@Q\x80\x91\x03\x90\xFD[\x83`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 T\x10\x15a\x02\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xB3\x90a\x0E4V[`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c_\xE8\xC1;\x89\x89\x89\x89`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x1D\x94\x93\x92\x91\x90a\x0F<V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03^\x91\x90a\x0F\xAFV[a\x03\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x94\x90a\x10(V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 Ta\x03\xE8\x91\x90a\x10wV[`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0c\xA9\x05\x9C\xBB`\xE0\x1B\x83\x87`@Q`$\x01a\x04B\x92\x91\x90a\x10\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP\x90Pa\x04\xEA\x81`\0\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\xC8\xCB8\xA4\x0F\xDFm\xD9\x94\tK\x96\x12\xD7\xD1[a\x9Aqt\xBA\xF2v\x04\x11\xE6P8\x13\xC0u\x11\x83\x87\x87\x87`@Qa\x05\x1F\x94\x93\x92\x91\x90a\x10\xF2V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPV[`\0\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x004\x14a\x05\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xB7\x90a\x11\xA9V[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x03\x15a\x08\xF0W`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 T\x14a\x06\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\x16\x90a\x12\x15V[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\x80\x91\x90a\x0C\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a\x12JV[\x90P`\0c#\xB8r\xDD`\xE0\x1B30\x87`@Q`$\x01a\x06\xE2\x93\x92\x91\x90a\x12wV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP\x90Pa\x07\x8A\x81`\0\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xC5\x91\x90a\x0C\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x06\x91\x90a\x12JV[\x90P\x82\x81\x10\x15a\x08KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08B\x90a\x12\xFAV[`@Q\x80\x91\x03\x90\xFD[\x85\x83a\x08W\x91\x90a\x13\x1AV[\x81\x14a\x08\x98W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x8F\x90a\x13\x9AV[`@Q\x80\x91\x03\x90\xFD[\x85`\x02`\0\x87\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F@\xA9\xCB:\x97\x07\xD3\xA6\x80\x91\xD8\xEF\x7F\xFDAX\xD0\x1D\x0B*\xD9+\x1EH\x9A\xBE\x83\x12\xDDT0#3\x87\x87`@Qa\x08\xE3\x93\x92\x91\x90a\x13\xBAV[`@Q\x80\x91\x03\x90\xA1PPPP[PPV[a\x08\xFD\x82a\nSV[a\t<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t3\x90a\x14=V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\td\x91\x90a\x14\xCEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xA6V[``\x91P[P\x91P\x91P\x81\x81\x90a\t\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xE5\x91\x90a\x15:V[`@Q\x80\x91\x03\x90\xFD[P`\0\x81Q\x11\x15a\nMW\x80\x80` \x01\x90Q\x81\x01\x90a\n\r\x91\x90a\x0F\xAFV[a\nLW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nC\x90a\x15\xA8V[`@Q\x80\x91\x03\x90\xFD[[PPPPV[`\0\x80\x82;\x90P`\0\x81\x11\x91PP\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\n~\x81a\nkV[\x81\x14a\n\x89W`\0\x80\xFD[PV[`\0\x815\x90Pa\n\x9B\x81a\nuV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xB7Wa\n\xB6a\nfV[[`\0a\n\xC5\x84\x82\x85\x01a\n\x8CV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\n\xE3\x81a\n\xCEV[\x82RPPV[`\0` \x82\x01\x90Pa\n\xFE`\0\x83\x01\x84a\n\xDAV[\x92\x91PPV[`\0\x80\xFD[`\0\x81\x90P\x82` `\x02\x02\x82\x01\x11\x15a\x0B%Wa\x0B$a\x0B\x04V[[\x92\x91PPV[`\0\x81\x90P\x82`@`\x02\x02\x82\x01\x11\x15a\x0BGWa\x0BFa\x0B\x04V[[\x92\x91PPV[`\0\x81\x90P\x82` `\x04\x02\x82\x01\x11\x15a\x0BiWa\x0Bha\x0B\x04V[[\x92\x91PPV[`\0\x80`\0\x80a\x01\x80\x85\x87\x03\x12\x15a\x0B\x8AWa\x0B\x89a\nfV[[`\0a\x0B\x98\x87\x82\x88\x01a\x0B\tV[\x94PP`@a\x0B\xA9\x87\x82\x88\x01a\x0B+V[\x93PP`\xC0a\x0B\xBA\x87\x82\x88\x01a\x0B\tV[\x92PPa\x01\0a\x0B\xCC\x87\x82\x88\x01a\x0BMV[\x91PP\x92\x95\x91\x94P\x92PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0C\x03\x82a\x0B\xD8V[\x90P\x91\x90PV[a\x0C\x13\x81a\x0B\xF8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0C.`\0\x83\x01\x84a\x0C\nV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0CG\x81a\x0C4V[\x81\x14a\x0CRW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0Cd\x81a\x0C>V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x81Wa\x0C\x80a\nfV[[`\0a\x0C\x8F\x85\x82\x86\x01a\x0CUV[\x92PP` a\x0C\xA0\x85\x82\x86\x01a\n\x8CV[\x91PP\x92P\x92\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7Fincorrect publicInput length\0\0\0\0`\0\x82\x01RPV[`\0a\x0C\xF1`\x1C\x83a\x0C\xAAV[\x91Pa\x0C\xFC\x82a\x0C\xBBV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\r \x81a\x0C\xE4V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FThe note has been already spent\0`\0\x82\x01RPV[`\0a\r\x8C`\x1F\x83a\x0C\xAAV[\x91Pa\r\x97\x82a\rVV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\r\xBB\x81a\r\x7FV[\x90P\x91\x90PV[\x7Fno enough remaining amount to cl`\0\x82\x01R\x7Faim\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0E\x1E`#\x83a\x0C\xAAV[\x91Pa\x0E)\x82a\r\xC2V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0EM\x81a\x0E\x11V[\x90P\x91\x90PV[\x82\x81\x837PPPV[a\x0Ei`@\x83\x83a\x0ETV[PPV[`\0`\x02\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0E\x99`@\x83\x83a\x0ETV[PPV[`\0a\x0E\xA9\x83\x83a\x0E\x8DV[`@\x83\x01\x90P\x92\x91PPV[`\0\x82\x90P\x92\x91PPV[`\0`@\x82\x01\x90P\x91\x90PV[a\x0E\xD6\x81a\x0EmV[a\x0E\xE0\x81\x84a\x0ExV[\x92Pa\x0E\xEB\x82a\x0E\x83V[\x80`\0[\x83\x81\x10\x15a\x0F$Wa\x0F\x01\x82\x84a\x0E\xB5V[a\x0F\x0B\x87\x82a\x0E\x9DV[\x96Pa\x0F\x16\x83a\x0E\xC0V[\x92PP`\x01\x81\x01\x90Pa\x0E\xEFV[PPPPPPV[a\x0F8`\x80\x83\x83a\x0ETV[PPV[`\0a\x01\x80\x82\x01\x90Pa\x0FR`\0\x83\x01\x87a\x0E]V[a\x0F_`@\x83\x01\x86a\x0E\xCDV[a\x0Fl`\xC0\x83\x01\x85a\x0E]V[a\x0Fza\x01\0\x83\x01\x84a\x0F,V[\x95\x94PPPPPV[a\x0F\x8C\x81a\n\xCEV[\x81\x14a\x0F\x97W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x0F\xA9\x81a\x0F\x83V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\xC5Wa\x0F\xC4a\nfV[[`\0a\x0F\xD3\x84\x82\x85\x01a\x0F\x9AV[\x91PP\x92\x91PPV[\x7Fgroth16 verification fail\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x10\x12`\x19\x83a\x0C\xAAV[\x91Pa\x10\x1D\x82a\x0F\xDCV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x10A\x81a\x10\x05V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x10\x82\x82a\x0C4V[\x91Pa\x10\x8D\x83a\x0C4V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x10\xA5Wa\x10\xA4a\x10HV[[\x92\x91PPV[a\x10\xB4\x81a\x0C4V[\x82RPPV[`\0`@\x82\x01\x90Pa\x10\xCF`\0\x83\x01\x85a\x0C\nV[a\x10\xDC` \x83\x01\x84a\x10\xABV[\x93\x92PPPV[a\x10\xEC\x81a\nkV[\x82RPPV[`\0`\x80\x82\x01\x90Pa\x11\x07`\0\x83\x01\x87a\x0C\nV[a\x11\x14` \x83\x01\x86a\x10\xABV[a\x11!`@\x83\x01\x85a\x10\xE3V[a\x11.``\x83\x01\x84a\x10\xE3V[\x95\x94PPPPPV[\x7FETH value is supposed to be 0 fo`\0\x82\x01R\x7Fr ERC20 deposit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x11\x93`/\x83a\x0C\xAAV[\x91Pa\x11\x9E\x82a\x117V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11\xC2\x81a\x11\x86V[\x90P\x91\x90PV[\x7Fcommitment exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x11\xFF`\x11\x83a\x0C\xAAV[\x91Pa\x12\n\x82a\x11\xC9V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x12.\x81a\x11\xF2V[\x90P\x91\x90PV[`\0\x81Q\x90Pa\x12D\x81a\x0C>V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12`Wa\x12_a\nfV[[`\0a\x12n\x84\x82\x85\x01a\x125V[\x91PP\x92\x91PPV[`\0``\x82\x01\x90Pa\x12\x8C`\0\x83\x01\x86a\x0C\nV[a\x12\x99` \x83\x01\x85a\x0C\nV[a\x12\xA6`@\x83\x01\x84a\x10\xABV[\x94\x93PPPPV[\x7FOVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x12\xE4`\x08\x83a\x0C\xAAV[\x91Pa\x12\xEF\x82a\x12\xAEV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x13\x13\x81a\x12\xD7V[\x90P\x91\x90PV[`\0a\x13%\x82a\x0C4V[\x91Pa\x130\x83a\x0C4V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13HWa\x13Ga\x10HV[[\x92\x91PPV[\x7FINCORRECT_AMOUNT_TRANSFERRED\0\0\0\0`\0\x82\x01RPV[`\0a\x13\x84`\x1C\x83a\x0C\xAAV[\x91Pa\x13\x8F\x82a\x13NV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x13\xB3\x81a\x13wV[\x90P\x91\x90PV[`\0``\x82\x01\x90Pa\x13\xCF`\0\x83\x01\x86a\x0C\nV[a\x13\xDC` \x83\x01\x85a\x10\xABV[a\x13\xE9`@\x83\x01\x84a\x10\xE3V[\x94\x93PPPPV[\x7FBAD_TOKEN_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x14'`\x11\x83a\x0C\xAAV[\x91Pa\x142\x82a\x13\xF1V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x14V\x81a\x14\x1AV[\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x14\x91W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x14vV[`\0\x84\x84\x01RPPPPV[`\0a\x14\xA8\x82a\x14]V[a\x14\xB2\x81\x85a\x14hV[\x93Pa\x14\xC2\x81\x85` \x86\x01a\x14sV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x14\xDA\x82\x84a\x14\x9DV[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x15\x0C\x82a\x14\xE5V[a\x15\x16\x81\x85a\x0C\xAAV[\x93Pa\x15&\x81\x85` \x86\x01a\x14sV[a\x15/\x81a\x14\xF0V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x15T\x81\x84a\x15\x01V[\x90P\x92\x91PPV[\x7FTOKEN_OPERATION_FAILED\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x15\x92`\x16\x83a\x0C\xAAV[\x91Pa\x15\x9D\x82a\x15\\V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x15\xC1\x81a\x15\x85V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xB3\x89H\x9F\xEC\xDB\xCC\xC9Z;\xB7\x0E9i\xC3\xAE\"\x95\t\xA7\xE2\x93\xFC\xFD\x9FA`a\x1A+o\xE3dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static PAYCOMMITMENT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80c\x17\xCC\x91\\\x14a\0OW\x80cU\x1CO\xD2\x14a\0\x8CW\x80c\x9Dv\xEAX\x14a\0\xB5W\x80c\xA0\xE5\xF8\x90\x14a\0\xE0W\x80c\xA4\xED\x8Bp\x14a\x01\x0BW[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0v`\x04\x806\x03\x81\x01\x90a\0q\x91\x90a\n\xA1V[a\x01'V[`@Qa\0\x83\x91\x90a\n\xE9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x98W`\0\x80\xFD[Pa\0\xB3`\x04\x806\x03\x81\x01\x90a\0\xAE\x91\x90a\x0BoV[a\x01GV[\0[4\x80\x15a\0\xC1W`\0\x80\xFD[Pa\0\xCAa\x053V[`@Qa\0\xD7\x91\x90a\x0C\x19V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xECW`\0\x80\xFD[Pa\0\xF5a\x05WV[`@Qa\x01\x02\x91\x90a\x0C\x19V[`@Q\x80\x91\x03\x90\xF3[a\x01%`\x04\x806\x03\x81\x01\x90a\x01 \x91\x90a\x0CjV[a\x05}V[\0[`\x03` R\x80`\0R`@`\0 `\0\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x04\x80\x10\x15a\x01\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\x82\x90a\r\x07V[`@Q\x80\x91\x03\x90\xFD[`\0\x81`\0`\x04\x81\x10a\x01\xA1Wa\x01\xA0a\r'V[[` \x02\x015\x90P`\0\x82`\x01`\x04\x81\x10a\x01\xBEWa\x01\xBDa\r'V[[` \x02\x015`\0\x1B\x90P`\0\x83`\x02`\x04\x81\x10a\x01\xDEWa\x01\xDDa\r'V[[` \x02\x015`\0\x1B\x90P`\0\x84`\x03`\x04\x81\x10a\x01\xFEWa\x01\xFDa\r'V[[` \x02\x015\x90P`\x03`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x02fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02]\x90a\r\xA2V[`@Q\x80\x91\x03\x90\xFD[\x83`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 T\x10\x15a\x02\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xB3\x90a\x0E4V[`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c_\xE8\xC1;\x89\x89\x89\x89`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x1D\x94\x93\x92\x91\x90a\x0F<V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03^\x91\x90a\x0F\xAFV[a\x03\x9DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x94\x90a\x10(V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 Ta\x03\xE8\x91\x90a\x10wV[`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0c\xA9\x05\x9C\xBB`\xE0\x1B\x83\x87`@Q`$\x01a\x04B\x92\x91\x90a\x10\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP\x90Pa\x04\xEA\x81`\0\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x7F\xC8\xCB8\xA4\x0F\xDFm\xD9\x94\tK\x96\x12\xD7\xD1[a\x9Aqt\xBA\xF2v\x04\x11\xE6P8\x13\xC0u\x11\x83\x87\x87\x87`@Qa\x05\x1F\x94\x93\x92\x91\x90a\x10\xF2V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPV[`\0\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x01`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x004\x14a\x05\xC0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xB7\x90a\x11\xA9V[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x03\x15a\x08\xF0W`\0`\x02`\0\x83\x81R` \x01\x90\x81R` \x01`\0 T\x14a\x06\x1FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\x16\x90a\x12\x15V[`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\x80\x91\x90a\x0C\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a\x12JV[\x90P`\0c#\xB8r\xDD`\xE0\x1B30\x87`@Q`$\x01a\x06\xE2\x93\x92\x91\x90a\x12wV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP\x90Pa\x07\x8A\x81`\0\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xC5\x91\x90a\x0C\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x06\x91\x90a\x12JV[\x90P\x82\x81\x10\x15a\x08KW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08B\x90a\x12\xFAV[`@Q\x80\x91\x03\x90\xFD[\x85\x83a\x08W\x91\x90a\x13\x1AV[\x81\x14a\x08\x98W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x8F\x90a\x13\x9AV[`@Q\x80\x91\x03\x90\xFD[\x85`\x02`\0\x87\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F@\xA9\xCB:\x97\x07\xD3\xA6\x80\x91\xD8\xEF\x7F\xFDAX\xD0\x1D\x0B*\xD9+\x1EH\x9A\xBE\x83\x12\xDDT0#3\x87\x87`@Qa\x08\xE3\x93\x92\x91\x90a\x13\xBAV[`@Q\x80\x91\x03\x90\xA1PPPP[PPV[a\x08\xFD\x82a\nSV[a\t<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t3\x90a\x14=V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Qa\td\x91\x90a\x14\xCEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xA6V[``\x91P[P\x91P\x91P\x81\x81\x90a\t\xEEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xE5\x91\x90a\x15:V[`@Q\x80\x91\x03\x90\xFD[P`\0\x81Q\x11\x15a\nMW\x80\x80` \x01\x90Q\x81\x01\x90a\n\r\x91\x90a\x0F\xAFV[a\nLW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nC\x90a\x15\xA8V[`@Q\x80\x91\x03\x90\xFD[[PPPPV[`\0\x80\x82;\x90P`\0\x81\x11\x91PP\x91\x90PV[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\n~\x81a\nkV[\x81\x14a\n\x89W`\0\x80\xFD[PV[`\0\x815\x90Pa\n\x9B\x81a\nuV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xB7Wa\n\xB6a\nfV[[`\0a\n\xC5\x84\x82\x85\x01a\n\x8CV[\x91PP\x92\x91PPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\n\xE3\x81a\n\xCEV[\x82RPPV[`\0` \x82\x01\x90Pa\n\xFE`\0\x83\x01\x84a\n\xDAV[\x92\x91PPV[`\0\x80\xFD[`\0\x81\x90P\x82` `\x02\x02\x82\x01\x11\x15a\x0B%Wa\x0B$a\x0B\x04V[[\x92\x91PPV[`\0\x81\x90P\x82`@`\x02\x02\x82\x01\x11\x15a\x0BGWa\x0BFa\x0B\x04V[[\x92\x91PPV[`\0\x81\x90P\x82` `\x04\x02\x82\x01\x11\x15a\x0BiWa\x0Bha\x0B\x04V[[\x92\x91PPV[`\0\x80`\0\x80a\x01\x80\x85\x87\x03\x12\x15a\x0B\x8AWa\x0B\x89a\nfV[[`\0a\x0B\x98\x87\x82\x88\x01a\x0B\tV[\x94PP`@a\x0B\xA9\x87\x82\x88\x01a\x0B+V[\x93PP`\xC0a\x0B\xBA\x87\x82\x88\x01a\x0B\tV[\x92PPa\x01\0a\x0B\xCC\x87\x82\x88\x01a\x0BMV[\x91PP\x92\x95\x91\x94P\x92PV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x0C\x03\x82a\x0B\xD8V[\x90P\x91\x90PV[a\x0C\x13\x81a\x0B\xF8V[\x82RPPV[`\0` \x82\x01\x90Pa\x0C.`\0\x83\x01\x84a\x0C\nV[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0CG\x81a\x0C4V[\x81\x14a\x0CRW`\0\x80\xFD[PV[`\0\x815\x90Pa\x0Cd\x81a\x0C>V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x81Wa\x0C\x80a\nfV[[`\0a\x0C\x8F\x85\x82\x86\x01a\x0CUV[\x92PP` a\x0C\xA0\x85\x82\x86\x01a\n\x8CV[\x91PP\x92P\x92\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7Fincorrect publicInput length\0\0\0\0`\0\x82\x01RPV[`\0a\x0C\xF1`\x1C\x83a\x0C\xAAV[\x91Pa\x0C\xFC\x82a\x0C\xBBV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\r \x81a\x0C\xE4V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FThe note has been already spent\0`\0\x82\x01RPV[`\0a\r\x8C`\x1F\x83a\x0C\xAAV[\x91Pa\r\x97\x82a\rVV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\r\xBB\x81a\r\x7FV[\x90P\x91\x90PV[\x7Fno enough remaining amount to cl`\0\x82\x01R\x7Faim\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x0E\x1E`#\x83a\x0C\xAAV[\x91Pa\x0E)\x82a\r\xC2V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x0EM\x81a\x0E\x11V[\x90P\x91\x90PV[\x82\x81\x837PPPV[a\x0Ei`@\x83\x83a\x0ETV[PPV[`\0`\x02\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x0E\x99`@\x83\x83a\x0ETV[PPV[`\0a\x0E\xA9\x83\x83a\x0E\x8DV[`@\x83\x01\x90P\x92\x91PPV[`\0\x82\x90P\x92\x91PPV[`\0`@\x82\x01\x90P\x91\x90PV[a\x0E\xD6\x81a\x0EmV[a\x0E\xE0\x81\x84a\x0ExV[\x92Pa\x0E\xEB\x82a\x0E\x83V[\x80`\0[\x83\x81\x10\x15a\x0F$Wa\x0F\x01\x82\x84a\x0E\xB5V[a\x0F\x0B\x87\x82a\x0E\x9DV[\x96Pa\x0F\x16\x83a\x0E\xC0V[\x92PP`\x01\x81\x01\x90Pa\x0E\xEFV[PPPPPPV[a\x0F8`\x80\x83\x83a\x0ETV[PPV[`\0a\x01\x80\x82\x01\x90Pa\x0FR`\0\x83\x01\x87a\x0E]V[a\x0F_`@\x83\x01\x86a\x0E\xCDV[a\x0Fl`\xC0\x83\x01\x85a\x0E]V[a\x0Fza\x01\0\x83\x01\x84a\x0F,V[\x95\x94PPPPPV[a\x0F\x8C\x81a\n\xCEV[\x81\x14a\x0F\x97W`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x0F\xA9\x81a\x0F\x83V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\xC5Wa\x0F\xC4a\nfV[[`\0a\x0F\xD3\x84\x82\x85\x01a\x0F\x9AV[\x91PP\x92\x91PPV[\x7Fgroth16 verification fail\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x10\x12`\x19\x83a\x0C\xAAV[\x91Pa\x10\x1D\x82a\x0F\xDCV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x10A\x81a\x10\x05V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x10\x82\x82a\x0C4V[\x91Pa\x10\x8D\x83a\x0C4V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x10\xA5Wa\x10\xA4a\x10HV[[\x92\x91PPV[a\x10\xB4\x81a\x0C4V[\x82RPPV[`\0`@\x82\x01\x90Pa\x10\xCF`\0\x83\x01\x85a\x0C\nV[a\x10\xDC` \x83\x01\x84a\x10\xABV[\x93\x92PPPV[a\x10\xEC\x81a\nkV[\x82RPPV[`\0`\x80\x82\x01\x90Pa\x11\x07`\0\x83\x01\x87a\x0C\nV[a\x11\x14` \x83\x01\x86a\x10\xABV[a\x11!`@\x83\x01\x85a\x10\xE3V[a\x11.``\x83\x01\x84a\x10\xE3V[\x95\x94PPPPPV[\x7FETH value is supposed to be 0 fo`\0\x82\x01R\x7Fr ERC20 deposit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x11\x93`/\x83a\x0C\xAAV[\x91Pa\x11\x9E\x82a\x117V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11\xC2\x81a\x11\x86V[\x90P\x91\x90PV[\x7Fcommitment exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x11\xFF`\x11\x83a\x0C\xAAV[\x91Pa\x12\n\x82a\x11\xC9V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x12.\x81a\x11\xF2V[\x90P\x91\x90PV[`\0\x81Q\x90Pa\x12D\x81a\x0C>V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12`Wa\x12_a\nfV[[`\0a\x12n\x84\x82\x85\x01a\x125V[\x91PP\x92\x91PPV[`\0``\x82\x01\x90Pa\x12\x8C`\0\x83\x01\x86a\x0C\nV[a\x12\x99` \x83\x01\x85a\x0C\nV[a\x12\xA6`@\x83\x01\x84a\x10\xABV[\x94\x93PPPPV[\x7FOVERFLOW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x12\xE4`\x08\x83a\x0C\xAAV[\x91Pa\x12\xEF\x82a\x12\xAEV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x13\x13\x81a\x12\xD7V[\x90P\x91\x90PV[`\0a\x13%\x82a\x0C4V[\x91Pa\x130\x83a\x0C4V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13HWa\x13Ga\x10HV[[\x92\x91PPV[\x7FINCORRECT_AMOUNT_TRANSFERRED\0\0\0\0`\0\x82\x01RPV[`\0a\x13\x84`\x1C\x83a\x0C\xAAV[\x91Pa\x13\x8F\x82a\x13NV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x13\xB3\x81a\x13wV[\x90P\x91\x90PV[`\0``\x82\x01\x90Pa\x13\xCF`\0\x83\x01\x86a\x0C\nV[a\x13\xDC` \x83\x01\x85a\x10\xABV[a\x13\xE9`@\x83\x01\x84a\x10\xE3V[\x94\x93PPPPV[\x7FBAD_TOKEN_ADDRESS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x14'`\x11\x83a\x0C\xAAV[\x91Pa\x142\x82a\x13\xF1V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x14V\x81a\x14\x1AV[\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x14\x91W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x14vV[`\0\x84\x84\x01RPPPPV[`\0a\x14\xA8\x82a\x14]V[a\x14\xB2\x81\x85a\x14hV[\x93Pa\x14\xC2\x81\x85` \x86\x01a\x14sV[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x14\xDA\x82\x84a\x14\x9DV[\x91P\x81\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x15\x0C\x82a\x14\xE5V[a\x15\x16\x81\x85a\x0C\xAAV[\x93Pa\x15&\x81\x85` \x86\x01a\x14sV[a\x15/\x81a\x14\xF0V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x15T\x81\x84a\x15\x01V[\x90P\x92\x91PPV[\x7FTOKEN_OPERATION_FAILED\0\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x15\x92`\x16\x83a\x0C\xAAV[\x91Pa\x15\x9D\x82a\x15\\V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x15\xC1\x81a\x15\x85V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xB3\x89H\x9F\xEC\xDB\xCC\xC9Z;\xB7\x0E9i\xC3\xAE\"\x95\t\xA7\xE2\x93\xFC\xFD\x9FA`a\x1A+o\xE3dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static PAYCOMMITMENT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PayCommitment<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PayCommitment<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PayCommitment<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PayCommitment<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PayCommitment<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PayCommitment)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PayCommitment<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PAYCOMMITMENT_ABI.clone(),
                client,
            ))
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
                PAYCOMMITMENT_ABI.clone(),
                PAYCOMMITMENT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `depositERC20` (0xa4ed8b70) function
        pub fn deposit_erc20(
            &self,
            amount: ::ethers::core::types::U256,
            commitment: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 237, 139, 112], (amount, commitment))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `groth16VerifierAddress` (0xa0e5f890) function
        pub fn groth_16_verifier_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([160, 229, 248, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nullifierHashes` (0x17cc915c) function
        pub fn nullifier_hashes(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 204, 145, 92], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenAddress` (0x9d76ea58) function
        pub fn token_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([157, 118, 234, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x551c4fd2) function
        pub fn withdraw(
            &self,
            p_a: [::ethers::core::types::U256; 2],
            p_b: [[::ethers::core::types::U256; 2]; 2],
            p_c: [::ethers::core::types::U256; 2],
            pub_signals: [::ethers::core::types::U256; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 28, 79, 210], (p_a, p_b, p_c, pub_signals))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LogClaim` event
        pub fn log_claim_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogClaimFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogDeposit` event
        pub fn log_deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogDepositFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PayCommitmentEvents>
        {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PayCommitment<M>
    {
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
        Hash,
    )]
    #[ethevent(name = "LogClaim", abi = "LogClaim(address,uint256,bytes32,bytes32)")]
    pub struct LogClaimFilter {
        pub receipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub commitment: [u8; 32],
        pub nullifier_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "LogDeposit", abi = "LogDeposit(address,uint256,bytes32)")]
    pub struct LogDepositFilter {
        pub depositor: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub commitment: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PayCommitmentEvents {
        LogClaimFilter(LogClaimFilter),
        LogDepositFilter(LogDepositFilter),
    }
    impl ::ethers::contract::EthLogDecode for PayCommitmentEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogClaimFilter::decode_log(log) {
                return Ok(PayCommitmentEvents::LogClaimFilter(decoded));
            }
            if let Ok(decoded) = LogDepositFilter::decode_log(log) {
                return Ok(PayCommitmentEvents::LogDepositFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PayCommitmentEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogClaimFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogDepositFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogClaimFilter> for PayCommitmentEvents {
        fn from(value: LogClaimFilter) -> Self {
            Self::LogClaimFilter(value)
        }
    }
    impl ::core::convert::From<LogDepositFilter> for PayCommitmentEvents {
        fn from(value: LogDepositFilter) -> Self {
            Self::LogDepositFilter(value)
        }
    }
    ///Container type for all input parameters for the `depositERC20` function with signature `depositERC20(uint256,bytes32)` and selector `0xa4ed8b70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "depositERC20", abi = "depositERC20(uint256,bytes32)")]
    pub struct DepositERC20Call {
        pub amount: ::ethers::core::types::U256,
        pub commitment: [u8; 32],
    }
    ///Container type for all input parameters for the `groth16VerifierAddress` function with signature `groth16VerifierAddress()` and selector `0xa0e5f890`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "groth16VerifierAddress", abi = "groth16VerifierAddress()")]
    pub struct Groth16VerifierAddressCall;
    ///Container type for all input parameters for the `nullifierHashes` function with signature `nullifierHashes(bytes32)` and selector `0x17cc915c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(bytes32)")]
    pub struct NullifierHashesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `tokenAddress` function with signature `tokenAddress()` and selector `0x9d76ea58`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "tokenAddress", abi = "tokenAddress()")]
    pub struct TokenAddressCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256[2],uint256[2][2],uint256[2],uint256[4])` and selector `0x551c4fd2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256[2],uint256[2][2],uint256[2],uint256[4])")]
    pub struct WithdrawCall {
        pub p_a: [::ethers::core::types::U256; 2],
        pub p_b: [[::ethers::core::types::U256; 2]; 2],
        pub p_c: [::ethers::core::types::U256; 2],
        pub pub_signals: [::ethers::core::types::U256; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PayCommitmentCalls {
        DepositERC20(DepositERC20Call),
        Groth16VerifierAddress(Groth16VerifierAddressCall),
        NullifierHashes(NullifierHashesCall),
        TokenAddress(TokenAddressCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for PayCommitmentCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositERC20Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositERC20(decoded));
            }
            if let Ok(decoded) =
                <Groth16VerifierAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Groth16VerifierAddress(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NullifierHashes(decoded));
            }
            if let Ok(decoded) = <TokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenAddress(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PayCommitmentCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositERC20(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Groth16VerifierAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NullifierHashes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PayCommitmentCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::Groth16VerifierAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NullifierHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositERC20Call> for PayCommitmentCalls {
        fn from(value: DepositERC20Call) -> Self {
            Self::DepositERC20(value)
        }
    }
    impl ::core::convert::From<Groth16VerifierAddressCall> for PayCommitmentCalls {
        fn from(value: Groth16VerifierAddressCall) -> Self {
            Self::Groth16VerifierAddress(value)
        }
    }
    impl ::core::convert::From<NullifierHashesCall> for PayCommitmentCalls {
        fn from(value: NullifierHashesCall) -> Self {
            Self::NullifierHashes(value)
        }
    }
    impl ::core::convert::From<TokenAddressCall> for PayCommitmentCalls {
        fn from(value: TokenAddressCall) -> Self {
            Self::TokenAddress(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for PayCommitmentCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `groth16VerifierAddress` function with signature `groth16VerifierAddress()` and selector `0xa0e5f890`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Groth16VerifierAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `nullifierHashes` function with signature `nullifierHashes(bytes32)` and selector `0x17cc915c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NullifierHashesReturn(pub bool);
    ///Container type for all return fields from the `tokenAddress` function with signature `tokenAddress()` and selector `0x9d76ea58`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokenAddressReturn(pub ::ethers::core::types::Address);
}
