use crate::client::messaging::{Request, Response};
use crate::embedded::server::Server;
use axum::{extract::State, http::StatusCode, Json};
use tracing::{debug, info};

pub(super) async fn run_query(
    State(database): State<Server>,
    Json(payload): Json<Request>,
) -> (StatusCode, Json<Response>) {
    let Request { query, vars, id } = payload;
    info!("Incoming request with ID:{id}");

    let response = database.query(query, vars).await;
    debug!("Sending response for {id} -> {response:?}");

    match response {
        Ok(results) => (StatusCode::OK, Json(Response { id, results })),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(Response {
                id,
                results: vec![error.as_response()],
            }),
        ),
    }
}
