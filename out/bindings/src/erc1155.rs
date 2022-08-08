pub use erc1155::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc1155 {
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
    #[doc = "ERC1155 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC1155_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"uri_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferSingle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"URI\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"accounts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfBatch\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeBatchTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"uri\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC1155_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b506040516200163d3803806200163d83398101604081905262000034916200006e565b6200003f8162000046565b50620002a5565b6002620000548282620001d9565b5050565b634e487b7160e01b600052604160045260246000fd5b600060208083850312156200008257600080fd5b82516001600160401b03808211156200009a57600080fd5b818501915085601f830112620000af57600080fd5b815181811115620000c457620000c462000058565b604051601f8201601f19908116603f01168101908382118183101715620000ef57620000ef62000058565b8160405282815288868487010111156200010857600080fd5b600093505b828410156200012c57848401860151818501870152928501926200010d565b828411156200013e5760008684830101525b98975050505050505050565b600181811c908216806200015f57607f821691505b6020821081036200018057634e487b7160e01b600052602260045260246000fd5b50919050565b601f821115620001d457600081815260208120601f850160051c81016020861015620001af5750805b601f850160051c820191505b81811015620001d057828155600101620001bb565b5050505b505050565b81516001600160401b03811115620001f557620001f562000058565b6200020d816200020684546200014a565b8462000186565b602080601f8311600181146200024557600084156200022c5750858301515b600019600386901b1c1916600185901b178555620001d0565b600085815260208120601f198616915b82811015620002765788860151825594840194600190910190840162000255565b5085821015620002955787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b61138880620002b56000396000f3fe608060405234801561001057600080fd5b50600436106100875760003560e01c80634e1273f41161005b5780634e1273f41461010a578063a22cb4651461012a578063e985e9c51461013d578063f242432a1461017957600080fd5b8062fdd58e1461008c57806301ffc9a7146100b25780630e89341c146100d55780632eb2c2d6146100f5575b600080fd5b61009f61009a366004610b3f565b61018c565b6040519081526020015b60405180910390f35b6100c56100c0366004610b82565b610222565b60405190151581526020016100a9565b6100e86100e3366004610ba6565b610274565b6040516100a99190610c0c565b610108610103366004610d6b565b610308565b005b61011d610118366004610e15565b610354565b6040516100a99190610f1b565b610108610138366004610f2e565b61047e565b6100c561014b366004610f6a565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205460ff1690565b610108610187366004610f9d565b61048d565b60006001600160a01b0383166101fc5760405162461bcd60e51b815260206004820152602a60248201527f455243313135353a2061646472657373207a65726f206973206e6f742061207660448201526930b634b21037bbb732b960b11b60648201526084015b60405180910390fd5b506000908152602081815260408083206001600160a01b03949094168352929052205490565b60006001600160e01b03198216636cdb3d1360e11b148061025357506001600160e01b031982166303a24d0760e21b145b8061026e57506301ffc9a760e01b6001600160e01b03198316145b92915050565b60606002805461028390611002565b80601f01602080910402602001604051908101604052809291908181526020018280546102af90611002565b80156102fc5780601f106102d1576101008083540402835291602001916102fc565b820191906000526020600020905b8154815290600101906020018083116102df57829003601f168201915b50505050509050919050565b6001600160a01b0385163314806103245750610324853361014b565b6103405760405162461bcd60e51b81526004016101f39061103c565b61034d85858585856104d2565b5050505050565b606081518351146103b95760405162461bcd60e51b815260206004820152602960248201527f455243313135353a206163636f756e747320616e6420696473206c656e677468604482015268040dad2e6dac2e8c6d60bb1b60648201526084016101f3565b6000835167ffffffffffffffff8111156103d5576103d5610c1f565b6040519080825280602002602001820160405280156103fe578160200160208202803683370190505b50905060005b8451811015610476576104498582815181106104225761042261108a565b602002602001015185838151811061043c5761043c61108a565b602002602001015161018c565b82828151811061045b5761045b61108a565b602090810291909101015261046f816110b6565b9050610404565b509392505050565b6104893383836106af565b5050565b6001600160a01b0385163314806104a957506104a9853361014b565b6104c55760405162461bcd60e51b81526004016101f39061103c565b61034d858585858561078f565b81518351146105345760405162461bcd60e51b815260206004820152602860248201527f455243313135353a2069647320616e6420616d6f756e7473206c656e677468206044820152670dad2e6dac2e8c6d60c31b60648201526084016101f3565b6001600160a01b03841661055a5760405162461bcd60e51b81526004016101f3906110cf565b3360005b845181101561064157600085828151811061057b5761057b61108a565b6020026020010151905060008583815181106105995761059961108a565b602090810291909101810151600084815280835260408082206001600160a01b038e1683529093529190912054909150818110156105e95760405162461bcd60e51b81526004016101f390611114565b6000838152602081815260408083206001600160a01b038e8116855292528083208585039055908b1682528120805484929061062690849061115e565b925050819055505050508061063a906110b6565b905061055e565b50846001600160a01b0316866001600160a01b0316826001600160a01b03167f4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb8787604051610691929190611176565b60405180910390a46106a78187878787876108b9565b505050505050565b816001600160a01b0316836001600160a01b0316036107225760405162461bcd60e51b815260206004820152602960248201527f455243313135353a2073657474696e6720617070726f76616c20737461747573604482015268103337b91039b2b63360b91b60648201526084016101f3565b6001600160a01b03838116600081815260016020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31910160405180910390a3505050565b6001600160a01b0384166107b55760405162461bcd60e51b81526004016101f3906110cf565b3360006107c185610a1d565b905060006107ce85610a1d565b90506000868152602081815260408083206001600160a01b038c168452909152902054858110156108115760405162461bcd60e51b81526004016101f390611114565b6000878152602081815260408083206001600160a01b038d8116855292528083208985039055908a1682528120805488929061084e90849061115e565b909155505060408051888152602081018890526001600160a01b03808b16928c821692918816917fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62910160405180910390a46108ae848a8a8a8a8a610a68565b505050505050505050565b6001600160a01b0384163b156106a75760405163bc197c8160e01b81526001600160a01b0385169063bc197c81906108fd90899089908890889088906004016111a4565b6020604051808303816000875af1925050508015610938575060408051601f3d908101601f1916820190925261093591810190611202565b60015b6109e45761094461121f565b806308c379a00361097d575061095861123b565b80610963575061097f565b8060405162461bcd60e51b81526004016101f39190610c0c565b505b60405162461bcd60e51b815260206004820152603460248201527f455243313135353a207472616e7366657220746f206e6f6e2d455243313135356044820152732932b1b2b4bb32b91034b6b83632b6b2b73a32b960611b60648201526084016101f3565b6001600160e01b0319811663bc197c8160e01b14610a145760405162461bcd60e51b81526004016101f3906112c5565b50505050505050565b60408051600180825281830190925260609160009190602080830190803683370190505090508281600081518110610a5757610a5761108a565b602090810291909101015292915050565b6001600160a01b0384163b156106a75760405163f23a6e6160e01b81526001600160a01b0385169063f23a6e6190610aac908990899088908890889060040161130d565b6020604051808303816000875af1925050508015610ae7575060408051601f3d908101601f19168201909252610ae491810190611202565b60015b610af35761094461121f565b6001600160e01b0319811663f23a6e6160e01b14610a145760405162461bcd60e51b81526004016101f3906112c5565b80356001600160a01b0381168114610b3a57600080fd5b919050565b60008060408385031215610b5257600080fd5b610b5b83610b23565b946020939093013593505050565b6001600160e01b031981168114610b7f57600080fd5b50565b600060208284031215610b9457600080fd5b8135610b9f81610b69565b9392505050565b600060208284031215610bb857600080fd5b5035919050565b6000815180845260005b81811015610be557602081850181015186830182015201610bc9565b81811115610bf7576000602083870101525b50601f01601f19169290920160200192915050565b602081526000610b9f6020830184610bbf565b634e487b7160e01b600052604160045260246000fd5b601f8201601f1916810167ffffffffffffffff81118282101715610c5b57610c5b610c1f565b6040525050565b600067ffffffffffffffff821115610c7c57610c7c610c1f565b5060051b60200190565b600082601f830112610c9757600080fd5b81356020610ca482610c62565b604051610cb18282610c35565b83815260059390931b8501820192828101915086841115610cd157600080fd5b8286015b84811015610cec5780358352918301918301610cd5565b509695505050505050565b600082601f830112610d0857600080fd5b813567ffffffffffffffff811115610d2257610d22610c1f565b604051610d39601f8301601f191660200182610c35565b818152846020838601011115610d4e57600080fd5b816020850160208301376000918101602001919091529392505050565b600080600080600060a08688031215610d8357600080fd5b610d8c86610b23565b9450610d9a60208701610b23565b9350604086013567ffffffffffffffff80821115610db757600080fd5b610dc389838a01610c86565b94506060880135915080821115610dd957600080fd5b610de589838a01610c86565b93506080880135915080821115610dfb57600080fd5b50610e0888828901610cf7565b9150509295509295909350565b60008060408385031215610e2857600080fd5b823567ffffffffffffffff80821115610e4057600080fd5b818501915085601f830112610e5457600080fd5b81356020610e6182610c62565b604051610e6e8282610c35565b83815260059390931b8501820192828101915089841115610e8e57600080fd5b948201945b83861015610eb357610ea486610b23565b82529482019490820190610e93565b96505086013592505080821115610ec957600080fd5b50610ed685828601610c86565b9150509250929050565b600081518084526020808501945080840160005b83811015610f1057815187529582019590820190600101610ef4565b509495945050505050565b602081526000610b9f6020830184610ee0565b60008060408385031215610f4157600080fd5b610f4a83610b23565b915060208301358015158114610f5f57600080fd5b809150509250929050565b60008060408385031215610f7d57600080fd5b610f8683610b23565b9150610f9460208401610b23565b90509250929050565b600080600080600060a08688031215610fb557600080fd5b610fbe86610b23565b9450610fcc60208701610b23565b93506040860135925060608601359150608086013567ffffffffffffffff811115610ff657600080fd5b610e0888828901610cf7565b600181811c9082168061101657607f821691505b60208210810361103657634e487b7160e01b600052602260045260246000fd5b50919050565b6020808252602e908201527f455243313135353a2063616c6c6572206973206e6f7420746f6b656e206f776e60408201526d195c881bdc88185c1c1c9bdd995960921b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000600182016110c8576110c86110a0565b5060010190565b60208082526025908201527f455243313135353a207472616e7366657220746f20746865207a65726f206164604082015264647265737360d81b606082015260800190565b6020808252602a908201527f455243313135353a20696e73756666696369656e742062616c616e636520666f60408201526939103a3930b739b332b960b11b606082015260800190565b60008219821115611171576111716110a0565b500190565b6040815260006111896040830185610ee0565b828103602084015261119b8185610ee0565b95945050505050565b6001600160a01b0386811682528516602082015260a0604082018190526000906111d090830186610ee0565b82810360608401526111e28186610ee0565b905082810360808401526111f68185610bbf565b98975050505050505050565b60006020828403121561121457600080fd5b8151610b9f81610b69565b600060033d11156112385760046000803e5060005160e01c5b90565b600060443d10156112495790565b6040516003193d81016004833e81513d67ffffffffffffffff816024840111818411171561127957505050505090565b82850191508151818111156112915750505050505090565b843d87010160208285010111156112ab5750505050505090565b6112ba60208286010187610c35565b509095945050505050565b60208082526028908201527f455243313135353a204552433131353552656365697665722072656a656374656040820152676420746f6b656e7360c01b606082015260800190565b6001600160a01b03868116825285166020820152604081018490526060810183905260a06080820181905260009061134790830184610bbf565b97965050505050505056fea264697066735822122058e21cac016df0b18d7c456d851f55f323c9d78f3540a10ddd150a570dbfb14b64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ERC1155<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC1155<M> {
        fn clone(&self) -> Self {
            ERC1155(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC1155<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC1155<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC1155))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC1155<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC1155_ABI.clone(), client).into()
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
                ERC1155_ABI.clone(),
                ERC1155_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balanceOf` (0x00fdd58e) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (account, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOfBatch` (0x4e1273f4) function"]
        pub fn balance_of_batch(
            &self,
            accounts: ::std::vec::Vec<ethers::core::types::Address>,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([78, 18, 115, 244], (accounts, ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            account: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (account, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function"]
        pub fn safe_batch_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, amounts, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xf242432a) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
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
        #[doc = "Calls the contract's `uri` (0x0e89341c) function"]
        pub fn uri(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([14, 137, 52, 28], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferBatch` event"]
        pub fn transfer_batch_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferBatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferSingle` event"]
        pub fn transfer_single_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferSingleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `URI` event"]
        pub fn uri_filter(&self) -> ethers::contract::builders::Event<M, UriFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC1155Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC1155<M> {
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub ids: Vec<ethers::core::types::U256>,
        pub values: Vec<ethers::core::types::U256>,
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
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: String,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC1155Events {
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
    }
    impl ethers::contract::EthLogDecode for ERC1155Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC1155Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(ERC1155Events::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ERC1155Events::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(ERC1155Events::UriFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC1155Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC1155Events::ApprovalForAllFilter(element) => element.fmt(f),
                ERC1155Events::TransferBatchFilter(element) => element.fmt(f),
                ERC1155Events::TransferSingleFilter(element) => element.fmt(f),
                ERC1155Events::UriFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub accounts: ::std::vec::Vec<ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub account: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `[46, 178, 194, 214]`"]
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
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `[242, 66, 67, 42]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
    #[doc = "Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC1155Calls {
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        IsApprovedForAll(IsApprovedForAllCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Uri(UriCall),
    }
    impl ethers::core::abi::AbiDecode for ERC1155Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155Calls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UriCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC1155Calls::Uri(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC1155Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC1155Calls::BalanceOf(element) => element.encode(),
                ERC1155Calls::BalanceOfBatch(element) => element.encode(),
                ERC1155Calls::IsApprovedForAll(element) => element.encode(),
                ERC1155Calls::SafeBatchTransferFrom(element) => element.encode(),
                ERC1155Calls::SafeTransferFrom(element) => element.encode(),
                ERC1155Calls::SetApprovalForAll(element) => element.encode(),
                ERC1155Calls::SupportsInterface(element) => element.encode(),
                ERC1155Calls::Uri(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC1155Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC1155Calls::BalanceOf(element) => element.fmt(f),
                ERC1155Calls::BalanceOfBatch(element) => element.fmt(f),
                ERC1155Calls::IsApprovedForAll(element) => element.fmt(f),
                ERC1155Calls::SafeBatchTransferFrom(element) => element.fmt(f),
                ERC1155Calls::SafeTransferFrom(element) => element.fmt(f),
                ERC1155Calls::SetApprovalForAll(element) => element.fmt(f),
                ERC1155Calls::SupportsInterface(element) => element.fmt(f),
                ERC1155Calls::Uri(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC1155Calls {
        fn from(var: BalanceOfCall) -> Self {
            ERC1155Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfBatchCall> for ERC1155Calls {
        fn from(var: BalanceOfBatchCall) -> Self {
            ERC1155Calls::BalanceOfBatch(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for ERC1155Calls {
        fn from(var: IsApprovedForAllCall) -> Self {
            ERC1155Calls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<SafeBatchTransferFromCall> for ERC1155Calls {
        fn from(var: SafeBatchTransferFromCall) -> Self {
            ERC1155Calls::SafeBatchTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for ERC1155Calls {
        fn from(var: SafeTransferFromCall) -> Self {
            ERC1155Calls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for ERC1155Calls {
        fn from(var: SetApprovalForAllCall) -> Self {
            ERC1155Calls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ERC1155Calls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ERC1155Calls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UriCall> for ERC1155Calls {
        fn from(var: UriCall) -> Self {
            ERC1155Calls::Uri(var)
        }
    }
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfBatchReturn(pub ::std::vec::Vec<ethers::core::types::U256>);
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UriReturn(pub String);
}
