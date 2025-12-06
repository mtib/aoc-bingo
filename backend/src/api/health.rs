use rocket::{get, serde::json::Json};

#[derive(serde::Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

#[get("/health")]
pub fn index() -> Json<HealthResponse> {
    Json(HealthResponse { status: "OK" })
}
