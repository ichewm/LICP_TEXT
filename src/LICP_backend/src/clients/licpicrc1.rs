use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Nat, Principal};
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT, AccountBalanceArgs, TransferArgs, Memo, Tokens, BlockIndex, TransferResult, Subaccount};
pub type NumTokens = Nat;

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub fee: Option<NumTokens>,
    pub created_at_time: Option<u64>,
    pub memo: Option<Memo>,
    pub amount: NumTokens,
}


#[derive(CandidType, Deserialize)]
pub enum TransferError {
    BadFee { expected_fee: NumTokens },
    BadBurn { min_burn_amount: NumTokens },
    InsufficientFunds { balance: NumTokens },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: Nat },
    GenericError { error_code: Nat, message: String },
}

pub type LICPICRC1TxReceipt = Result<Nat, TransferError>;

#[async_trait]
pub trait LICPICRC1 {
    async fn icrc1_transfer(&self, arg: TransferArg) -> CallResult<(LICPICRC1TxReceipt,)>;
}

#[async_trait]
impl LICPICRC1 for Principal {
    async fn icrc1_transfer(&self, arg: TransferArg) -> CallResult<(LICPICRC1TxReceipt,)> {
        call(*self, "icrc1_transfer", (arg,)).await
    }
}
