use crate::models::journal_form::JournalEntryForm;

use super::*;

impl JournalForm {
    pub fn view(&self, ctx: &Context<Self>) -> Html {
        if self.model.loading {
            html! {
                <div class="loading-indicator">
                    <div class="spinner"></div>
                    <p>{"データを読み込んでいます..."}</p>
                </div>
            }
        } else {
            let accounts = &self.model.accounts;

            html! {
                <form onsubmit={ctx.link().callback(|e: SubmitEvent| {
                    e.prevent_default();
                    Msg::Submit
                })}>
                    <div class="journal-form">
                        <div class="form-row">
                            <div class="form-group">
                                <label>{"日付"}</label>
                                <input
                                    type="date"
                                    value={self.model.date.clone()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        Msg::SetDate(input.value())
                                    })}
                                    required=true
                                />
                            </div>

                            <div class="form-group">
                                <label>{"摘要"}</label>
                                <input
                                    type="text"
                                    value={self.model.description.clone()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                        Msg::SetDescription(input.value())
                                    })}
                                    required=true
                                />
                            </div>
                        </div>

                        <div class="entries">
                            { for (0..self.model.entries.len()).step_by(2).map(|i| {
                                let debit_entry = &self.model.entries[i];
                                let credit_entry = self.model.entries.get(i + 1);

                                html! {
                                    <div class="entry-pair">
                                        <div class="entry-side debit">
                                            <span class="entry-label">{"借方"}</span>
                                            {self.render_entry(ctx, i, debit_entry, accounts)}
                                        </div>
                                        {if let Some(credit) = credit_entry {
                                            html! {
                                                <div class="entry-side credit">
                                                    <span class="entry-label">{"貸方"}</span>
                                                    {self.render_entry(ctx, i + 1, credit, accounts)}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                        <button
                                            type="button"
                                            onclick={ctx.link().callback(move |_| Msg::RemoveEntry(i))}
                                            disabled={self.model.entries.len() <= 2}
                                        >
                                            {"削除"}
                                        </button>
                                    </div>
                                }
                            })}
                        </div>

                        <button type="button" onclick={ctx.link().callback(|_| Msg::AddEntry)}>
                            {"仕訳行を追加"}
                        </button>

                        <div class="form-actions">
                            <button type="submit">{"保存"}</button>
                            <button
                                type="button"
                                onclick={ctx.link().callback(|_| Msg::Cancel)}
                            >
                                {"キャンセル"}
                            </button>
                        </div>

                        {if let Some(error) = &self.model.error {
                            html! {
                                <div class="error-message validation">
                                    {error}
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </form>
            }
        }
    }

    fn render_entry(&self, ctx: &Context<Self>, index: usize, entry: &JournalEntryForm, accounts: &[Account]) -> Html {
        html! {
            <div class="entry-row">
                <select
                    value={entry.id.to_string()}
                    onchange={ctx.link().callback(move |e: Event| {
                        let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                        let value = select.value().parse::<i32>().unwrap_or(-1);
                        Msg::SetEntryAccount(index, value)
                    })}
                >
                    <option value="-1">{"選択してください"}</option>
                    {for accounts.iter().map(|account| {
                        html! {
                            <option value={account.id.to_string()}>
                                {format!("{}", account.name)}
                            </option>
                        }
                    })}
                </select>

                <input
                    type="number"
                    value={entry.amount.clone()}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        Msg::SetEntryAmount(index, input.value())
                    })}
                />

                <select
                    value={if entry.is_debit {"debit"} else {"credit"}}
                    onchange={ctx.link().callback(move |e: Event| {
                        let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                        Msg::SetEntryIsDebit(index, select.value() == "debit")
                    })}
                >
                    <option value="debit">{"借方"}</option>
                    <option value="credit">{"貸方"}</option>
                </select>
            </div>
        }
    }
}