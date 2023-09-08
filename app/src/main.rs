use bindings::hyperlane_message_sender::HyperlaneMessageSender;

use ethers::{prelude::*, providers::SEPOLIA, types::{Address,H256}};

use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = SEPOLIA.provider();
    let provider = Arc::new(provider);

    let sender_address = "0x03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144".parse::<Address>()?;
    let recipient = "0x0000000000000000000000006482CdA5DF7605B52592a3D04af1f7e3004262FE".parse::<H256>()?;

    let wallet = "c1836c120a271f4633073501c04cc93a6ee2ba3b267847cb0fc90e29765d1694".parse::<LocalWallet>()?.with_chain_id(Chain::Sepolia);

    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let contract = HyperlaneMessageSender::new(sender_address, client.clone());
    
    let _receipt = contract.send_string(80001, recipient.as_fixed_bytes().to_owned(), "hello from rust".to_owned()).send().await?.await?;
    
    Ok(())
}
