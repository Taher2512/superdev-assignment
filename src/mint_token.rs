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
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: i64,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    success: bool,
    data: MintTokenData,
}

#[derive(Serialize)]
pub struct MintTokenData {
    program_id: String,
    accounts: Vec<AccountData>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountData {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> Result<Json<MintTokenResponse>, (StatusCode, Json<ErrorResponse>)> {
    if payload.mint.is_empty() || payload.destination.is_empty() || payload.authority.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        })));
    }

    if payload.amount <= 0 {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Amount must be greater than zero".to_string(),
        })));
    }

    let _mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(pub_key) => pub_key,
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
                success: false,
                error: "Invalid mint address".to_string(),
            })));
        }
    };

    let _destination_pubkey = match Pubkey::from_str(&payload.destination) {
        Ok(pub_key) => pub_key,
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
                success: false,
                error: "Invalid destination address".to_string(),
            })));
        }
    };

    let _authority_pubkey = match Pubkey::from_str(&payload.authority) {
        Ok(pub_key) => pub_key,
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
                success: false,
                error: "Invalid authority address".to_string(),
            })));
        }
    };

    let accounts = vec![
        AccountData {
            pubkey: payload.mint.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountData {
            pubkey: payload.destination.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountData {
            pubkey: payload.authority.clone(),
            is_signer: true,
            is_writable: false,
        },
    ];

    let response = MintTokenResponse {
        success: true,
        data: MintTokenData {
            program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(&[1, 2, 3, 4]),
        },
    };

    Ok(Json(response))
}
