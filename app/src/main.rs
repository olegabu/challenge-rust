use bindings::hyperlane_message_sender::HyperlaneMessageSender;
use ethers::{prelude::*, providers::SEPOLIA, types::{Address,H256}};
use eyre::Result;
use std::sync::Arc;
use clap::Parser;
use std::env;

/// Demo program to send a message via Hyperlane
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the sender chain
    #[arg(long, default_value = "sepolia")]
    sender_chain: String,
    /// Contract address on the sender chain
    #[arg(long, default_value = "03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144")]
    sender_address: String,
    /// Name of the receiver chain
    #[arg(long, default_value = "mumbai")]
    receiver_chain: String,
    /// Contract address on the receiver chain
    #[arg(long, default_value = "6482CdA5DF7605B52592a3D04af1f7e3004262FE")]
    receiver_address: String,
    /// Message to send
    #[arg(long, default_value = "hello from rust")]
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let provider = SEPOLIA.provider();

    let sender_address = args.sender_address.parse::<Address>()?;

    let recipient = format!("000000000000000000000000{}", args.receiver_address);
    let recipient = recipient.parse::<H256>()?;

    let demo_key = env::var("demo_key")?;

    let wallet = demo_key.parse::<LocalWallet>()?.with_chain_id(Chain::Sepolia);

    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let sender_contract = HyperlaneMessageSender::new(sender_address, client.clone());
    
    // first `await` returns a PendingTransaction, second one waits for it to be mined
    let receipt = sender_contract.send_string(80001, recipient.as_fixed_bytes().to_owned(), "hello from rust".to_owned()).send().await?.await;

    println!("HyperlaneMessageSender.send_string tx receipt {:?}", receipt.unwrap());
    
    Ok(())
}
