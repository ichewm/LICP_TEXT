mod clients;
mod common;

use hex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use ic_cdk::api::call::arg_data_raw;
use serde_bytes::ByteBuf;
use crate::clients::licpicrc1::{LICPICRC1, LICPICRC1TxReceipt, TransferArg, Account};
use crate::common::types::{CanisterName};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{heartbeat, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk::api::{canister_balance, time};
use ic_cdk::export::candid::{export_service, CandidType, Deserialize, Int, Nat, Principal};
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
    let canister_id = ic_cdk::api::id();

    // let from_account_identifier = AccountIdentifier::new(
    //     &canister_id,
    //     &Subaccount::from(_from)
    // );
    // let from_subaccount = Subaccount(from_account_identifier.as_ref());
    // let from_account_identifier_string = from_account_identifier.to_string();
    // let mut from_account:[u8; 32] = [0; 32];
    // from_account.copy_from_slice(&hex::decode(from_account_identifier_string).unwrap(),);
    // let from_subaccount = Subaccount(from_account);

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

//Principal to 


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


