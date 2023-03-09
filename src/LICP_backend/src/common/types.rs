use std::cell::Cell;
// use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};

type CanisterId = Principal;

#[derive(CandidType, Deserialize, Clone)]
pub enum CanisterName {
    Licpicrc1Canister,
    ICPCanister
}

pub enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {  // 公钥回复
    pub public_key_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignatureReply {   // 签名回复
    pub signature_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignatureVerificationReply {  // 签名验证回复
    pub is_signature_valid: bool,
}


#[derive(CandidType, Serialize, Debug)]
pub struct ECDSAPublicKey {  // ECDSAP公钥
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ECDSAPublicKeyReply {  // ECDSAP公钥回复
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignWithECDSA {  // 使用 ECDSA 签名
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithECDSAReply {  // 使用 ECDSA 回复签名
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
pub struct EcdsaKeyId {  // Ecdsa 密钥 ID
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone)]
pub enum EcdsaCurve {  // Ecdsa 曲线
    #[serde(rename = "secp256k1")]
    Secp256k1,
}
