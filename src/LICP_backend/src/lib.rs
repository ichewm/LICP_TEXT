mod clients;
mod common;

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
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT, AccountBalanceArgs, TransferArgs, Memo, Tokens, BlockIndex, TransferResult, Subaccount};

const ICP_FEE: u64 = 10_000;

pub type CanistersMapping = BTreeMap<String, Principal>;

// Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()

thread_local! {
    static CANISTERSMAPPING:RefCell<CanistersMapping> = RefCell::default();
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

// 查询子账户ICP余额
#[update]
pub async fn icp_balance(account_principal: Principal) -> u64 {

    let canister_id = ic_cdk::api::id();
    let account_principal = ic_cdk::api::caller();  // 当前请求用户的唯一标识
    let account_id = AccountIdentifier::new(&canister_id, &Subaccount::from(account_principal));
    let balance_args = AccountBalanceArgs { account: account_id };

    let balance = ic_ledger_types::account_balance(query_canister(String::from("licpicrc")), balance_args)
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


