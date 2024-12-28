use crate::models::journal::GraphQLJournalsResponse;

#[derive(Debug)]
pub enum Msg {
    LoadJournals,
    JournalsLoaded(GraphQLJournalsResponse),
    Error(String),
    LoadNextPage,
    LoadPreviousPage,
}