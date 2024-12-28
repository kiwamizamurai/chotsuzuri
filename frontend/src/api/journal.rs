use super::error::ApiError;
use gloo_console::log;
use gloo_net::http::Request;
use crate::models::{journal::{Account, GraphQLJournalsResponse, Journal}, journal_form::{CreateJournalRequest, CreateJournalResponse}};
use serde_json;
use chrono::{DateTime, Utc};
use super::error::ApiResult;

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

    pub async fn fetch_journals(&self, page: i32) -> ApiResult<GraphQLJournalsResponse> {
        let query = r#"
            query ListJournals($page: Int!) {
                journals(
                    pagination: { page: $page, perPage: 5 }
                    filter: {}
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

        let request_body = serde_json::json!({
            "query": query,
            "variables": {
                "page": page
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
                Err(ApiError::new(format!("failed to parse json: {}", e), Some(status)))
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
}