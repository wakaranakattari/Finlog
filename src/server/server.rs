use crate::utils::color::*;
use axum::Router;
use io::Write;
use std::{io, net::SocketAddr, process::Command};
use tower_http::services::ServeDir;
use std::path::Path;

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
    if !Path::new("web/public/js/main.js").exists() {
        println!();
        print!("{}", color_info_print("Building web version..."));
        io::stdout().flush().unwrap();

        if !Path::new("web/node_modules").exists() {
            #[cfg(target_os = "windows")]
            Command::new("cmd")
                .args(["/c", "cd /d web && npm install"])
                .output()?;

            #[cfg(not(target_os = "windows"))]
            Command::new("sh")
                .args(["-c", "cd web && npm install"])
                .output()?;
        }

        #[cfg(target_os = "windows")]
        Command::new("cmd")
            .args(["/c", "cd /d web && npx shadow-cljs release app"])
            .output()?;

        #[cfg(not(target_os = "windows"))]
        Command::new("sh")
            .args(["-c", "cd web && npx shadow-cljs release app"])
            .output()?;

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