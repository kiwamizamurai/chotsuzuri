use super::*;
use crate::models::journal_list::JournalFilter;

impl JournalList {
    pub fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::LoadAccounts => {
                let api = self.api.clone();
                ctx.link().send_future(async move {
                    match api.fetch_accounts().await {
                        Ok(accounts) => Msg::AccountsLoaded(accounts),
                        Err(e) => Msg::Error(e.message),
                    }
                });
                true
            }
            Msg::AccountsLoaded(accounts) => {
                self.model.accounts = accounts;
                ctx.link().send_message(Msg::LoadJournals);
                true
            }
            Msg::LoadJournals => {
                self.model.loading = true;
                let api = self.api.clone();
                let current_page = self.model.page_info.current_page;
                let filter = self.model.filter.clone();
                ctx.link().send_future(async move {
                    match api.fetch_journals(current_page, filter).await {
                        Ok(response) => Msg::JournalsLoaded(response),
                        Err(e) => Msg::Error(e.message),
                    }
                });
                true
            }
            Msg::SetDateFrom(date) => {
                self.model.filter.date_from = if date.is_empty() { None } else { Some(date) };
                true
            }
            Msg::SetDateTo(date) => {
                self.model.filter.date_to = if date.is_empty() { None } else { Some(date) };
                true
            }
            Msg::SetAccountCode(code) => {
                self.model.filter.account_code = if code.is_empty() { None } else { Some(code) };
                true
            }
            Msg::SetAmountMin(amount) => {
                self.model.filter.amount_min = amount;
                true
            }
            Msg::SetAmountMax(amount) => {
                self.model.filter.amount_max = amount;
                true
            }
            Msg::ApplyFilter => {
                self.model.page_info.current_page = 1;
                ctx.link().send_message(Msg::LoadJournals);
                true
            }
            Msg::ResetFilter => {
                self.model.filter = JournalFilter::default();
                self.model.page_info.current_page = 1;
                ctx.link().send_message(Msg::LoadJournals);
                true
            }
            Msg::JournalsLoaded(response) => {
                self.model.loading = false;
                if let Some(data) = response.data {
                    self.model.journals = data.journals.items;
                    self.model.page_info = data.journals.page_info;
                    true
                } else {
                    self.model.error = Some("データが取得できませんでしたよ".to_string());
                    true
                }
            }
            Msg::Error(error) => {
                self.model.loading = false;
                self.model.error = Some(error);
                true
            }
            Msg::LoadNextPage => {
                if self.model.page_info.has_next_page {
                    self.model.page_info.current_page += 1;
                    ctx.link().send_message(Msg::LoadJournals);
                }
                true
            }
            Msg::LoadPreviousPage => {
                if self.model.page_info.has_prev_page {
                    self.model.page_info.current_page -= 1;
                    ctx.link().send_message(Msg::LoadJournals);
                }
                true
            }
        }
    }
} 