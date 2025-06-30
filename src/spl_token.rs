use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use axum::Json;
use base64::{Engine as _, engine::general_purpose};
use std::str::FromStr;

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Deserialize)]
pub struct InitializeMint {
    #[serde(alias = "mintAuthority")]
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
pub struct MintData {
    program_id: String,
    accounts: Vec<AccountData>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct MintResponse {
    success: bool,
    data: MintData,
}

#[derive(Serialize)]
pub struct AccountData {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(MintResponse),
    Error(ErrorResponse),
}

pub async fn spl_token_initialize_mint_instruction(
    Json(payload): Json<InitializeMint>,
) -> Json<ApiResponse> {
    let mint_authority = match Pubkey::from_str(&payload.mint_authority) {
        Ok(pub_key) => pub_key,
        Err(_) => {
            return Json(ApiResponse::Error(ErrorResponse {
                success: false,
                error: "Invalid mint authority public key".to_string(),
            }));
        }
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pub_key) => pub_key,
        Err(_) => {
            return Json(ApiResponse::Error(ErrorResponse {
                success: false,
                error: "Invalid mint public key".to_string(),
            }));
        }
    };

    let instruction = match initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        None,
        payload.decimals,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Json(ApiResponse::Error(ErrorResponse {
                success: false,
                error: "Failed to create mint instruction".to_string(),
            }));
        }
    };

    let accounts: Vec<AccountData> = instruction
        .accounts
        .iter()
        .map(|acc| AccountData {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let instruction_data = general_purpose::STANDARD.encode(&instruction.data);

    let response = MintResponse {
        success: true,
        data: MintData {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data,
        },
    };

    Json(ApiResponse::Success(response))
}