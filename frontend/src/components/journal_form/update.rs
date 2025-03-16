use crate::{models::validation::ValidationError, models::journal_form::JournalEntryForm};

use super::*;

use wasm_bindgen_futures::spawn_local;

impl JournalForm {
    pub fn update(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::LoadAccounts => {
                self.model.loading = true;
                let link = ctx.link().clone();
                let api = self.api.clone();

                spawn_local(async move {
                    match api.fetch_accounts().await {
                        Ok(accounts) => {
                            link.send_message(Msg::SetAccounts(accounts));
                        }
                        Err(e) => {
                            let error_msg = e.message;
                            log!(error_msg.clone());
                            link.send_message(Msg::FetchError(error_msg));
                        }
                    }
                });
                true
            }
            Msg::SetDate(date) => {
                self.model.date = date;
                true
            }
            Msg::SetDescription(description) => {
                self.model.description = description;
                true
            }
            Msg::AddEntry => {
                self.model.entries.push(JournalEntryForm::default_debit());
                self.model.entries.push(JournalEntryForm::default_credit());
                true
            }
            Msg::RemoveEntry(index) => {
                // ペアの削除（2エントリーずつ）
                let pair_index = (index / 2) * 2;
                self.model.entries.drain(pair_index..=pair_index + 1);
                true
            }
            Msg::SetEntryAccount(index, account_id) => {
                if let Some(entry) = self.model.entries.get_mut(index) {
                    entry.id = account_id;
                }
                true
            }
            Msg::SetEntryAmount(index, amount) => {
                if let Some(entry) = self.model.entries.get_mut(index) {
                    entry.amount = amount;
                }
                true
            }
            Msg::SetEntryIsDebit(index, is_debit) => {
                if let Some(entry) = self.model.entries.get_mut(index) {
                    entry.is_debit = is_debit;
                }
                true
            }
            Msg::Submit => {
                match self.validate_entries() {
                    Ok(()) => {
                        log!("Validation passed");

                        let on_submit = ctx.props().on_submit.clone();
                        let request = self.create_request();
                        let link = ctx.link().clone();

                        link.send_message(Msg::SubmitStart);

                        let api = self.api.clone();
                        spawn_local(async move {
                            match api.create_journal(request).await {
                                Ok(journal) => {
                                    log!("仕訳の作成に成功");
                                    on_submit.emit(journal);
                                    link.send_message(Msg::SubmitComplete);
                                },
                                Err(e) => {
                                    let error_msg = format!("仕訳の作成に失敗: {}", e.message);
                                    log!(error_msg.clone());
                                    link.send_message(Msg::FetchError(error_msg));
                                    link.send_message(Msg::SubmitComplete);
                                }
                            }
                        });
                    }
                    Err(ValidationError { field: _, message }) => {
                        log!(format!("Validation error: {}", message));
                        self.model.error = Some(message);
                    }
                }
                true
            }
            Msg::SubmitStart => {
                self.model.loading = true;
                true
            }
            Msg::SubmitComplete => {
                self.model.loading = false;
                true
            }
            Msg::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
            Msg::SetAccounts(accounts) => {
                self.model.accounts = accounts;
                self.model.loading = false;
                true
            }
            Msg::FetchError(error) => {
                self.model.error = Some(error);
                self.model.loading = false;
                true
            }
            Msg::InitializeForm(journal) => {
                if let Some(j) = journal {
                    self.model.date = j.date.format("%Y-%m-%d").to_string();
                    self.model.description = j.description;
                    self.model.entries = j.entries.into_iter().map(|e| JournalEntryForm {
                        id: e.account.id,
                        amount: e.amount.to_string(),
                        is_debit: e.is_debit,
                    }).collect();
                } else {
                    self.model.date = chrono::Local::now().format("%Y-%m-%d").to_string();
                    self.model.entries = vec![
                        JournalEntryForm::default_debit(),
                        JournalEntryForm::default_credit(),
                    ];
                }
                true
            }
            Msg::FileSelected(file) => {
                self.model.selected_file = Some(file);
                true
            }
            Msg::ExtractReceiptStart => {
                if let Some(file) = self.model.selected_file.take() {
                    self.model.loading = true;
                    let link = ctx.link().clone();
                    let api = self.api.clone();

                    spawn_local(async move {
                        match api.extract_receipt(file).await {
                            Ok(journal) => {
                                link.send_message(Msg::ExtractReceiptComplete(journal));
                            }
                            Err(e) => {
                                link.send_message(Msg::FetchError(e.message));
                            }
                        }
                    });
                }
                true
            }
            Msg::ExtractReceiptComplete(journal) => {
                self.model.loading = false;
                self.model.date = journal.date.format("%Y-%m-%d").to_string();
                self.model.description = journal.description;
                self.model.entries = journal.entries.into_iter().map(|e| JournalEntryForm {
                    id: e.account.id,
                    amount: e.amount.to_string(),
                    is_debit: e.is_debit,
                }).collect();
                true
            }
        }
    }
}