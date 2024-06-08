pub use usdt_eth20_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod usdt_eth20_token {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ::ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ::ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ::ethers::providers::Middleware;
    ///USDT_ETH20Token was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_upgradedAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"deprecate\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"_value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"deprecated\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_evilUser\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addBlackList\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_from\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"_to\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"_value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"upgradedAddress\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balances\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"maximumFee\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"_totalSupply\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [],\n    \"name\": \"unpause\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"_maker\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getBlackListStatus\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowed\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [],\n    \"name\": \"pause\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"getOwner\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_to\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"_value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"newBasisPoints\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"newMaxFee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setParams\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"issue\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"redeem\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"_owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"_spender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [\n      {\n        \"name\": \"remaining\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"basisPointsRate\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isBlackListed\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_clearedUser\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"removeBlackList\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"MAX_UINT\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_blackListedUser\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"destroyBlackFunds\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"name\": \"_initialSupply\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"name\": \"_symbol\",\n        \"type\": \"string\"\n      },\n      {\n        \"name\": \"_decimals\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Issue\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Redeem\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Deprecate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"feeBasisPoints\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"name\": \"maxFee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Params\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"_blackListedUser\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"name\": \"_balance\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"DestroyedBlackFunds\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"AddedBlackList\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RemovedBlackList\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [],\n    \"name\": \"Pause\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [],\n    \"name\": \"Unpause\",\n    \"type\": \"event\"\n  }\n]";
    /// The parsed JSON-ABI of the contract.
    pub static USDT_ETH20TOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct USDT_ETH20Token<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for USDT_ETH20Token<M> {
        fn clone(&self) -> Self {
            USDT_ETH20Token(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for USDT_ETH20Token<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for USDT_ETH20Token<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(USDT_ETH20Token))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> USDT_ETH20Token<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                USDT_ETH20TOKEN_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `MAX_UINT` (0xe5b5019a) function
        pub fn max_uint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 181, 1, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_totalSupply` (0x3eaaf86b) function
        pub fn _total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 170, 248, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBlackList` (0x0ecb93c0) function
        pub fn add_black_list(
            &self,
            evil_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 203, 147, 192], evil_user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowed` (0x5c658165) function
        pub fn allowed(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 101, 129, 101], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            who: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], who)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `basisPointsRate` (0xdd644f72) function
        pub fn basis_points_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 100, 79, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deprecate` (0x0753c30c) function
        pub fn deprecate(
            &self,
            upgraded_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 83, 195, 12], upgraded_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deprecated` (0x0e136b19) function
        pub fn deprecated(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 19, 107, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `destroyBlackFunds` (0xf3bdc228) function
        pub fn destroy_black_funds(
            &self,
            black_listed_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 189, 194, 40], black_listed_user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlackListStatus` (0x59bf1abe) function
        pub fn get_black_list_status(
            &self,
            maker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([89, 191, 26, 190], maker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwner` (0x893d20e8) function
        pub fn get_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([137, 61, 32, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBlackListed` (0xe47d6060) function
        pub fn is_black_listed(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 125, 96, 96], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `issue` (0xcc872b66) function
        pub fn issue(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 135, 43, 102], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maximumFee` (0x35390714) function
        pub fn maximum_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 57, 7, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
        ///Calls the contract's `redeem` (0xdb006a75) function
        pub fn redeem(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 0, 106, 117], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBlackList` (0xe4997dc5) function
        pub fn remove_black_list(
            &self,
            cleared_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 153, 125, 197], cleared_user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setParams` (0xc0324c77) function
        pub fn set_params(
            &self,
            new_basis_points: ::ethers::core::types::U256,
            new_max_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 50, 76, 119], (new_basis_points, new_max_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
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
        ///Calls the contract's `upgradedAddress` (0x26976e3f) function
        pub fn upgraded_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([38, 151, 110, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedBlackList` event
        pub fn added_black_list_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, AddedBlackListFilter> {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(&self) -> ::ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `Deprecate` event
        pub fn deprecate_filter(&self) -> ::ethers::contract::builders::Event<M, DeprecateFilter> {
            self.0.event()
        }
        ///Gets the contract's `DestroyedBlackFunds` event
        pub fn destroyed_black_funds_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, DestroyedBlackFundsFilter> {
            self.0.event()
        }
        ///Gets the contract's `Issue` event
        pub fn issue_filter(&self) -> ::ethers::contract::builders::Event<M, IssueFilter> {
            self.0.event()
        }
        ///Gets the contract's `Params` event
        pub fn params_filter(&self) -> ::ethers::contract::builders::Event<M, ParamsFilter> {
            self.0.event()
        }
        ///Gets the contract's `Pause` event
        pub fn pause_filter(&self) -> ::ethers::contract::builders::Event<M, PauseFilter> {
            self.0.event()
        }
        ///Gets the contract's `Redeem` event
        pub fn redeem_filter(&self) -> ::ethers::contract::builders::Event<M, RedeemFilter> {
            self.0.event()
        }
        ///Gets the contract's `RemovedBlackList` event
        pub fn removed_black_list_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, RemovedBlackListFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(&self) -> ::ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        ///Gets the contract's `Unpause` event
        pub fn unpause_filter(&self) -> ::ethers::contract::builders::Event<M, UnpauseFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, USDT_ETH20TokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for USDT_ETH20Token<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "AddedBlackList", abi = "AddedBlackList(address)")]
    pub struct AddedBlackListFilter {
        pub user: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Deprecate", abi = "Deprecate(address)")]
    pub struct DeprecateFilter {
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "DestroyedBlackFunds",
        abi = "DestroyedBlackFunds(address,uint256)"
    )]
    pub struct DestroyedBlackFundsFilter {
        pub black_listed_user: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Issue", abi = "Issue(uint256)")]
    pub struct IssueFilter {
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Params", abi = "Params(uint256,uint256)")]
    pub struct ParamsFilter {
        pub fee_basis_points: ::ethers::core::types::U256,
        pub max_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Pause", abi = "Pause()")]
    pub struct PauseFilter();
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Redeem", abi = "Redeem(uint256)")]
    pub struct RedeemFilter {
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "RemovedBlackList", abi = "RemovedBlackList(address)")]
    pub struct RemovedBlackListFilter {
        pub user: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Unpause", abi = "Unpause()")]
    pub struct UnpauseFilter();
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum USDT_ETH20TokenEvents {
        AddedBlackListFilter(AddedBlackListFilter),
        ApprovalFilter(ApprovalFilter),
        DeprecateFilter(DeprecateFilter),
        DestroyedBlackFundsFilter(DestroyedBlackFundsFilter),
        IssueFilter(IssueFilter),
        ParamsFilter(ParamsFilter),
        PauseFilter(PauseFilter),
        RedeemFilter(RedeemFilter),
        RemovedBlackListFilter(RemovedBlackListFilter),
        TransferFilter(TransferFilter),
        UnpauseFilter(UnpauseFilter),
    }
    impl ::ethers::contract::EthLogDecode for USDT_ETH20TokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddedBlackListFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::AddedBlackListFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DeprecateFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::DeprecateFilter(decoded));
            }
            if let Ok(decoded) = DestroyedBlackFundsFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::DestroyedBlackFundsFilter(decoded));
            }
            if let Ok(decoded) = IssueFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::IssueFilter(decoded));
            }
            if let Ok(decoded) = ParamsFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::ParamsFilter(decoded));
            }
            if let Ok(decoded) = PauseFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::PauseFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RemovedBlackListFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::RemovedBlackListFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpauseFilter::decode_log(log) {
                return Ok(USDT_ETH20TokenEvents::UnpauseFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for USDT_ETH20TokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                USDT_ETH20TokenEvents::AddedBlackListFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::ApprovalFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::DeprecateFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::DestroyedBlackFundsFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::IssueFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::ParamsFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::PauseFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::RedeemFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::RemovedBlackListFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::TransferFilter(element) => element.fmt(f),
                USDT_ETH20TokenEvents::UnpauseFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `MAX_UINT` function with signature `MAX_UINT()` and selector `0xe5b5019a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_UINT", abi = "MAX_UINT()")]
    pub struct MaxUintCall;
    ///Container type for all input parameters for the `_totalSupply` function with signature `_totalSupply()` and selector `0x3eaaf86b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "_totalSupply", abi = "_totalSupply()")]
    pub struct _TotalSupplyCall;
    ///Container type for all input parameters for the `addBlackList` function with signature `addBlackList(address)` and selector `0x0ecb93c0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "addBlackList", abi = "addBlackList(address)")]
    pub struct AddBlackListCall {
        pub evil_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowed` function with signature `allowed(address,address)` and selector `0x5c658165`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowed", abi = "allowed(address,address)")]
    pub struct AllowedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub who: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `basisPointsRate` function with signature `basisPointsRate()` and selector `0xdd644f72`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "basisPointsRate", abi = "basisPointsRate()")]
    pub struct BasisPointsRateCall;
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `deprecate` function with signature `deprecate(address)` and selector `0x0753c30c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "deprecate", abi = "deprecate(address)")]
    pub struct DeprecateCall {
        pub upgraded_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deprecated` function with signature `deprecated()` and selector `0x0e136b19`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "deprecated", abi = "deprecated()")]
    pub struct DeprecatedCall;
    ///Container type for all input parameters for the `destroyBlackFunds` function with signature `destroyBlackFunds(address)` and selector `0xf3bdc228`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "destroyBlackFunds", abi = "destroyBlackFunds(address)")]
    pub struct DestroyBlackFundsCall {
        pub black_listed_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getBlackListStatus` function with signature `getBlackListStatus(address)` and selector `0x59bf1abe`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBlackListStatus", abi = "getBlackListStatus(address)")]
    pub struct GetBlackListStatusCall {
        pub maker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOwner` function with signature `getOwner()` and selector `0x893d20e8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOwner", abi = "getOwner()")]
    pub struct GetOwnerCall;
    ///Container type for all input parameters for the `isBlackListed` function with signature `isBlackListed(address)` and selector `0xe47d6060`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "isBlackListed", abi = "isBlackListed(address)")]
    pub struct IsBlackListedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `issue` function with signature `issue(uint256)` and selector `0xcc872b66`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "issue", abi = "issue(uint256)")]
    pub struct IssueCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `maximumFee` function with signature `maximumFee()` and selector `0x35390714`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "maximumFee", abi = "maximumFee()")]
    pub struct MaximumFeeCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256)` and selector `0xdb006a75`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "redeem", abi = "redeem(uint256)")]
    pub struct RedeemCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeBlackList` function with signature `removeBlackList(address)` and selector `0xe4997dc5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeBlackList", abi = "removeBlackList(address)")]
    pub struct RemoveBlackListCall {
        pub cleared_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setParams` function with signature `setParams(uint256,uint256)` and selector `0xc0324c77`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setParams", abi = "setParams(uint256,uint256)")]
    pub struct SetParamsCall {
        pub new_basis_points: ::ethers::core::types::U256,
        pub new_max_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `upgradedAddress` function with signature `upgradedAddress()` and selector `0x26976e3f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradedAddress", abi = "upgradedAddress()")]
    pub struct UpgradedAddressCall;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum USDT_ETH20TokenCalls {
        MaxUint(MaxUintCall),
        _TotalSupply(_TotalSupplyCall),
        AddBlackList(AddBlackListCall),
        Allowance(AllowanceCall),
        Allowed(AllowedCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Balances(BalancesCall),
        BasisPointsRate(BasisPointsRateCall),
        Decimals(DecimalsCall),
        Deprecate(DeprecateCall),
        Deprecated(DeprecatedCall),
        DestroyBlackFunds(DestroyBlackFundsCall),
        GetBlackListStatus(GetBlackListStatusCall),
        GetOwner(GetOwnerCall),
        IsBlackListed(IsBlackListedCall),
        Issue(IssueCall),
        MaximumFee(MaximumFeeCall),
        Name(NameCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Redeem(RedeemCall),
        RemoveBlackList(RemoveBlackListCall),
        SetParams(SetParamsCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpgradedAddress(UpgradedAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for USDT_ETH20TokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxUintCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::MaxUint(decoded));
            }
            if let Ok(decoded) =
                <_TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::_TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <AddBlackListCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::AddBlackList(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <AllowedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Allowed(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Balances(decoded));
            }
            if let Ok(decoded) =
                <BasisPointsRateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::BasisPointsRate(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DeprecateCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Deprecate(decoded));
            }
            if let Ok(decoded) =
                <DeprecatedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Deprecated(decoded));
            }
            if let Ok(decoded) =
                <DestroyBlackFundsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::DestroyBlackFunds(decoded));
            }
            if let Ok(decoded) =
                <GetBlackListStatusCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::GetBlackListStatus(decoded));
            }
            if let Ok(decoded) =
                <GetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::GetOwner(decoded));
            }
            if let Ok(decoded) =
                <IsBlackListedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::IsBlackListed(decoded));
            }
            if let Ok(decoded) =
                <IssueCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Issue(decoded));
            }
            if let Ok(decoded) =
                <MaximumFeeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::MaximumFee(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Pause(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RemoveBlackListCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::RemoveBlackList(decoded));
            }
            if let Ok(decoded) =
                <SetParamsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::SetParams(decoded));
            }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpgradedAddressCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(USDT_ETH20TokenCalls::UpgradedAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for USDT_ETH20TokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                USDT_ETH20TokenCalls::MaxUint(element) => element.encode(),
                USDT_ETH20TokenCalls::_TotalSupply(element) => element.encode(),
                USDT_ETH20TokenCalls::AddBlackList(element) => element.encode(),
                USDT_ETH20TokenCalls::Allowance(element) => element.encode(),
                USDT_ETH20TokenCalls::Allowed(element) => element.encode(),
                USDT_ETH20TokenCalls::Approve(element) => element.encode(),
                USDT_ETH20TokenCalls::BalanceOf(element) => element.encode(),
                USDT_ETH20TokenCalls::Balances(element) => element.encode(),
                USDT_ETH20TokenCalls::BasisPointsRate(element) => element.encode(),
                USDT_ETH20TokenCalls::Decimals(element) => element.encode(),
                USDT_ETH20TokenCalls::Deprecate(element) => element.encode(),
                USDT_ETH20TokenCalls::Deprecated(element) => element.encode(),
                USDT_ETH20TokenCalls::DestroyBlackFunds(element) => element.encode(),
                USDT_ETH20TokenCalls::GetBlackListStatus(element) => element.encode(),
                USDT_ETH20TokenCalls::GetOwner(element) => element.encode(),
                USDT_ETH20TokenCalls::IsBlackListed(element) => element.encode(),
                USDT_ETH20TokenCalls::Issue(element) => element.encode(),
                USDT_ETH20TokenCalls::MaximumFee(element) => element.encode(),
                USDT_ETH20TokenCalls::Name(element) => element.encode(),
                USDT_ETH20TokenCalls::Owner(element) => element.encode(),
                USDT_ETH20TokenCalls::Pause(element) => element.encode(),
                USDT_ETH20TokenCalls::Paused(element) => element.encode(),
                USDT_ETH20TokenCalls::Redeem(element) => element.encode(),
                USDT_ETH20TokenCalls::RemoveBlackList(element) => element.encode(),
                USDT_ETH20TokenCalls::SetParams(element) => element.encode(),
                USDT_ETH20TokenCalls::Symbol(element) => element.encode(),
                USDT_ETH20TokenCalls::TotalSupply(element) => element.encode(),
                USDT_ETH20TokenCalls::Transfer(element) => element.encode(),
                USDT_ETH20TokenCalls::TransferFrom(element) => element.encode(),
                USDT_ETH20TokenCalls::TransferOwnership(element) => element.encode(),
                USDT_ETH20TokenCalls::Unpause(element) => element.encode(),
                USDT_ETH20TokenCalls::UpgradedAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for USDT_ETH20TokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                USDT_ETH20TokenCalls::MaxUint(element) => element.fmt(f),
                USDT_ETH20TokenCalls::_TotalSupply(element) => element.fmt(f),
                USDT_ETH20TokenCalls::AddBlackList(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Allowance(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Allowed(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Approve(element) => element.fmt(f),
                USDT_ETH20TokenCalls::BalanceOf(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Balances(element) => element.fmt(f),
                USDT_ETH20TokenCalls::BasisPointsRate(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Decimals(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Deprecate(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Deprecated(element) => element.fmt(f),
                USDT_ETH20TokenCalls::DestroyBlackFunds(element) => element.fmt(f),
                USDT_ETH20TokenCalls::GetBlackListStatus(element) => element.fmt(f),
                USDT_ETH20TokenCalls::GetOwner(element) => element.fmt(f),
                USDT_ETH20TokenCalls::IsBlackListed(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Issue(element) => element.fmt(f),
                USDT_ETH20TokenCalls::MaximumFee(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Name(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Owner(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Pause(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Paused(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Redeem(element) => element.fmt(f),
                USDT_ETH20TokenCalls::RemoveBlackList(element) => element.fmt(f),
                USDT_ETH20TokenCalls::SetParams(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Symbol(element) => element.fmt(f),
                USDT_ETH20TokenCalls::TotalSupply(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Transfer(element) => element.fmt(f),
                USDT_ETH20TokenCalls::TransferFrom(element) => element.fmt(f),
                USDT_ETH20TokenCalls::TransferOwnership(element) => element.fmt(f),
                USDT_ETH20TokenCalls::Unpause(element) => element.fmt(f),
                USDT_ETH20TokenCalls::UpgradedAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxUintCall> for USDT_ETH20TokenCalls {
        fn from(var: MaxUintCall) -> Self {
            USDT_ETH20TokenCalls::MaxUint(var)
        }
    }
    impl ::std::convert::From<_TotalSupplyCall> for USDT_ETH20TokenCalls {
        fn from(var: _TotalSupplyCall) -> Self {
            USDT_ETH20TokenCalls::_TotalSupply(var)
        }
    }
    impl ::std::convert::From<AddBlackListCall> for USDT_ETH20TokenCalls {
        fn from(var: AddBlackListCall) -> Self {
            USDT_ETH20TokenCalls::AddBlackList(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for USDT_ETH20TokenCalls {
        fn from(var: AllowanceCall) -> Self {
            USDT_ETH20TokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<AllowedCall> for USDT_ETH20TokenCalls {
        fn from(var: AllowedCall) -> Self {
            USDT_ETH20TokenCalls::Allowed(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for USDT_ETH20TokenCalls {
        fn from(var: ApproveCall) -> Self {
            USDT_ETH20TokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for USDT_ETH20TokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            USDT_ETH20TokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalancesCall> for USDT_ETH20TokenCalls {
        fn from(var: BalancesCall) -> Self {
            USDT_ETH20TokenCalls::Balances(var)
        }
    }
    impl ::std::convert::From<BasisPointsRateCall> for USDT_ETH20TokenCalls {
        fn from(var: BasisPointsRateCall) -> Self {
            USDT_ETH20TokenCalls::BasisPointsRate(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for USDT_ETH20TokenCalls {
        fn from(var: DecimalsCall) -> Self {
            USDT_ETH20TokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DeprecateCall> for USDT_ETH20TokenCalls {
        fn from(var: DeprecateCall) -> Self {
            USDT_ETH20TokenCalls::Deprecate(var)
        }
    }
    impl ::std::convert::From<DeprecatedCall> for USDT_ETH20TokenCalls {
        fn from(var: DeprecatedCall) -> Self {
            USDT_ETH20TokenCalls::Deprecated(var)
        }
    }
    impl ::std::convert::From<DestroyBlackFundsCall> for USDT_ETH20TokenCalls {
        fn from(var: DestroyBlackFundsCall) -> Self {
            USDT_ETH20TokenCalls::DestroyBlackFunds(var)
        }
    }
    impl ::std::convert::From<GetBlackListStatusCall> for USDT_ETH20TokenCalls {
        fn from(var: GetBlackListStatusCall) -> Self {
            USDT_ETH20TokenCalls::GetBlackListStatus(var)
        }
    }
    impl ::std::convert::From<GetOwnerCall> for USDT_ETH20TokenCalls {
        fn from(var: GetOwnerCall) -> Self {
            USDT_ETH20TokenCalls::GetOwner(var)
        }
    }
    impl ::std::convert::From<IsBlackListedCall> for USDT_ETH20TokenCalls {
        fn from(var: IsBlackListedCall) -> Self {
            USDT_ETH20TokenCalls::IsBlackListed(var)
        }
    }
    impl ::std::convert::From<IssueCall> for USDT_ETH20TokenCalls {
        fn from(var: IssueCall) -> Self {
            USDT_ETH20TokenCalls::Issue(var)
        }
    }
    impl ::std::convert::From<MaximumFeeCall> for USDT_ETH20TokenCalls {
        fn from(var: MaximumFeeCall) -> Self {
            USDT_ETH20TokenCalls::MaximumFee(var)
        }
    }
    impl ::std::convert::From<NameCall> for USDT_ETH20TokenCalls {
        fn from(var: NameCall) -> Self {
            USDT_ETH20TokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for USDT_ETH20TokenCalls {
        fn from(var: OwnerCall) -> Self {
            USDT_ETH20TokenCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PauseCall> for USDT_ETH20TokenCalls {
        fn from(var: PauseCall) -> Self {
            USDT_ETH20TokenCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for USDT_ETH20TokenCalls {
        fn from(var: PausedCall) -> Self {
            USDT_ETH20TokenCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for USDT_ETH20TokenCalls {
        fn from(var: RedeemCall) -> Self {
            USDT_ETH20TokenCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RemoveBlackListCall> for USDT_ETH20TokenCalls {
        fn from(var: RemoveBlackListCall) -> Self {
            USDT_ETH20TokenCalls::RemoveBlackList(var)
        }
    }
    impl ::std::convert::From<SetParamsCall> for USDT_ETH20TokenCalls {
        fn from(var: SetParamsCall) -> Self {
            USDT_ETH20TokenCalls::SetParams(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for USDT_ETH20TokenCalls {
        fn from(var: SymbolCall) -> Self {
            USDT_ETH20TokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for USDT_ETH20TokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            USDT_ETH20TokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for USDT_ETH20TokenCalls {
        fn from(var: TransferCall) -> Self {
            USDT_ETH20TokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for USDT_ETH20TokenCalls {
        fn from(var: TransferFromCall) -> Self {
            USDT_ETH20TokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for USDT_ETH20TokenCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            USDT_ETH20TokenCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for USDT_ETH20TokenCalls {
        fn from(var: UnpauseCall) -> Self {
            USDT_ETH20TokenCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UpgradedAddressCall> for USDT_ETH20TokenCalls {
        fn from(var: UpgradedAddressCall) -> Self {
            USDT_ETH20TokenCalls::UpgradedAddress(var)
        }
    }
    ///Container type for all return fields from the `MAX_UINT` function with signature `MAX_UINT()` and selector `0xe5b5019a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct MaxUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_totalSupply` function with signature `_totalSupply()` and selector `0x3eaaf86b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct _TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AllowanceReturn {
        pub remaining: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `allowed` function with signature `allowed(address,address)` and selector `0x5c658165`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `basisPointsRate` function with signature `basisPointsRate()` and selector `0xdd644f72`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BasisPointsRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deprecated` function with signature `deprecated()` and selector `0x0e136b19`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DeprecatedReturn(pub bool);
    ///Container type for all return fields from the `getBlackListStatus` function with signature `getBlackListStatus(address)` and selector `0x59bf1abe`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetBlackListStatusReturn(pub bool);
    ///Container type for all return fields from the `getOwner` function with signature `getOwner()` and selector `0x893d20e8`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isBlackListed` function with signature `isBlackListed(address)` and selector `0xe47d6060`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct IsBlackListedReturn(pub bool);
    ///Container type for all return fields from the `maximumFee` function with signature `maximumFee()` and selector `0x35390714`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct MaximumFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `upgradedAddress` function with signature `upgradedAddress()` and selector `0x26976e3f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct UpgradedAddressReturn(pub ::ethers::core::types::Address);
}
