use std::str::FromStr;
// use tracing_subscriber;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use secp256k1::SecretKey;
mod contract_queries;
use web3::{
    types::Address,
};


#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("https://bsc-testnet-rpc.publicnode.com").unwrap();
    let web3 = web3::Web3::new(transport);
    let airdrop_contract = contract_queries::AirdropContract::new(
        &web3, "0x41a9ae5A4dF28E38ADe68e9124F5FA0F58237C0a".to_string(),
    ).await;
    let wallet:SecretKey = SecretKey::from_str("your account private key").unwrap();
    let account: Address = Address::from_str("your account address").unwrap();
    //Calling functions logic here
    Ok(())
}






