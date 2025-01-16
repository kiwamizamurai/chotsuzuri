use crate::models::journal::{GraphQLJournalsResponse, Account};

#[derive(Debug)]
pub enum Msg {
    LoadJournals,
    JournalsLoaded(GraphQLJournalsResponse),
    LoadAccounts,
    AccountsLoaded(Vec<Account>),
    Error(String),
    LoadNextPage,
    LoadPreviousPage,
    SetDateFrom(String),
    SetDateTo(String),
    SetAccountCode(String),
    SetAmountMin(Option<i32>),
    SetAmountMax(Option<i32>),
    ApplyFilter,
    ResetFilter,
}