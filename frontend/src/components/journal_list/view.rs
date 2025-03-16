use super::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use web_sys::Event;

impl JournalList {
    pub fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <div class="filter-form">
                    <div class="filter-row">
                        <div class="filter-group">
                            <label>{"日付範囲"}</label>
                            <div class="date-range">
                                <input
                                    type="date"
                                    value={self.model.filter.date_from.clone().unwrap_or_default()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                        Msg::SetDateFrom(input.value())
                                    })}
                                />
                                <span>{" 〜 "}</span>
                                <input
                                    type="date"
                                    value={self.model.filter.date_to.clone().unwrap_or_default()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                        Msg::SetDateTo(input.value())
                                    })}
                                />
                            </div>
                        </div>
                        <div class="filter-group">
                            <label>{"勘定科目"}</label>
                            <select
                                onchange={ctx.link().callback(|e: Event| {
                                    let input = e.target_unchecked_into::<HtmlSelectElement>();
                                    Msg::SetAccountCode(input.value())
                                })}
                            >
                                <option value="">{"すべて"}</option>
                                {for self.model.accounts.iter().map(|account| {
                                    html! {
                                        <option value={account.code.clone()}>
                                            {format!("{} - {}", account.code, account.name)}
                                        </option>
                                    }
                                })}
                            </select>
                        </div>
                        <div class="filter-group">
                            <label>{"金額範囲"}</label>
                            <div class="amount-range">
                                <input
                                    type="number"
                                    placeholder="最小"
                                    value={self.model.filter.amount_min.map(|v| v.to_string()).unwrap_or_default()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                        Msg::SetAmountMin(input.value().parse::<i32>().ok())
                                    })}
                                />
                                <span>{" 〜 "}</span>
                                <input
                                    type="number"
                                    placeholder="最大"
                                    value={self.model.filter.amount_max.map(|v| v.to_string()).unwrap_or_default()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                        Msg::SetAmountMax(input.value().parse::<i32>().ok())
                                    })}
                                />
                            </div>
                        </div>
                    </div>
                    <div class="filter-actions">
                        <button
                            class="button-primary"
                            onclick={ctx.link().callback(|_| Msg::ApplyFilter)}
                        >
                            {"検索"}
                        </button>
                        <button
                            class="button-secondary"
                            onclick={ctx.link().callback(|_| Msg::ResetFilter)}
                        >
                            {"リセット"}
                        </button>
                    </div>
                </div>

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