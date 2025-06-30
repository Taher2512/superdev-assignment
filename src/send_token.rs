use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::transfer;
use spl_associated_token_account::get_associated_token_address;
use axum::Json;
use std::str::FromStr;
use base64;

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

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(SendTokenResponse),
    Error(ErrorResponse),
}

fn is_valid_pubkey(pubkey_str: &str) -> bool {
    match Pubkey::from_str(pubkey_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> Json<ApiResponse> {
    if payload.destination.is_empty() || payload.mint.is_empty() || payload.owner.is_empty() {
        return Json(ApiResponse::Error(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        }));
    }

    if payload.amount == 0 {
        return Json(ApiResponse::Error(ErrorResponse {
            success: false,
            error: "Amount must be greater than zero".to_string(),
        }));
    }

    if !is_valid_pubkey(&payload.destination) {
        return Json(ApiResponse::Error(ErrorResponse {
            success: false,
            error: "Invalid destination address".to_string(),
        }));
    }

    if !is_valid_pubkey(&payload.mint) {
        return Json(ApiResponse::Error(ErrorResponse {
            success: false,
            error: "Invalid mint address".to_string(),
        }));
    }

    if !is_valid_pubkey(&payload.owner) {
        return Json(ApiResponse::Error(ErrorResponse {
            success: false,
            error: "Invalid owner address".to_string(),
        }));
    }

    let destination_pubkey = Pubkey::from_str(&payload.destination).unwrap();
    let mint_pubkey = Pubkey::from_str(&payload.mint).unwrap();
    let owner_pubkey = Pubkey::from_str(&payload.owner).unwrap();

    let source_ata = get_associated_token_address(&owner_pubkey, &mint_pubkey);
    let destination_ata = get_associated_token_address(&destination_pubkey, &mint_pubkey);

    let instruction = match transfer(
        &spl_token::id(),
        &source_ata,
        &destination_ata,
        &owner_pubkey,
        &[],
        payload.amount,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Json(ApiResponse::Error(ErrorResponse {
                success: false,
                error: "Failed to create token transfer instruction".to_string(),
            }));
        }
    };

    let instruction_data = base64::encode(&instruction.data);

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let response = SendTokenResponse {
        success: true,
        data: SendTokenData {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data,
        },
    };

    Json(ApiResponse::Success(response))
}
