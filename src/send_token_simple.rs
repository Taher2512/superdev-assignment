use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use axum::{Json, http::StatusCode};
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    success: bool,
    data: SendTokenData,
}

#[derive(Serialize)]
pub struct SendTokenData {
    program_id: String,
    accounts: Vec<AccountInfo>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pubkey: String,
    #[serde(rename = "isSigner")]
    is_signer: bool,
}

fn is_valid_pubkey(pubkey_str: &str) -> bool {
    match Pubkey::from_str(pubkey_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> Result<Json<SendTokenResponse>, (StatusCode, Json<ErrorResponse>)> {
    if payload.destination.is_empty() || payload.mint.is_empty() || payload.owner.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        })));
    }

    if payload.amount == 0 {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Amount must be greater than zero".to_string(),
        })));
    }

    if !is_valid_pubkey(&payload.destination) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Invalid destination address".to_string(),
        })));
    }

    if !is_valid_pubkey(&payload.mint) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Invalid mint address".to_string(),
        })));
    }

    if !is_valid_pubkey(&payload.owner) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Invalid owner address".to_string(),
        })));
    }

    let accounts = vec![
        AccountInfo {
            pubkey: payload.mint.clone(),
            is_signer: false,
        },
        AccountInfo {
            pubkey: payload.destination.clone(),
            is_signer: false,
        },
        AccountInfo {
            pubkey: payload.owner.clone(),
            is_signer: true,
        },
    ];

    let response = SendTokenResponse {
        success: true,
        data: SendTokenData {
            program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(&[1, 2, 3, 4]), // Mock instruction data
        },
    };

    Ok(Json(response))
}
