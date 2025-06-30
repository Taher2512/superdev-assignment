use axum::{routing::post, Router};


mod keypair;
mod spl_token;

use keypair::{
    get_keypair, 
};

use spl_token::spl_token_initialize_mint_instruction;

#[tokio::main]
async fn main() {
    let router01 = Router::new().route("/keypair", post(get_keypair)).route("/token/create", post(spl_token_initialize_mint_instruction));

    let address = "0.0.0.0:80";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();

    axum::serve(listener, router01).await.unwrap();
}

