use crate::query::{QueryRequest, QueryResponse};
use axum::{extract::State, http::StatusCode, Json};
use eight::embedded::Server;

pub async fn run_query(
    State(database): State<Server>,
    Json(payload): Json<QueryRequest>,
) -> (StatusCode, Json<QueryResponse>) {
    let QueryRequest { query, vars, id } = payload;
    let response = database.query(query, vars).await;

    match response {
        Ok(results) => (StatusCode::OK, Json(QueryResponse { id, results })),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(QueryResponse {
                id,
                results: vec![error.as_response()],
            }),
        ),
    }
}
