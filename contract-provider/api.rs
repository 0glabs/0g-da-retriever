pub use api::*;
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
pub mod api {
    const _: () = {
        ::core::include_bytes!(
            "/home/liuyuan/zg/0g-da-retriever/contract-provider/abi/IDASigners.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("epochNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("epochNumber"),
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
                    ::std::borrow::ToOwned::to_owned("getAggPkG1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAggPkG1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_quorumId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_quorumBitmap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aggPkG1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("total"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hit"),
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
                    ::std::borrow::ToOwned::to_owned("getQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getQuorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_epoch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_quorumId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                2usize,
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                2usize,
                                                            ),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDASigners.SignerDetail[]",
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
                    ::std::borrow::ToOwned::to_owned("isSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
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
                    ::std::borrow::ToOwned::to_owned("quorumCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_epoch"),
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
                    ::std::borrow::ToOwned::to_owned("registerNextEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerNextEpoch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                    ::std::borrow::ToOwned::to_owned("registerSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDASigners.SignerDetail",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                    ::std::borrow::ToOwned::to_owned("registeredEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registeredEpoch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_epoch"),
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
                    ::std::borrow::ToOwned::to_owned("updateSocket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateSocket"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_socket"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NewSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pkG1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pkG2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SocketUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SocketUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
    pub static API_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct api<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for api<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for api<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for api<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for api<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(api)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> api<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    API_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `epochNumber` (0xf4145a83) function
        pub fn epoch_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 20, 90, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAggPkG1` (0x50b73739) function
        pub fn get_agg_pk_g1(
            &self,
            epoch: ::ethers::core::types::U256,
            quorum_id: ::ethers::core::types::U256,
            quorum_bitmap: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (G1Point, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([80, 183, 55, 57], (epoch, quorum_id, quorum_bitmap))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorum` (0x6ab6f654) function
        pub fn get_quorum(
            &self,
            epoch: ::ethers::core::types::U256,
            quorum_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([106, 182, 246, 84], (epoch, quorum_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSigner` (0xd1f5e5f8) function
        pub fn get_signer(
            &self,
            account: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<SignerDetail>,
        > {
            self.0
                .method_hash([209, 245, 229, 248], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSigner` (0x7df73e27) function
        pub fn is_signer(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([125, 247, 62, 39], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumCount` (0x5ecba503) function
        pub fn quorum_count(
            &self,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 203, 165, 3], epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerNextEpoch` (0x56a32372) function
        pub fn register_next_epoch(
            &self,
            signature: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 163, 35, 114], (signature,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSigner` (0x7ca4dd5e) function
        pub fn register_signer(
            &self,
            signer: SignerDetail,
            signature: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 164, 221, 94], (signer, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registeredEpoch` (0x6c9e560c) function
        pub fn registered_epoch(
            &self,
            account: ::ethers::core::types::Address,
            epoch: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 158, 86, 12], (account, epoch))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSocket` (0x0cf4b767) function
        pub fn update_socket(
            &self,
            socket: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 244, 183, 103], socket)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewSigner` event
        pub fn new_signer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewSignerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SocketUpdated` event
        pub fn socket_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SocketUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, apiEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for api<M> {
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
    #[ethevent(
        name = "NewSigner",
        abi = "NewSigner(address,(uint256,uint256),(uint256[2],uint256[2]))"
    )]
    pub struct NewSignerFilter {
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
        pub pk_g1: G1Point,
        pub pk_g2: G2Point,
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
    #[ethevent(name = "SocketUpdated", abi = "SocketUpdated(address,string)")]
    pub struct SocketUpdatedFilter {
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
        pub socket: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum apiEvents {
        NewSignerFilter(NewSignerFilter),
        SocketUpdatedFilter(SocketUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for apiEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewSignerFilter::decode_log(log) {
                return Ok(apiEvents::NewSignerFilter(decoded));
            }
            if let Ok(decoded) = SocketUpdatedFilter::decode_log(log) {
                return Ok(apiEvents::SocketUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for apiEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewSignerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SocketUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<NewSignerFilter> for apiEvents {
        fn from(value: NewSignerFilter) -> Self {
            Self::NewSignerFilter(value)
        }
    }
    impl ::core::convert::From<SocketUpdatedFilter> for apiEvents {
        fn from(value: SocketUpdatedFilter) -> Self {
            Self::SocketUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `epochNumber` function with signature `epochNumber()` and selector `0xf4145a83`
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
    #[ethcall(name = "epochNumber", abi = "epochNumber()")]
    pub struct EpochNumberCall;
    ///Container type for all input parameters for the `getAggPkG1` function with signature `getAggPkG1(uint256,uint256,bytes)` and selector `0x50b73739`
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
    #[ethcall(name = "getAggPkG1", abi = "getAggPkG1(uint256,uint256,bytes)")]
    pub struct GetAggPkG1Call {
        pub epoch: ::ethers::core::types::U256,
        pub quorum_id: ::ethers::core::types::U256,
        pub quorum_bitmap: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getQuorum` function with signature `getQuorum(uint256,uint256)` and selector `0x6ab6f654`
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
    #[ethcall(name = "getQuorum", abi = "getQuorum(uint256,uint256)")]
    pub struct GetQuorumCall {
        pub epoch: ::ethers::core::types::U256,
        pub quorum_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSigner` function with signature `getSigner(address[])` and selector `0xd1f5e5f8`
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
    #[ethcall(name = "getSigner", abi = "getSigner(address[])")]
    pub struct GetSignerCall {
        pub account: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `isSigner` function with signature `isSigner(address)` and selector `0x7df73e27`
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
    #[ethcall(name = "isSigner", abi = "isSigner(address)")]
    pub struct IsSignerCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quorumCount` function with signature `quorumCount(uint256)` and selector `0x5ecba503`
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
    #[ethcall(name = "quorumCount", abi = "quorumCount(uint256)")]
    pub struct QuorumCountCall {
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerNextEpoch` function with signature `registerNextEpoch((uint256,uint256))` and selector `0x56a32372`
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
    #[ethcall(name = "registerNextEpoch", abi = "registerNextEpoch((uint256,uint256))")]
    pub struct RegisterNextEpochCall {
        pub signature: G1Point,
    }
    ///Container type for all input parameters for the `registerSigner` function with signature `registerSigner((address,string,(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))` and selector `0x7ca4dd5e`
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
        name = "registerSigner",
        abi = "registerSigner((address,string,(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))"
    )]
    pub struct RegisterSignerCall {
        pub signer: SignerDetail,
        pub signature: G1Point,
    }
    ///Container type for all input parameters for the `registeredEpoch` function with signature `registeredEpoch(address,uint256)` and selector `0x6c9e560c`
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
    #[ethcall(name = "registeredEpoch", abi = "registeredEpoch(address,uint256)")]
    pub struct RegisteredEpochCall {
        pub account: ::ethers::core::types::Address,
        pub epoch: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateSocket` function with signature `updateSocket(string)` and selector `0x0cf4b767`
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
    #[ethcall(name = "updateSocket", abi = "updateSocket(string)")]
    pub struct UpdateSocketCall {
        pub socket: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum apiCalls {
        EpochNumber(EpochNumberCall),
        GetAggPkG1(GetAggPkG1Call),
        GetQuorum(GetQuorumCall),
        GetSigner(GetSignerCall),
        IsSigner(IsSignerCall),
        QuorumCount(QuorumCountCall),
        RegisterNextEpoch(RegisterNextEpochCall),
        RegisterSigner(RegisterSignerCall),
        RegisteredEpoch(RegisteredEpochCall),
        UpdateSocket(UpdateSocketCall),
    }
    impl ::ethers::core::abi::AbiDecode for apiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EpochNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EpochNumber(decoded));
            }
            if let Ok(decoded) = <GetAggPkG1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAggPkG1(decoded));
            }
            if let Ok(decoded) = <GetQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorum(decoded));
            }
            if let Ok(decoded) = <GetSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSigner(decoded));
            }
            if let Ok(decoded) = <IsSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsSigner(decoded));
            }
            if let Ok(decoded) = <QuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumCount(decoded));
            }
            if let Ok(decoded) = <RegisterNextEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterNextEpoch(decoded));
            }
            if let Ok(decoded) = <RegisterSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterSigner(decoded));
            }
            if let Ok(decoded) = <RegisteredEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisteredEpoch(decoded));
            }
            if let Ok(decoded) = <UpdateSocketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateSocket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for apiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EpochNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAggPkG1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterNextEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisteredEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateSocket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for apiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EpochNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAggPkG1(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterNextEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSocket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EpochNumberCall> for apiCalls {
        fn from(value: EpochNumberCall) -> Self {
            Self::EpochNumber(value)
        }
    }
    impl ::core::convert::From<GetAggPkG1Call> for apiCalls {
        fn from(value: GetAggPkG1Call) -> Self {
            Self::GetAggPkG1(value)
        }
    }
    impl ::core::convert::From<GetQuorumCall> for apiCalls {
        fn from(value: GetQuorumCall) -> Self {
            Self::GetQuorum(value)
        }
    }
    impl ::core::convert::From<GetSignerCall> for apiCalls {
        fn from(value: GetSignerCall) -> Self {
            Self::GetSigner(value)
        }
    }
    impl ::core::convert::From<IsSignerCall> for apiCalls {
        fn from(value: IsSignerCall) -> Self {
            Self::IsSigner(value)
        }
    }
    impl ::core::convert::From<QuorumCountCall> for apiCalls {
        fn from(value: QuorumCountCall) -> Self {
            Self::QuorumCount(value)
        }
    }
    impl ::core::convert::From<RegisterNextEpochCall> for apiCalls {
        fn from(value: RegisterNextEpochCall) -> Self {
            Self::RegisterNextEpoch(value)
        }
    }
    impl ::core::convert::From<RegisterSignerCall> for apiCalls {
        fn from(value: RegisterSignerCall) -> Self {
            Self::RegisterSigner(value)
        }
    }
    impl ::core::convert::From<RegisteredEpochCall> for apiCalls {
        fn from(value: RegisteredEpochCall) -> Self {
            Self::RegisteredEpoch(value)
        }
    }
    impl ::core::convert::From<UpdateSocketCall> for apiCalls {
        fn from(value: UpdateSocketCall) -> Self {
            Self::UpdateSocket(value)
        }
    }
    ///Container type for all return fields from the `epochNumber` function with signature `epochNumber()` and selector `0xf4145a83`
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
    pub struct EpochNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAggPkG1` function with signature `getAggPkG1(uint256,uint256,bytes)` and selector `0x50b73739`
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
    pub struct GetAggPkG1Return {
        pub agg_pk_g1: G1Point,
        pub total: ::ethers::core::types::U256,
        pub hit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getQuorum` function with signature `getQuorum(uint256,uint256)` and selector `0x6ab6f654`
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
    pub struct GetQuorumReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getSigner` function with signature `getSigner(address[])` and selector `0xd1f5e5f8`
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
    pub struct GetSignerReturn(pub ::std::vec::Vec<SignerDetail>);
    ///Container type for all return fields from the `isSigner` function with signature `isSigner(address)` and selector `0x7df73e27`
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
    pub struct IsSignerReturn(pub bool);
    ///Container type for all return fields from the `quorumCount` function with signature `quorumCount(uint256)` and selector `0x5ecba503`
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
    pub struct QuorumCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `registeredEpoch` function with signature `registeredEpoch(address,uint256)` and selector `0x6c9e560c`
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
    pub struct RegisteredEpochReturn(pub bool);
    ///`G1Point(uint256,uint256)`
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
    pub struct G1Point {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///`G2Point(uint256[2],uint256[2])`
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
    pub struct G2Point {
        pub x: [::ethers::core::types::U256; 2],
        pub y: [::ethers::core::types::U256; 2],
    }
    ///`SignerDetail(address,string,(uint256,uint256),(uint256[2],uint256[2]))`
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
    pub struct SignerDetail {
        pub signer: ::ethers::core::types::Address,
        pub socket: ::std::string::String,
        pub pk_g1: G1Point,
        pub pk_g2: G2Point,
    }
}
