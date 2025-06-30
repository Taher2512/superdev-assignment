use axum::{routing::post, Router};

mod keypair;
mod spl_token;
mod mint_token;
mod message;
mod send_sol;
mod send_token_simple;

use keypair::get_keypair;
use spl_token::spl_token_initialize_mint_instruction;
use mint_token::mint_token;
use message::{sign_message, verify_message};
use send_sol::send_sol;
use send_token_simple::send_token;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(get_keypair))
        .route("/token/create", post(spl_token_initialize_mint_instruction))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    let address = "0.0.0.0:80";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();

    println!("Server running on {}", address);
    axum::serve(listener, app).await.unwrap();
}

