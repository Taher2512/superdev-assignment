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

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(KeypairResponse),
    Error(ErrorResponse),
}

pub async fn get_keypair() -> Json<ApiResponse> {
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

    Json(ApiResponse::Success(response))
}