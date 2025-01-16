use crate::models::journal::{Account, Journal};
use web_sys::File;

#[derive(Debug)]
pub enum Msg {
    LoadAccounts,
    SetDate(String),
    SetDescription(String),
    AddEntry,
    RemoveEntry(usize),
    SetEntryAccount(usize, i32),
    SetEntryAmount(usize, String),
    SetEntryIsDebit(usize, bool),
    Submit,
    Cancel,
    SetAccounts(Vec<Account>),
    FetchError(String),
    SubmitStart,
    SubmitComplete,
    InitializeForm(Option<Journal>),
    FileSelected(File),
    ExtractReceiptStart,
    ExtractReceiptComplete(Journal),
}