pub use ierc1155_receiver::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ierc1155_receiver {
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
    #[doc = "IERC1155Receiver was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IERC1155RECEIVER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IERC1155Receiver<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IERC1155Receiver<M> {
        fn clone(&self) -> Self {
            IERC1155Receiver(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IERC1155Receiver<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IERC1155Receiver<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IERC1155Receiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IERC1155Receiver<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IERC1155RECEIVER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function"]
        pub fn on_erc1155_batch_received(
            &self,
            operator: ethers::core::types::Address,
            from: ethers::core::types::Address,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (operator, from, ids, values, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC1155Received` (0xf23a6e61) function"]
        pub fn on_erc1155_received(
            &self,
            operator: ethers::core::types::Address,
            from: ethers::core::types::Address,
            id: ethers::core::types::U256,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (operator, from, id, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IERC1155Receiver<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `[188, 25, 124, 129]`"]
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall {
        pub operator: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `[242, 58, 110, 97]`"]
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall {
        pub operator: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IERC1155ReceiverCalls {
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ethers::core::abi::AbiDecode for IERC1155ReceiverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC1155ReceiverCalls::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC1155ReceiverCalls::OnERC1155Received(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC1155ReceiverCalls::SupportsInterface(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IERC1155ReceiverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IERC1155ReceiverCalls::OnERC1155BatchReceived(element) => element.encode(),
                IERC1155ReceiverCalls::OnERC1155Received(element) => element.encode(),
                IERC1155ReceiverCalls::SupportsInterface(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IERC1155ReceiverCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IERC1155ReceiverCalls::OnERC1155BatchReceived(element) => element.fmt(f),
                IERC1155ReceiverCalls::OnERC1155Received(element) => element.fmt(f),
                IERC1155ReceiverCalls::SupportsInterface(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<OnERC1155BatchReceivedCall> for IERC1155ReceiverCalls {
        fn from(var: OnERC1155BatchReceivedCall) -> Self {
            IERC1155ReceiverCalls::OnERC1155BatchReceived(var)
        }
    }
    impl ::std::convert::From<OnERC1155ReceivedCall> for IERC1155ReceiverCalls {
        fn from(var: OnERC1155ReceivedCall) -> Self {
            IERC1155ReceiverCalls::OnERC1155Received(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for IERC1155ReceiverCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            IERC1155ReceiverCalls::SupportsInterface(var)
        }
    }
    #[doc = "Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `[188, 25, 124, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `[242, 58, 110, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
