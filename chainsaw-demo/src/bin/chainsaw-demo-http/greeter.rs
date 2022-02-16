use axum::extract::Path;

#[tracing::instrument]
pub async fn greeter(Path((name, surname)): Path<(String, String)>) -> String {
    format!("Hello, {name} {surname}!\n")
}
