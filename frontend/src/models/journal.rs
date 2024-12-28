use gql_client::GraphQLErrorMessage;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// TODO

// rest response

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Journal {
    pub id: String,
    #[serde(rename = "journalNumber")]
    pub journal_number: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub entries: Vec<JournalEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JournalEntry {
    pub id: String,
    pub account: Account,
    #[serde(rename = "isDebit")]
    pub is_debit: bool,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Account {
    pub id: i32,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalFilter {
    pub date_range: Option<DateRange>,
    pub account_codes: Option<Vec<String>>,
    pub amount_range: Option<AmountRange>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

// graphql response

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLErrorMessage>>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLJournalsData {
    pub journals: JournalListResponse
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "hasPrevPage")]
    pub has_prev_page: bool,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    #[serde(rename = "currentPage")]
    pub current_page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalListResponse {
    pub items: Vec<JournalGraphql>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

pub type GraphQLJournalsResponse = GraphQLResponse<GraphQLJournalsData>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JournalGraphql {
    pub id: String,
    #[serde(rename = "journalNumber")]
    pub journal_number: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub entries: Vec<JournalEntryGraphql>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JournalEntryGraphql {
    pub id: String,
    pub account: AccountGraphql,
    #[serde(rename = "isDebit")]
    pub is_debit: bool,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountGraphql {
    pub id: String,
    pub code: String,
    pub name: String,
}