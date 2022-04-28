use axum::extract::{Extension, Path};
use prometheus::Counter;

/// HTTP endpoint that greets the given name and surname.
///
/// Also takes a Prometheus counter extension and increments the counter by 1 on
/// every call.
#[tracing::instrument]
pub async fn greeter(
    Path((name, surname)): Path<(String, String)>,
    Extension(counter_metric): Extension<Counter>,
) -> String {
    counter_metric.inc();
    format!("Hello, {name} {surname}!\n")
}
