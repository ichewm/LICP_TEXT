use std::cell::Cell;

use candid::Principal;
use ic_cdk::export::candid::{CandidType, Deserialize, Nat};



#[derive(CandidType, Deserialize, Clone)]
pub enum CanisterName {
    Licpicrc1Canister,
    ICPCanister
}
