use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};
use axum::Json;

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    success: bool,
    data: KeypairData,
}

#[derive(Serialize)]
pub struct KeypairData {
    pubkey: String,
    secret: String,
}

pub async fn get_keypair() -> Json<KeypairResponse> {
    let keypair = Keypair::new();
    let address = keypair.pubkey();
    let secret = keypair.to_base58_string();

    let response = KeypairResponse {
        success: true,
        data: KeypairData {
            pubkey: address.to_string(),
            secret,
        },
    };

    Json(response)
}