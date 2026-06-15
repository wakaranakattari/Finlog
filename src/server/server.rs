use crate::utils::color::*;
use axum::Router;
use io::Write;
use std::{io, net::SocketAddr};
use tower_http::services::ServeDir;

pub async fn run_server(port: u16) {
    let app = Router::new().fallback_service(ServeDir::new("./web"));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    print!(
        "{}",
        color_info_print(&format!("Web version launched at: http://{addr}/"))
    );
    io::stdout().flush().unwrap();

    println!("{}", color_info_print("Server running in background"));

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}
