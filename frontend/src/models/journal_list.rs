use crate::models::journal::{JournalGraphql, PageInfo, Account};

#[derive(Default, Clone)]
pub struct JournalFilter {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub account_code: Option<String>,
    pub amount_min: Option<i32>,
    pub amount_max: Option<i32>,
}

#[derive(Default)]
pub struct Model {
    pub journals: Vec<JournalGraphql>,
    pub accounts: Vec<Account>,
    pub page_info: PageInfo,
    pub loading: bool,
    pub error: Option<String>,
    pub filter: JournalFilter,
}

impl Model {
    pub fn new() -> Self {
        Self {
            journals: Vec::new(),
            accounts: Vec::new(),
            page_info: PageInfo {
                current_page: 1,
                total_pages: 0,
                has_next_page: false,
                has_prev_page: false,
            },
            loading: false,
            error: None,
            filter: JournalFilter::default(),
        }
    }
}