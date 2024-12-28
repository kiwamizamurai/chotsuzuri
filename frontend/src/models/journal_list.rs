use crate::models::journal::{JournalGraphql, PageInfo};

#[derive(Default)]
pub struct Model {
    pub journals: Vec<JournalGraphql>,
    pub page_info: PageInfo,
    pub loading: bool,
    pub error: Option<String>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            journals: Vec::new(),
            page_info: PageInfo {
                current_page: 1,
                total_pages: 0,
                has_next_page: false,
                has_prev_page: false,
            },
            loading: false,
            error: None,
        }
    }
}