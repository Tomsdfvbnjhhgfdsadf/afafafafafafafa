pub use checker_abi::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod checker_abi {
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
    ///CHECKER_ABI was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"constant\":true,\"inputs\":[{\"name\":\"user\",\"type\":\"address\"},{\"name\":\"token\",\"type\":\"address\"}],\"name\":\"tokenBalance\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"users\",\"type\":\"address[]\"},{\"name\":\"tokens\",\"type\":\"address[]\"}],\"name\":\"balances\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"payable\":true,\"stateMutability\":\"payable\",\"type\":\"fallback\"}]";
    /// The parsed JSON-ABI of the contract.
    pub static CHECKER_ABI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct CHECKER_ABI<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for CHECKER_ABI<M> {
        fn clone(&self) -> Self {
            CHECKER_ABI(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CHECKER_ABI<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CHECKER_ABI<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CHECKER_ABI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CHECKER_ABI<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                CHECKER_ABI_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `balances` (0xf0002ea9) function
        pub fn balances(
            &self,
            users: ::std::vec::Vec<::ethers::core::types::Address>,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([240, 0, 46, 169], (users, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenBalance` (0x1049334f) function
        pub fn token_balance(
            &self,
            user: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([16, 73, 51, 79], (user, token))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for CHECKER_ABI<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address[],address[])` and selector `0xf0002ea9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balances", abi = "balances(address[],address[])")]
    pub struct BalancesCall {
        pub users: ::std::vec::Vec<::ethers::core::types::Address>,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `tokenBalance` function with signature `tokenBalance(address,address)` and selector `0x1049334f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenBalance", abi = "tokenBalance(address,address)")]
    pub struct TokenBalanceCall {
        pub user: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum CHECKER_ABICalls {
        Balances(BalancesCall),
        TokenBalance(TokenBalanceCall),
    }
    impl ::ethers::core::abi::AbiDecode for CHECKER_ABICalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CHECKER_ABICalls::Balances(decoded));
            }
            if let Ok(decoded) =
                <TokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CHECKER_ABICalls::TokenBalance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CHECKER_ABICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CHECKER_ABICalls::Balances(element) => element.encode(),
                CHECKER_ABICalls::TokenBalance(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CHECKER_ABICalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CHECKER_ABICalls::Balances(element) => element.fmt(f),
                CHECKER_ABICalls::TokenBalance(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalancesCall> for CHECKER_ABICalls {
        fn from(var: BalancesCall) -> Self {
            CHECKER_ABICalls::Balances(var)
        }
    }
    impl ::std::convert::From<TokenBalanceCall> for CHECKER_ABICalls {
        fn from(var: TokenBalanceCall) -> Self {
            CHECKER_ABICalls::TokenBalance(var)
        }
    }
    ///Container type for all return fields from the `balances` function with signature `balances(address[],address[])` and selector `0xf0002ea9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalancesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `tokenBalance` function with signature `tokenBalance(address,address)` and selector `0x1049334f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TokenBalanceReturn(pub ::ethers::core::types::U256);
}
