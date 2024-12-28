use super::*;

impl JournalList {
    pub fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                if self.model.loading {
                    <div class="loading-indicator">
                        <div class="spinner"></div>
                        <p>{"データを読み込んでいます..."}</p>
                    </div>
                } else if let Some(error) = &self.model.error {
                    <div class="error-message">
                        {error}
                    </div>
                } else {
                    <>
                        <table class="journal-table">
                            <thead>
                                <tr>
                                    <th>{"伝票番号"}</th>
                                    <th>{"日付"}</th>
                                    <th>{"摘要"}</th>
                                    <th>{"借方科目"}</th>
                                    <th>{"借方金額"}</th>
                                    <th>{"貸方科目"}</th>
                                    <th>{"貸方金額"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for self.model.journals.iter().map(|journal| {
                                    let debit_entries: Vec<_> = journal.entries.iter()
                                        .filter(|e| e.is_debit)
                                        .collect();
                                    let credit_entries: Vec<_> = journal.entries.iter()
                                        .filter(|e| !e.is_debit)
                                        .collect();
                                    let max_rows = debit_entries.len().max(credit_entries.len());

                                    html! {
                                        {for (0..max_rows).map(|i| {
                                            html! {
                                                <tr>
                                                    if i == 0 {
                                                        <td rowspan={max_rows.to_string()}>{&journal.journal_number}</td>
                                                        <td rowspan={max_rows.to_string()}>{journal.date.format("%Y-%m-%d")}</td>
                                                        <td rowspan={max_rows.to_string()}>{&journal.description}</td>
                                                    }
                                                    <td>{debit_entries.get(i).map_or("", |e| &e.account.name)}</td>
                                                    <td class="amount">
                                                        {debit_entries.get(i)
                                                            .map_or("".to_string(), |e| format_currency(e.amount))}
                                                    </td>
                                                    <td>{credit_entries.get(i).map_or("", |e| &e.account.name)}</td>
                                                    <td class="amount">
                                                        {credit_entries.get(i)
                                                            .map_or("".to_string(), |e| format_currency(e.amount))}
                                                    </td>
                                                </tr>
                                            }
                                        })}
                                    }
                                })}
                            </tbody>
                        </table>

                        <div class="pagination">
                            <button
                                onclick={ctx.link().callback(|_| Msg::LoadPreviousPage)}
                                disabled={!self.model.page_info.has_prev_page}
                            >
                                {"前へ"}
                            </button>
                            <span class="page-info">
                                {format!("ページ {}/{}",
                                    self.model.page_info.current_page,
                                    self.model.page_info.total_pages
                                )}
                            </span>
                            <button
                                onclick={ctx.link().callback(|_| Msg::LoadNextPage)}
                                disabled={!self.model.page_info.has_next_page}
                            >
                                {"次へ"}
                            </button>
                        </div>
                    </>
                }
            </div>
        }
    }
}