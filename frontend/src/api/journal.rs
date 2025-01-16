use super::error::ApiError;
use gloo_console::log;
use gloo_net::http::Request;
use crate::models::{
    journal::{Account, GraphQLJournalsResponse, Journal},
    journal_form::{CreateJournalRequest, CreateJournalResponse},
    journal_list::JournalFilter,
};
use serde_json;
use chrono::{DateTime, Utc};
use super::error::ApiResult;
use wasm_bindgen::JsCast;
use web_sys::{Blob, File, FormData};

#[derive(Clone)]
pub struct JournalApi {
    endpoint: String,
}

impl JournalApi {
    pub fn new() -> Self {
        Self {
            endpoint: "http://localhost:8080".to_string(),
        }
    }

    pub async fn fetch_journals(&self, page: i32, filter: JournalFilter) -> ApiResult<GraphQLJournalsResponse> {
        let query = r#"
            query ListJournals($page: Int!, $filter: JournalFilter) {
                journals(
                    pagination: { page: $page, perPage: 5 }
                    filter: $filter
                ) {
                    items {
                        id
                        journalNumber
                        date
                        description
                        entries {
                            id
                            account {
                                id
                                code
                                name
                                accountType
                            }
                            isDebit
                            amount
                        }
                        createdAt
                        updatedAt
                    }
                    pageInfo {
                        hasNextPage
                        hasPrevPage
                        totalPages
                        currentPage
                    }
                }
            }
        "#;

        let mut filter_obj = serde_json::Map::new();

        if let Some(date_from) = filter.date_from {
            let mut date_range = serde_json::Map::new();
            date_range.insert("from".to_string(), serde_json::Value::String(date_from));
            if let Some(date_to) = filter.date_to {
                date_range.insert("to".to_string(), serde_json::Value::String(date_to));
            }
            filter_obj.insert("dateRange".to_string(), serde_json::Value::Object(date_range));
        }

        if let Some(account_code) = filter.account_code {
            filter_obj.insert(
                "accountCodes".to_string(),
                serde_json::Value::Array(vec![serde_json::Value::String(account_code)]),
            );
        }

        if filter.amount_min.is_some() || filter.amount_max.is_some() {
            let mut amount_range = serde_json::Map::new();
            if let Some(min) = filter.amount_min {
                amount_range.insert("min".to_string(), serde_json::Value::Number(min.into()));
            }
            if let Some(max) = filter.amount_max {
                amount_range.insert("max".to_string(), serde_json::Value::Number(max.into()));
            }
            filter_obj.insert("amountRange".to_string(), serde_json::Value::Object(amount_range));
        }

        let request_body = serde_json::json!({
            "query": query,
            "variables": {
                "page": page,
                "filter": filter_obj
            }
        });

        let response = Request::post(&format!("{}/query", self.endpoint))
            .json(&request_body)
            .map_err(|e| ApiError::new(e.to_string(), None))?
            .send()
            .await
            .map_err(|e| ApiError::new(e.to_string(), None))?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| ApiError::new(e.to_string(), Some(status)))?;

        log!(format!("Raw Response: {}", response_text));

        match serde_json::from_str::<GraphQLJournalsResponse>(&response_text) {
            Ok(response) => {
                log!(format!("Parsed Response: {:?}", response));
                Ok(response)
            }
            Err(e) => {
                log!(format!("Parse Error: {}", e));
                Err(ApiError::new(format!("JSONのパースに失敗: {}", e), Some(status)))
            }
        }
    }

    pub async fn create_journal(&self, request: CreateJournalRequest) -> ApiResult<Journal> {
        log!(format!("Request: {:?}", request));

        let response = Request::post(&format!("{}/api/journals", self.endpoint))
            .json(&request)
            .map_err(|e| ApiError::new(e.to_string(), None))?
            .send()
            .await
            .map_err(|e| ApiError::new(e.to_string(), None))?;

        let status = response.status();
        if status != 201 {
            return Err(ApiError::new(
                format!("Unexpected status code: {}", status),
                Some(status)
            ));
        }

        let text = response
            .text()
            .await
            .map_err(|e| ApiError::new(e.to_string(), Some(status)))?;

        match serde_json::from_str::<CreateJournalResponse>(&text) {
            Ok(response) => {
                let date = DateTime::parse_from_rfc3339(&response.date)
                    .map_err(|e| ApiError::new(format!("Invalid date format: {}", e), None))?
                    .with_timezone(&Utc);

                Ok(Journal {
                    id: "temp".to_string(),
                    journal_number: "temp".to_string(),
                    date,
                    description: response.description,
                    entries: vec![],
                })
            },
            Err(e) => {
                log!(format!("Struct parse error: {}", e));
                Err(ApiError::new(format!("Failed to parse response into struct: {}", e), Some(status)))
            }
        }
    }

    pub async fn fetch_accounts(&self) -> ApiResult<Vec<Account>> {
        let response = Request::get(&format!("{}/api/accounts", self.endpoint))
            .send()
            .await
            .map_err(|e| ApiError::new(e.to_string(), None))?;

        let status = response.status();
        response
            .json::<Vec<Account>>()
            .await
            .map_err(|e| ApiError::new(e.to_string(), Some(status)))
    }

    pub async fn extract_receipt(&self, file: File) -> Result<Journal, ApiError> {
        let form_data = FormData::new().map_err(|_| ApiError {
            message: "Failed to create FormData".to_string(),
            status_code: None,
        })?;

        let file_name = file.name();
        let blob: Blob = file.unchecked_into();
        form_data.append_with_blob_and_filename(
            "file",
            &blob,
            file_name.as_str(),
        ).map_err(|_| ApiError {
            message: "Failed to append file to FormData".to_string(),
            status_code: None,
        })?;

        let response = Request::post("http://localhost:8888/api/extract-receipt-ocr-llm")
            .body(&form_data)
            .send()
            .await
            .map_err(|e| ApiError {
                message: format!("Network error: {}", e),
                status_code: None,
            })?;

        if response.ok() {
            response.json().await.map_err(|e| ApiError {
                message: format!("Failed to parse response: {}", e),
                status_code: None,
            })
        } else {
            Err(ApiError {
                message: format!("Server error: {}", response.status()),
                status_code: Some(response.status()),
            })
        }
    }
}