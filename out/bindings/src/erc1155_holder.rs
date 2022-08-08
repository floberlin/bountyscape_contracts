pub use erc1155_holder::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc1155_holder {
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
    #[doc = "ERC1155Holder was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC1155HOLDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC1155HOLDER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506103c5806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806301ffc9a714610046578063bc197c811461006e578063f23a6e61146100a6575b600080fd5b6100596100543660046100fc565b6100c5565b60405190151581526020015b60405180910390f35b61008d61007c366004610280565b63bc197c8160e01b95945050505050565b6040516001600160e01b03199091168152602001610065565b61008d6100b436600461032a565b63f23a6e6160e01b95945050505050565b60006001600160e01b03198216630271189760e51b14806100f657506301ffc9a760e01b6001600160e01b03198316145b92915050565b60006020828403121561010e57600080fd5b81356001600160e01b03198116811461012657600080fd5b9392505050565b80356001600160a01b038116811461014457600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561018857610188610149565b604052919050565b600082601f8301126101a157600080fd5b8135602067ffffffffffffffff8211156101bd576101bd610149565b8160051b6101cc82820161015f565b92835284810182019282810190878511156101e657600080fd5b83870192505b84831015610205578235825291830191908301906101ec565b979650505050505050565b600082601f83011261022157600080fd5b813567ffffffffffffffff81111561023b5761023b610149565b61024e601f8201601f191660200161015f565b81815284602083860101111561026357600080fd5b816020850160208301376000918101602001919091529392505050565b600080600080600060a0868803121561029857600080fd5b6102a18661012d565b94506102af6020870161012d565b9350604086013567ffffffffffffffff808211156102cc57600080fd5b6102d889838a01610190565b945060608801359150808211156102ee57600080fd5b6102fa89838a01610190565b9350608088013591508082111561031057600080fd5b5061031d88828901610210565b9150509295509295909350565b600080600080600060a0868803121561034257600080fd5b61034b8661012d565b94506103596020870161012d565b93506040860135925060608601359150608086013567ffffffffffffffff81111561038357600080fd5b61031d8882890161021056fea2646970667358221220d6b9a4bbba2237dadf1c7ba942b4a9488df0f84a9007bfcb78aa3dd19b7736c964736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ERC1155Holder<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC1155Holder<M> {
        fn clone(&self) -> Self {
            ERC1155Holder(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC1155Holder<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC1155Holder<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC1155Holder))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC1155Holder<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC1155HOLDER_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                ERC1155HOLDER_ABI.clone(),
                ERC1155HOLDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function"]
        pub fn on_erc1155_batch_received(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ::std::vec::Vec<ethers::core::types::U256>,
            p3: ::std::vec::Vec<ethers::core::types::U256>,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC1155Received` (0xf23a6e61) function"]
        pub fn on_erc1155_received(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC1155Holder<M> {
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
    pub struct OnERC1155BatchReceivedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ::std::vec::Vec<ethers::core::types::U256>,
        pub ::std::vec::Vec<ethers::core::types::U256>,
        pub ethers::core::types::Bytes,
    );
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
    pub struct OnERC1155ReceivedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
    );
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
    pub enum ERC1155HolderCalls {
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ethers::core::abi::AbiDecode for ERC1155HolderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155HolderCalls::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155HolderCalls::OnERC1155Received(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155HolderCalls::SupportsInterface(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC1155HolderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC1155HolderCalls::OnERC1155BatchReceived(element) => element.encode(),
                ERC1155HolderCalls::OnERC1155Received(element) => element.encode(),
                ERC1155HolderCalls::SupportsInterface(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC1155HolderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC1155HolderCalls::OnERC1155BatchReceived(element) => element.fmt(f),
                ERC1155HolderCalls::OnERC1155Received(element) => element.fmt(f),
                ERC1155HolderCalls::SupportsInterface(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<OnERC1155BatchReceivedCall> for ERC1155HolderCalls {
        fn from(var: OnERC1155BatchReceivedCall) -> Self {
            ERC1155HolderCalls::OnERC1155BatchReceived(var)
        }
    }
    impl ::std::convert::From<OnERC1155ReceivedCall> for ERC1155HolderCalls {
        fn from(var: OnERC1155ReceivedCall) -> Self {
            ERC1155HolderCalls::OnERC1155Received(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ERC1155HolderCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ERC1155HolderCalls::SupportsInterface(var)
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
