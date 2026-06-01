#![recursion_limit = "256"]

pub mod app;

pub async fn run_main() {
    if let Err(error) = app::run().await {
        if app::args_request_json() {
            let payload = serde_json::json!({
                "code": 1,
                "msg": "error",
                "error": format!("{error:#}"),
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&payload)
                    .unwrap_or_else(|_| "{\"code\":1,\"msg\":\"error\"}".to_string())
            );
        } else {
            eprintln!("Error: {error:#}");
        }
        std::process::exit(1);
    }
}
