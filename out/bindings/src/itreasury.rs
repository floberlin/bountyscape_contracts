pub use itreasury::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod itreasury {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Itreasury was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ITREASURY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"contractor\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawReward\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct Itreasury<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Itreasury<M> {
        fn clone(&self) -> Self {
            Itreasury(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Itreasury<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Itreasury<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Itreasury))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Itreasury<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ITREASURY_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `whitelist` (0x98fc55d8) function"]
        pub fn whitelist(
            &self,
            addr: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 252, 85, 216], (addr, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xf3fef3a3) function"]
        pub fn withdraw(
            &self,
            addr: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 254, 243, 163], (addr, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawReward` (0x3eb1fdc6) function"]
        pub fn withdraw_reward(
            &self,
            contractor: ethers::core::types::Address,
            id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 177, 253, 198], (contractor, id, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Itreasury<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `whitelist` function with signature `whitelist(address,uint256)` and selector `[152, 252, 85, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelist", abi = "whitelist(address,uint256)")]
    pub struct WhitelistCall {
        pub addr: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256)` and selector `[243, 254, 243, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256)")]
    pub struct WithdrawCall {
        pub addr: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdrawReward` function with signature `withdrawReward(address,uint256,uint256)` and selector `[62, 177, 253, 198]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawReward",
        abi = "withdrawReward(address,uint256,uint256)"
    )]
    pub struct WithdrawRewardCall {
        pub contractor: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ItreasuryCalls {
        Whitelist(WhitelistCall),
        Withdraw(WithdrawCall),
        WithdrawReward(WithdrawRewardCall),
    }
    impl ethers::core::abi::AbiDecode for ItreasuryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <WhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ItreasuryCalls::Whitelist(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ItreasuryCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ItreasuryCalls::WithdrawReward(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ItreasuryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ItreasuryCalls::Whitelist(element) => element.encode(),
                ItreasuryCalls::Withdraw(element) => element.encode(),
                ItreasuryCalls::WithdrawReward(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ItreasuryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ItreasuryCalls::Whitelist(element) => element.fmt(f),
                ItreasuryCalls::Withdraw(element) => element.fmt(f),
                ItreasuryCalls::WithdrawReward(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<WhitelistCall> for ItreasuryCalls {
        fn from(var: WhitelistCall) -> Self {
            ItreasuryCalls::Whitelist(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ItreasuryCalls {
        fn from(var: WithdrawCall) -> Self {
            ItreasuryCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawRewardCall> for ItreasuryCalls {
        fn from(var: WithdrawRewardCall) -> Self {
            ItreasuryCalls::WithdrawReward(var)
        }
    }
}
