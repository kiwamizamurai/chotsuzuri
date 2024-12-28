use super::*;
use crate::models::validation::ValidationError;
use crate::models::journal_form::{CreateJournalEntryRequest, CreateJournalRequest};

impl JournalForm {
    pub fn validate_entries(&self) -> Result<(), ValidationError> {
        if self.model.date.is_empty() {
            return Err(ValidationError::with_field(
                "date".to_string(),
                "取引日を入力してください。".to_string()
            ));
        }

        if self.model.description.is_empty() {
            return Err(ValidationError::with_field(
                "description".to_string(),
                "取引の内容を入力してください。".to_string()
            ));
        }

        for (i, entry) in self.model.entries.iter().enumerate() {
            let entry_position = if entry.is_debit {
                format!("{}行目の仕訳の借方", (i / 2) + 1)
            } else {
                format!("{}行目の仕訳の貸方", (i / 2) + 1)
            };

            if entry.id <= 0 {
                return Err(ValidationError::with_field(
                    format!("entries[{}].account", i),
                    format!("{}: 勘定科目が選択されていません。", entry_position)
                ));
            }

            match entry.amount.parse::<i32>() {
                Ok(a) if a <= 0 => {
                    return Err(ValidationError::with_field(
                        format!("entries[{}].amount", i),
                        format!("{}: 金額は0より大きい値を入力してください。", entry_position)
                    ))
                }
                Err(_) => {
                    return Err(ValidationError::with_field(
                        format!("entries[{}].amount", i),
                        format!("{}: 金額は半角数字で入力してください。", entry_position)
                    ))
                }
                _ => {}
            }
        }

        let debit_total: i32 = self.model.entries.iter()
            .filter(|e| e.is_debit)
            .filter_map(|e| e.amount.parse::<i32>().ok())
            .sum();
        let credit_total: i32 = self.model.entries.iter()
            .filter(|e| !e.is_debit)
            .filter_map(|e| e.amount.parse::<i32>().ok())
            .sum();

        if debit_total != credit_total {
            return Err(ValidationError::new(format!(
                "借方合計 ¥{} と貸方合計 ¥{} が一致していません。\n仕訳は借方と貸方の金額が同じになるように入力してください。",
                debit_total, credit_total
            )));
        }

        Ok(())
    }

    pub fn create_request(&self) -> CreateJournalRequest {
        CreateJournalRequest {
            date: format!("{}T00:00:00Z", self.model.date),
            description: self.model.description.clone(),
            entries: self.model.entries.iter().map(|entry| CreateJournalEntryRequest {
                account_id: entry.id,
                is_debit: entry.is_debit,
                amount: entry.amount.parse().unwrap_or(0),
            }).collect(),
        }
    }
}