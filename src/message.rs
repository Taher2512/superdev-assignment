use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer, signature::Signature};
use axum::Json;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};
use bs58;

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    success: bool,
    data: SignMessageData,
}

#[derive(Serialize)]
pub struct SignMessageData {
    signature: String,
    public_key: String,
    message: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    success: bool,
    data: VerifyMessageData,
}

#[derive(Serialize)]
pub struct VerifyMessageData {
    valid: bool,
    message: String,
    pubkey: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SignApiResponse {
    Success(SignMessageResponse),
    Error(ErrorResponse),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum VerifyApiResponse {
    Success(VerifyMessageResponse),
    Error(ErrorResponse),
}

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> Json<SignApiResponse> {
    if payload.message.is_empty() {
        return Json(SignApiResponse::Error(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        }));
    }
    
    if payload.secret.is_empty() {
        return Json(SignApiResponse::Error(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        }));
    }

    // Parse the secret key from base58
    let keypair = if payload.secret.len() < 32 {
        return Json(SignApiResponse::Error(ErrorResponse {
            success: false,
            error: "Invalid secret key format".to_string(),
        }));
    } else {
        match bs58::decode(&payload.secret).into_vec() {
            Ok(bytes) if bytes.len() == 64 => {
                match Keypair::try_from(bytes.as_slice()) {
                    Ok(kp) => kp,
                    Err(_) => {
                        return Json(SignApiResponse::Error(ErrorResponse {
                            success: false,
                            error: "Invalid secret key format".to_string(),
                        }));
                    }
                }
            }
            _ => {
                return Json(SignApiResponse::Error(ErrorResponse {
                    success: false,
                    error: "Invalid secret key format".to_string(),
                }));
            }
        }
    };

    let message_bytes = payload.message.as_bytes();

    let signature = keypair.sign_message(message_bytes);

    let signature_base64 = general_purpose::STANDARD.encode(signature.as_ref());

    let public_key = keypair.pubkey().to_string();

    let response = SignMessageResponse {
        success: true,
        data: SignMessageData {
            signature: signature_base64,
            public_key,
            message: payload.message,
        },
    };

    Json(SignApiResponse::Success(response))
}

pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> Json<VerifyApiResponse> {
    if payload.message.is_empty() || payload.signature.is_empty() || payload.pubkey.is_empty() {
        return Json(VerifyApiResponse::Error(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        }));
    }

    let public_key = match solana_sdk::pubkey::Pubkey::from_str(&payload.pubkey) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(VerifyApiResponse::Error(ErrorResponse {
                success: false,
                error: "Invalid public key format".to_string(),
            }));
        }
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(VerifyApiResponse::Error(ErrorResponse {
                success: false,
                error: "Invalid signature format".to_string(),
            }));
        }
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return Json(VerifyApiResponse::Error(ErrorResponse {
                success: false,
                error: "Invalid signature format".to_string(),
            }));
        }
    };

    let message_bytes = payload.message.as_bytes();

    let is_valid = signature.verify(public_key.as_ref(), message_bytes);

    let response = VerifyMessageResponse {
        success: true,
        data: VerifyMessageData {
            valid: is_valid,
            message: payload.message,
            pubkey: payload.pubkey,
        },
    };

    Json(VerifyApiResponse::Success(response))
}
