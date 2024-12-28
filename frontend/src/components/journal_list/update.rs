use super::*;

impl JournalList {
    pub fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::LoadJournals => {
                self.model.loading = true;
                let api = self.api.clone();
                let current_page = self.model.page_info.current_page;
                ctx.link().send_future(async move {
                    match api.fetch_journals(current_page).await {
                        Ok(response) => Msg::JournalsLoaded(response),
                        Err(e) => Msg::Error(e.message),
                    }
                });
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