mod app;
mod domain;
mod infra;

use app::VAMicDaemon;
use va_lib::VAResult;

#[tokio::main]
async fn main() -> VAResult<()> {
    VAMicDaemon::new().listen().await
}
