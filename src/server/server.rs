use std::{
    io::{self, Write},
    net::SocketAddr,
    path::Path,
    process::Command,
};

use crate::utils::color::*;
use axum::Router;
use indicatif::{ProgressBar, ProgressStyle};
use tower_http::services::ServeDir;

/// Starts the embedded web server on the given port.
///
/// If the frontend has not been built yet, automatically installs
/// npm dependencies and compiles the ClojureScript bundle via
/// shadow-cljs. Build commands are platform-aware: `cmd` on Windows,
/// `sh` on Linux/macOS.
///
/// The HTTP server is spawned as a background Tokio task and serves
/// static files from `./web/public`.
///
/// # Errors
/// Returns an error if any build command, socket binding, or
/// server startup fails.
pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("web/public/js/main.js").exists()
        || !Path::new("web/public/css/main.css").exists()
    {
        print!(
            "{}",
            color_info_print(
                "When you first launch the WEB version of Finlog,\nfor your convenience, the necessary dependencies are automatically installed\nand the web application is compiled. Please wait a bit!"
            )
        );
        io::stdout().flush().unwrap();
        println!();

        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner().template("{spinner:.blue} {msg}")?);
        pb.set_message(color_info_print("Building web version..."));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        if !Path::new("web/node_modules").exists() {
            #[cfg(target_os = "windows")]
            Command::new("cmd")
                .args(["/c", "npm install"])
                .current_dir(Path::new("web"))
                .output()?;

            #[cfg(not(target_os = "windows"))]
            Command::new("sh")
                .args(["-c", "npm install"])
                .current_dir(Path::new("web"))
                .output()?;
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/c", "npx sass scss/main.scss:css/main.css scss/expenses.scss:css/expenses.css scss/statistic.scss:css/statistic.css --style=compressed --no-source-map"])
                .current_dir(Path::new("web/public"))
                .output()?;

            Command::new("cmd")
                .args(["/c", "npx shadow-cljs release app"])
                .current_dir(Path::new("web"))
                .output()?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            Command::new("sh")
                .args(["-c", "npx sass scss/main.scss:css/main.css scss/expenses.scss:css/expenses.css scss/statistic.scss:css/statistic.css --style=compressed --no-source-map"])
                .current_dir(Path::new("web/public"))
                .output()?;

            Command::new("sh")
                .args(["-c", "npx shadow-cljs release app"])
                .current_dir(Path::new("web"))
                .output()?;
        }

        pb.finish_and_clear();
        print!("{}", color_info_print("Build completed!"));
        io::stdout().flush().unwrap();
    }

    let app = Router::new().fallback_service(ServeDir::new("./web/public"));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!();
    print!(
        "{}",
        color_info_print(&format!("Web version launched at: http://{addr}/"))
    );
    io::stdout().flush().unwrap();

    print!("{}", color_info_print("Server running in background"));
    io::stdout().flush().unwrap();
    println!();

    Ok(())
}