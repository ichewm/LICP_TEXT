mod clients;
mod common;

use hex;
use std::convert::TryFrom;
use std::str::FromStr;
use std::cell::RefCell;
use std::collections::BTreeMap;
use ic_cdk::api::call::arg_data_raw;
use serde_bytes::ByteBuf;
use crate::clients::licpicrc1::{LICPICRC1, LICPICRC1TxReceipt, TransferArg, Account};
use crate::common::types::*;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{heartbeat, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk::api::{canister_balance, time};
use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_cdk::export::candid::{
    export_service, CandidType, Deserialize,
    Int, Nat, 
    Principal};
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT, DEFAULT_FEE, AccountBalanceArgs, TransferArgs, Memo, Tokens, BlockIndex, TransferResult, Subaccount, TransferError};

pub type CanistersMapping = BTreeMap<String, Principal>;
pub type AccountMapping = BTreeMap<Principal, AccountIdentifier>;  // 子账户主账户映射

thread_local! {
    static CANISTERSMAPPING:RefCell<CanistersMapping> = RefCell::default();
    static ACCOUNTMAPPING: RefCell<AccountMapping> = RefCell::default();
}

#[query]
pub fn query_canister(canister_name:String) -> Principal {
    CANISTERSMAPPING.with(|canistersmapping|{
        canistersmapping.borrow().get(&canister_name).unwrap().clone()
    })
}

#[update]
pub async fn set_canister(canister_name:String, canister_id:Principal) {
    CANISTERSMAPPING.with(|canistersmapping|{
        canistersmapping.borrow_mut().insert(canister_name, canister_id);
    })
}

// 将指定用户余额转给其他用户
#[update]
pub async fn transfer_tototo(_from:Principal, _to:String, value: u64) -> Result<u64, TransferError> {
    let mut bytes: [u8; 32] = [0; 32];
    bytes.copy_from_slice(&hex::decode(_to).unwrap());
    let to_account = AccountIdentifier::try_from(bytes).unwrap();

    let transfer_args = TransferArgs {
            memo: Memo(0),
            amount: Tokens::from_e8s(value) - DEFAULT_FEE,
            fee: DEFAULT_FEE,
            from_subaccount: Some(Subaccount::from(_from)),
            to: to_account,
            created_at_time: None,
        };
    ic_ledger_types::transfer(query_canister(String::from("icp")), transfer_args).await.expect("transfer error")

}


#[query]
pub fn query_account_principal() -> Principal{
    ic_cdk::api::caller()
}

#[query]
pub fn query_canister_id() -> Principal {
    ic_cdk::api::id()
}

// 生成属于这个罐的用户id
#[query]
fn select_canister_account_id() -> String {
    let canister_id = ic_cdk::api::id();
    let account_principal = ic_cdk::api::caller();  // 当前请求用户的唯一标识
    AccountIdentifier::new(
        &canister_id,
        &Subaccount::from(account_principal)
    ).to_string()
}

// 根据输入 主体生成 account_id
#[query]
fn principal_to_account_id(account_principal:Principal) -> String {
    let canister_id = ic_cdk::api::id();
    AccountIdentifier::new(
        &canister_id,
        &Subaccount::from(account_principal)
    ).to_string()
}


// 查询子账户ICP余额
#[update]
pub async fn icp_balance(account_principal: Principal) -> u64 {
    let canister_id = ic_cdk::api::id();
    let account_principal = ic_cdk::api::caller();  // 当前请求用户的唯一标识
    let account_id = AccountIdentifier::new(&canister_id, &Subaccount::from(account_principal));
    let balance_args = AccountBalanceArgs { account: account_id };

    let balance = ic_ledger_types::account_balance(query_canister(String::from("icp")), balance_args)
        .await.expect("no balance");
    balance.e8s()
}


#[update]
pub async fn minting_target_tokens() {
    let account_id = Account {
        owner:ic_cdk::api::caller(),
        subaccount: Option::Some(DEFAULT_SUBACCOUNT),
    };
    let arg = TransferArg{
        from_subaccount: None,
        to: account_id,
        fee: None,
        created_at_time: None,
        memo: Option::Some(Memo(0)),
        amount: Nat::from(100),
    };
    LICPICRC1::icrc1_transfer(&query_canister(String::from("licpicrc")), arg).await;

}

// 使用 tecdsa 生成一个 用于管理 icp 的 罐

// 控制 管理icp 的罐 发起交易

// 展示这个罐的id


// 关于 tecdsa 测试

#[update]
async fn public_key() -> Result<PublicKeyReply, String> {
    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,): (ECDSAPublicKeyReply,) =
        ic_cdk::call(mgmt_canister_id(), "ecdsa_public_key", (request,))
            .await
            .map_err(|e| format!("ecdsa_public_key failed {}", e.1))?;

    Ok(PublicKeyReply {
        public_key_hex: hex::encode(&res.public_key),
    })
}

// public key to Principal
#[update]
pub async fn public_key_to_principal() -> Principal {
    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,): (ECDSAPublicKeyReply,) =
        ic_cdk::call(mgmt_canister_id(), "ecdsa_public_key", (request,))
            .await
            .map_err(|e| format!("ecdsa_public_key failed {}", e.1)).expect("server error");
    
    Principal::self_authenticating(res.public_key)
}



#[update]
async fn sign(message: String) -> Result<SignatureReply, String> {
    let request = SignWithECDSA {
        message_hash: sha256(&message).to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,): (SignWithECDSAReply,) = ic_cdk::api::call::call_with_payment(
        mgmt_canister_id(),
        "sign_with_ecdsa",
        (request,),
        25_000_000_000,
    )
    .await
    .map_err(|e| format!("sign_with_ecdsa failed {}", e.1))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(&response.signature),
    })
}

#[query]
async fn verify(
    signature_hex: String,
    message: String,
    public_key_hex: String,
) -> Result<SignatureVerificationReply, String> {
    let signature_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
    let pubkey_bytes = hex::decode(&public_key_hex).expect("failed to hex-decode public key");
    let message_bytes = message.as_bytes();

    use k256::ecdsa::signature::Verifier;
    let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
        .expect("failed to deserialize signature");
    let is_signature_valid= k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
        .expect("failed to deserialize sec1 encoding into public key")
        .verify(message_bytes, &signature)
        .is_ok();

    Ok(SignatureVerificationReply{
        is_signature_valid
    })
}

fn mgmt_canister_id() -> CanisterId {
    CanisterId::from_str(&"aaaaa-aa").unwrap()
}

fn sha256(input: &String) -> [u8; 32] {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}


impl EcdsaKeyIds {
    fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}

getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}