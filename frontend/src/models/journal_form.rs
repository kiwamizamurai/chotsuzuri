use crate::models::journal::Account;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Model {
    pub date: String,
    pub description: String,
    pub entries: Vec<JournalEntryForm>,
    pub error: Option<String>,
    pub accounts: Vec<Account>,
    pub loading: bool,
}

#[derive(Default)]
pub struct JournalEntryForm {
    pub id: i32,
    pub amount: String,
    pub is_debit: bool,
}

impl JournalEntryForm {
    pub fn default_debit() -> Self {
        Self {
            id: -1,
            amount: String::new(),
            is_debit: true,
        }
    }

    pub fn default_credit() -> Self {
        Self {
            id: -1,
            amount: String::new(),
            is_debit: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateJournalRequest {
    pub date: String,
    pub description: String,
    pub entries: Vec<CreateJournalEntryRequest>,
}

#[derive(Debug, Serialize)]
pub struct CreateJournalEntryRequest {
    pub account_id: i32,
    pub is_debit: bool,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJournalResponse {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Entries")]
    pub entries: Vec<CreateJournalEntryResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJournalEntryResponse {
    #[serde(rename = "AccountID")]
    pub account_id: i32,
    #[serde(rename = "IsDebit")]
    pub is_debit: bool,
    #[serde(rename = "Amount")]
    pub amount: i32,
}