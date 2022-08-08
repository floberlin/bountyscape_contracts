pub use bountyscape_tresuary::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod bountyscape_tresuary {
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
    #[doc = "BountyscapeTresuary was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BOUNTYSCAPETRESUARY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BountyscapeAddr\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"addressIdMapping\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBountyscape\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"contractor\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"withdrawReward\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOUNTYSCAPETRESUARY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061001a33610028565b61002333610028565b610078565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6109ab806100876000396000f3fe6080604052600436106100ab5760003560e01c80638da5cb5b116100645780638da5cb5b146101a457806398fc55d8146101c2578063bc197c81146101e2578063f23a6e6114610227578063f2fde38b14610253578063f3fef3a31461027357600080fd5b806301ffc9a7146100b75780633eb1fdc6146100ec5780633ffd91801461010157806357ef63cb14610139578063715018a61461016f5780637524e83c1461018457600080fd5b366100b257005b600080fd5b3480156100c357600080fd5b506100d76100d2366004610651565b610286565b60405190151581526020015b60405180910390f35b6100ff6100fa36600461069e565b6102bd565b005b34801561010d57600080fd5b50600154610121906001600160a01b031681565b6040516001600160a01b0390911681526020016100e3565b34801561014557600080fd5b506101216101543660046106d1565b6002602052600090815260409020546001600160a01b031681565b34801561017b57600080fd5b506100ff610367565b34801561019057600080fd5b506100ff61019f3660046106ea565b61037b565b3480156101b057600080fd5b506000546001600160a01b0316610121565b3480156101ce57600080fd5b506100ff6101dd366004610705565b6103a5565b3480156101ee57600080fd5b5061020e6101fd366004610866565b63bc197c8160e01b95945050505050565b6040516001600160e01b031990911681526020016100e3565b34801561023357600080fd5b5061020e610242366004610910565b63f23a6e6160e01b95945050505050565b34801561025f57600080fd5b506100ff61026e3660046106ea565b61043f565b6100ff610281366004610705565b6104b8565b60006001600160e01b03198216630271189760e51b14806102b757506301ffc9a760e01b6001600160e01b03198316145b92915050565b6000828152600260205260409020546001600160a01b0384811691161461032b5760405162461bcd60e51b815260206004820152601a60248201527f43616c6c6572206973206e6f742077686974656c69737465642100000000000060448201526064015b60405180910390fd5b6040516001600160a01b0384169082156108fc029083906000818181858888f19350505050158015610361573d6000803e3d6000fd5b50505050565b61036f6105a7565b6103796000610601565b565b6103836105a7565b600180546001600160a01b0319166001600160a01b0392909216919091179055565b6001546001600160a01b031633146104115760405162461bcd60e51b815260206004820152602960248201527f43616c6c6572206973206e6f7420426f756e7479736361706520736d61727420604482015268636f6e74726163742160b81b6064820152608401610322565b600090815260026020526040902080546001600160a01b0319166001600160a01b0392909216919091179055565b6104476105a7565b6001600160a01b0381166104ac5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610322565b6104b581610601565b50565b6000818152600260205260409020546001600160a01b038381169116146105215760405162461bcd60e51b815260206004820152601a60248201527f43616c6c6572206973206e6f742077686974656c6973746564210000000000006044820152606401610322565b60018054604051637921219560e11b81523060048201526001600160a01b03858116602483015260448201859052606482019390935260a06084820152600060a482015291169063f242432a9060c401600060405180830381600087803b15801561058b57600080fd5b505af115801561059f573d6000803e3d6000fd5b505050505050565b6000546001600160a01b031633146103795760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610322565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b60006020828403121561066357600080fd5b81356001600160e01b03198116811461067b57600080fd5b9392505050565b80356001600160a01b038116811461069957600080fd5b919050565b6000806000606084860312156106b357600080fd5b6106bc84610682565b95602085013595506040909401359392505050565b6000602082840312156106e357600080fd5b5035919050565b6000602082840312156106fc57600080fd5b61067b82610682565b6000806040838503121561071857600080fd5b61072183610682565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561076e5761076e61072f565b604052919050565b600082601f83011261078757600080fd5b8135602067ffffffffffffffff8211156107a3576107a361072f565b8160051b6107b2828201610745565b92835284810182019282810190878511156107cc57600080fd5b83870192505b848310156107eb578235825291830191908301906107d2565b979650505050505050565b600082601f83011261080757600080fd5b813567ffffffffffffffff8111156108215761082161072f565b610834601f8201601f1916602001610745565b81815284602083860101111561084957600080fd5b816020850160208301376000918101602001919091529392505050565b600080600080600060a0868803121561087e57600080fd5b61088786610682565b945061089560208701610682565b9350604086013567ffffffffffffffff808211156108b257600080fd5b6108be89838a01610776565b945060608801359150808211156108d457600080fd5b6108e089838a01610776565b935060808801359150808211156108f657600080fd5b50610903888289016107f6565b9150509295509295909350565b600080600080600060a0868803121561092857600080fd5b61093186610682565b945061093f60208701610682565b93506040860135925060608601359150608086013567ffffffffffffffff81111561096957600080fd5b610903888289016107f656fea26469706673582212200a1e1c6fb52f97ff25f0c93a9edbb9abe880917a825a4ad7912d9adc10c36ecd64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct BountyscapeTresuary<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BountyscapeTresuary<M> {
        fn clone(&self) -> Self {
            BountyscapeTresuary(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BountyscapeTresuary<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BountyscapeTresuary<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BountyscapeTresuary))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BountyscapeTresuary<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BOUNTYSCAPETRESUARY_ABI.clone(), client)
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
                BOUNTYSCAPETRESUARY_ABI.clone(),
                BOUNTYSCAPETRESUARY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `BountyscapeAddr` (0x3ffd9180) function"]
        pub fn bountyscape_addr(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([63, 253, 145, 128], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addressIdMapping` (0x57ef63cb) function"]
        pub fn address_id_mapping(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([87, 239, 99, 203], p0)
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBountyscape` (0x7524e83c) function"]
        pub fn set_bountyscape(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 36, 232, 60], addr)
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
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
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
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 254, 243, 163], (addr, id))
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
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for BountyscapeTresuary<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `BountyscapeAddr` function with signature `BountyscapeAddr()` and selector `[63, 253, 145, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BountyscapeAddr", abi = "BountyscapeAddr()")]
    pub struct BountyscapeAddrCall;
    #[doc = "Container type for all input parameters for the `addressIdMapping` function with signature `addressIdMapping(uint256)` and selector `[87, 239, 99, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addressIdMapping", abi = "addressIdMapping(uint256)")]
    pub struct AddressIdMappingCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setBountyscape` function with signature `setBountyscape(address)` and selector `[117, 36, 232, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBountyscape", abi = "setBountyscape(address)")]
    pub struct SetBountyscapeCall {
        pub addr: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
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
        pub id: ethers::core::types::U256,
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
    pub enum BountyscapeTresuaryCalls {
        BountyscapeAddr(BountyscapeAddrCall),
        AddressIdMapping(AddressIdMappingCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetBountyscape(SetBountyscapeCall),
        SupportsInterface(SupportsInterfaceCall),
        TransferOwnership(TransferOwnershipCall),
        Whitelist(WhitelistCall),
        Withdraw(WithdrawCall),
        WithdrawReward(WithdrawRewardCall),
    }
    impl ethers::core::abi::AbiDecode for BountyscapeTresuaryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BountyscapeAddrCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::BountyscapeAddr(decoded));
            }
            if let Ok(decoded) =
                <AddressIdMappingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::AddressIdMapping(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::OnERC1155Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetBountyscapeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::SetBountyscape(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::Whitelist(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BountyscapeTresuaryCalls::WithdrawReward(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BountyscapeTresuaryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BountyscapeTresuaryCalls::BountyscapeAddr(element) => element.encode(),
                BountyscapeTresuaryCalls::AddressIdMapping(element) => element.encode(),
                BountyscapeTresuaryCalls::OnERC1155BatchReceived(element) => element.encode(),
                BountyscapeTresuaryCalls::OnERC1155Received(element) => element.encode(),
                BountyscapeTresuaryCalls::Owner(element) => element.encode(),
                BountyscapeTresuaryCalls::RenounceOwnership(element) => element.encode(),
                BountyscapeTresuaryCalls::SetBountyscape(element) => element.encode(),
                BountyscapeTresuaryCalls::SupportsInterface(element) => element.encode(),
                BountyscapeTresuaryCalls::TransferOwnership(element) => element.encode(),
                BountyscapeTresuaryCalls::Whitelist(element) => element.encode(),
                BountyscapeTresuaryCalls::Withdraw(element) => element.encode(),
                BountyscapeTresuaryCalls::WithdrawReward(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BountyscapeTresuaryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BountyscapeTresuaryCalls::BountyscapeAddr(element) => element.fmt(f),
                BountyscapeTresuaryCalls::AddressIdMapping(element) => element.fmt(f),
                BountyscapeTresuaryCalls::OnERC1155BatchReceived(element) => element.fmt(f),
                BountyscapeTresuaryCalls::OnERC1155Received(element) => element.fmt(f),
                BountyscapeTresuaryCalls::Owner(element) => element.fmt(f),
                BountyscapeTresuaryCalls::RenounceOwnership(element) => element.fmt(f),
                BountyscapeTresuaryCalls::SetBountyscape(element) => element.fmt(f),
                BountyscapeTresuaryCalls::SupportsInterface(element) => element.fmt(f),
                BountyscapeTresuaryCalls::TransferOwnership(element) => element.fmt(f),
                BountyscapeTresuaryCalls::Whitelist(element) => element.fmt(f),
                BountyscapeTresuaryCalls::Withdraw(element) => element.fmt(f),
                BountyscapeTresuaryCalls::WithdrawReward(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BountyscapeAddrCall> for BountyscapeTresuaryCalls {
        fn from(var: BountyscapeAddrCall) -> Self {
            BountyscapeTresuaryCalls::BountyscapeAddr(var)
        }
    }
    impl ::std::convert::From<AddressIdMappingCall> for BountyscapeTresuaryCalls {
        fn from(var: AddressIdMappingCall) -> Self {
            BountyscapeTresuaryCalls::AddressIdMapping(var)
        }
    }
    impl ::std::convert::From<OnERC1155BatchReceivedCall> for BountyscapeTresuaryCalls {
        fn from(var: OnERC1155BatchReceivedCall) -> Self {
            BountyscapeTresuaryCalls::OnERC1155BatchReceived(var)
        }
    }
    impl ::std::convert::From<OnERC1155ReceivedCall> for BountyscapeTresuaryCalls {
        fn from(var: OnERC1155ReceivedCall) -> Self {
            BountyscapeTresuaryCalls::OnERC1155Received(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BountyscapeTresuaryCalls {
        fn from(var: OwnerCall) -> Self {
            BountyscapeTresuaryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for BountyscapeTresuaryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            BountyscapeTresuaryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetBountyscapeCall> for BountyscapeTresuaryCalls {
        fn from(var: SetBountyscapeCall) -> Self {
            BountyscapeTresuaryCalls::SetBountyscape(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for BountyscapeTresuaryCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            BountyscapeTresuaryCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for BountyscapeTresuaryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            BountyscapeTresuaryCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WhitelistCall> for BountyscapeTresuaryCalls {
        fn from(var: WhitelistCall) -> Self {
            BountyscapeTresuaryCalls::Whitelist(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for BountyscapeTresuaryCalls {
        fn from(var: WithdrawCall) -> Self {
            BountyscapeTresuaryCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawRewardCall> for BountyscapeTresuaryCalls {
        fn from(var: WithdrawRewardCall) -> Self {
            BountyscapeTresuaryCalls::WithdrawReward(var)
        }
    }
    #[doc = "Container type for all return fields from the `BountyscapeAddr` function with signature `BountyscapeAddr()` and selector `[63, 253, 145, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BountyscapeAddrReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `addressIdMapping` function with signature `addressIdMapping(uint256)` and selector `[87, 239, 99, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddressIdMappingReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
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
