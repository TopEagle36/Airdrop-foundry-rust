use secp256k1::SecretKey;
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::{
    contract::{Contract, Options},
    types::{Address,H160, H256, U256},
};

use std::fmt;
use std::str::FromStr;

pub struct AirdropContract(Contract<Http>);

impl AirdropContract {
    pub async fn new(web3: &web3::Web3<web3::transports::Http>, address: String,) -> Self {
        let address = Address::from_str(&address).unwrap();
        let contract = Contract::from_json(web3.eth(), address, include_bytes!("airdrop_abi.json")).unwrap();
        AirdropContract(contract)
    }

    pub async fn distribute_eth_by_amount(&self, account: Address, wallet: SecretKey, addrs: &mut [Address], amount: U256 )->H256{
        let recipients: Vec<Address> = addrs.to_vec();
        let tx = self.0.signed_call(
            "distrubuteEther", 
            (recipients, amount), 
            Options {
                gas: Some(5_000_000.into()),
                value: Some(amount),
                ..Default::default()
            },
            SecretKeyRef::new(&wallet),
        ).await
        .unwrap();
        tx
    }
    pub async fn distribute_eth_by_percent(&self, web3: &web3::Web3<web3::transports::Http>, account: Address, wallet: SecretKey, addrs: &mut [Address], percent: u32 )->H256{
        let recipients: Vec<Address> = addrs.to_vec();
        let balance = web3.eth().balance(account, None).await.unwrap();
        let amount = balance/100 * percent;
        let tx = self.0.signed_call(
            "distrubuteEther", 
            (recipients, amount), 
            Options {
                gas: Some(5_000_000.into()),
                value: Some(amount),
                ..Default::default()
            },
            SecretKeyRef::new(&wallet),
        ).await
        .unwrap();
        tx
    }

    pub async fn distribute_token_by_amount(&self, account: Address, token: String, wallet:SecretKey, addrs: &mut [Address], amount: U256)->H256{
        let token_address = H160::from_str(&token).expect("Invalid address");
        let recipients: Vec<Address> = addrs.to_vec();
        let tx = self.0.signed_call(
            "distrubuteToken", 
            (token_address, recipients, amount), 
            Options {
                gas: Some(5_000_000.into()),
                ..Default::default()
            },
            SecretKeyRef::new(&wallet),
        ).await
        .unwrap();
        tx
    }
    pub async fn distribute_token_by_percent(&self, web3: &web3::Web3<web3::transports::Http>, account: Address, token: String, wallet:SecretKey, addrs: &mut [Address], percent: u32)->H256{
        let recipients: Vec<Address> = addrs.to_vec();
        let token_address = H160::from_str(&token).expect("Invalid address");
        let token_contract = web3::contract::Contract::from_json(
            web3.eth(),
            token_address,
            include_bytes!("erc20_abi.json"),
        ).unwrap();
        let token_balance: U256  = token_contract.query("balanceOf", account, None, Options::default(), None).await.unwrap();
        let amount = token_balance/100*percent;
        let tx = self.0.signed_call(
            "distrubuteToken", 
            (token_address, recipients, amount), 
            Options {
                gas: Some(5_000_000.into()),
                ..Default::default()
            },
            SecretKeyRef::new(&wallet),
        ).await
        .unwrap();
        tx
    }
}