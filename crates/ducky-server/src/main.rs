mod config;
mod handlers;
mod middleware;
mod platform;
mod server;
mod state;
mod static_files;

use clap::Parser;
use config::ServerConfig;
use ducky_core::{db::init_pool, settings::ensure_workspace_initialized};
use state::AppState;
use std::io::ErrorKind;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("\n   Error: {err}\n");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .without_time()
        .init();

    let cfg = ServerConfig::parse();
    let version = env!("CARGO_PKG_VERSION");

    println!("\n       Ducky v{version}\n");
    println!("   Initializing...");

    if !cfg.is_loopback() {
        eprintln!(
            "   Warning: binding to non-loopback address '{}' exposes Ducky on the network.",
            cfg.host
        );
    }

    let data_dir = platform::resolve_data_dir(cfg.data_dir.as_deref())?;
    let db_path = data_dir.join("ducky.db");
    let default_workspace =
        platform::resolve_default_workspace(cfg.workspace.as_deref(), &data_dir);

    println!("   Preparing application...");
    let pool = init_pool(&db_path).await?;
    let _ = ensure_workspace_initialized(&pool, &default_workspace).await?;

    let addr = format!("{}:{}", cfg.host, cfg.port);
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) if e.kind() == ErrorKind::AddrInUse => {
            anyhow::bail!(
                "Port {} is already in use (another Ducky instance may already be running).\n   Try passing --port <PORT> or opening http://{}:{}",
                cfg.port,
                cfg.host,
                cfg.port
            );
        }
        Err(e) => {
            anyhow::bail!("Could not bind local server to {addr}: {e}");
        }
    };

    let bound_addr = listener.local_addr()?;
    let url = format!("http://{}:{}", cfg.host, bound_addr.port());

    let state = AppState {
        pool: pool.clone(),
        default_workspace,
        host: cfg.host.clone(),
        port: bound_addr.port(),
        dev_origin: cfg.dev_origin.clone(),
        version,
    };

    let router = server::build_router(state);

    println!("\n   Local server is ready.\n");
    println!("   {url}\n");

    if !cfg.no_open {
        println!("   Opening browser...");
        if let Err(err) = platform::open_browser(&url) {
            eprintln!("   Note: could not auto-open browser ({err}). Open {url} manually.");
        }
    }

    println!("   Press Ctrl+C to exit.\n");

    axum::serve(listener, router)
        .with_graceful_shutdown(platform::shutdown_signal())
        .await?;

    pool.close().await;
    println!("   Ducky stopped cleanly.");
    Ok(())
}
