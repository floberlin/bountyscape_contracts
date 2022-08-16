pub use contract_script::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod contract_script {
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
    #[doc = "ContractScript was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONTRACTSCRIPT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONTRACTSCRIPT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805460ff1916600117905534801561001d57600080fd5b5061014c8061002d6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630a9254e4146100515780633a76846314610053578063c04062261461008b578063f8ccbf4714610093575b600080fd5b005b61006e737109709ecfa91a80626ff3989d68f67f5b1dd12d81565b6040516001600160a01b0390911681526020015b60405180910390f35b6100516100b0565b6000546100a09060ff1681565b6040519015158152602001610082565b604080516302bf260160e61b81529051737109709ecfa91a80626ff3989d68f67f5b1dd12d9163afc9804091600480830192600092919082900301818387803b1580156100fc57600080fd5b505af1158015610110573d6000803e3d6000fd5b5050505056fea2646970667358221220ec9eaee5e041861ce33c15b9b5ff71560d7da4cc91b205e0ffa088bc4887e4d564736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ContractScript<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ContractScript<M> {
        fn clone(&self) -> Self {
            ContractScript(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ContractScript<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ContractScript<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ContractScript))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ContractScript<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CONTRACTSCRIPT_ABI.clone(), client)
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
                CONTRACTSCRIPT_ABI.clone(),
                CONTRACTSCRIPT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `run` (0xc0406226) function"]
        pub fn run(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ContractScript<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    #[doc = "Container type for all input parameters for the `run` function with signature `run()` and selector `[192, 64, 98, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
    #[doc = "Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[doc = "Container type for all input parameters for the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vm", abi = "vm()")]
    pub struct VmCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ContractScriptCalls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for ContractScriptCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractScriptCalls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ContractScriptCalls::Run(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractScriptCalls::SetUp(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ContractScriptCalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ContractScriptCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ContractScriptCalls::IsScript(element) => element.encode(),
                ContractScriptCalls::Run(element) => element.encode(),
                ContractScriptCalls::SetUp(element) => element.encode(),
                ContractScriptCalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ContractScriptCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ContractScriptCalls::IsScript(element) => element.fmt(f),
                ContractScriptCalls::Run(element) => element.fmt(f),
                ContractScriptCalls::SetUp(element) => element.fmt(f),
                ContractScriptCalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for ContractScriptCalls {
        fn from(var: IsScriptCall) -> Self {
            ContractScriptCalls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for ContractScriptCalls {
        fn from(var: RunCall) -> Self {
            ContractScriptCalls::Run(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for ContractScriptCalls {
        fn from(var: SetUpCall) -> Self {
            ContractScriptCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<VmCall> for ContractScriptCalls {
        fn from(var: VmCall) -> Self {
            ContractScriptCalls::Vm(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsScriptReturn(pub bool);
    #[doc = "Container type for all return fields from the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VmReturn(pub ethers::core::types::Address);
}
