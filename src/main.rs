use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration};

use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet},
    types::{Address, U256},
    contract::abigen
};

use serde_json::Value;



use alloy_primitives::{Address};
   

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()

        // `GET /` goes to `root`
        .route("/", get(root))

        // `POST /users` goes to `create_user`
        .route("/send-usdc", post(send_usdc));


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}


// Basic Function to Send USDC
async fn send_usdc(Json(payload): Json<SendUsdc>) -> Result<T, E> {

    let from_wallet: LocalWallet = process.env.PRIVATE_KEY;
    .parse::<LocalWallet>()
    .unwrap();

    let to_address = payload.receiver.parse::<Address>()?;
    let amount = payload.amount;

    let usdc_token_addr = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>()?;

    // 1. Generate the ABI for the ERC20 contract. This is will define an `ERC20Contract` struct in
    // this scope that will let us call the methods of the contract.
    abigen!(
        ERC20Contract,
        r#"[
            function balanceOf(address account) external view returns (uint256)
            function decimals() external view returns (uint8)
            function symbol() external view returns (string memory)
            function transfer(address to, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#,
    );

    // 2. Create the contract instance to let us call methods of the contract and let it sign
    // transactions with the sender wallet.
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/" + process.env.INFURA_KEY)?.interval(Duration::from_millis(10u64));

    // Chain ID
    let chain_id = provider.get_chainid().await?;

    let signer = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64())));
    let contract = ERC20Contract::new(usdc_token_addr, signer);

    // 3. Fetch the decimals used by the contract so we can compute the decimal amount to send.
    let whole_amount: u64 = amount;
    let decimals = contract.decimals().call();
    let decimal_amount = U256::from(whole_amount) * U256::exp10(decimals as usize);

    // 4. Transfer the desired amount of tokens to the `to_address`
    let tx = contract.transfer(to_address, decimal_amount);

    let pending_tx = tx.send().await?;
    let _mined_tx = pending_tx.await?;

    println!("Transaction Receipt: {}", serde_json::to_string(&_mined_tx)?);

    // Extract the tx hash for printing
    let json_str = serde_json::to_string(&_mined_tx)?;
    let json: Value = serde_json::from_str(&json_str)?;

    if let Some(transaction_hash) = json["transactionHash"].as_str() {
        println!("\n URL: https://etherscan.io/tx/{}", transaction_hash);
    } else {
        println!("Transaction Hash not found");
    }

    Ok(())

}


// a request body type

#[derive(Deserialize, Serialize)]

struct SendUsdc {
    receiver: String,
    amount: u64,
}