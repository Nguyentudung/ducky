use anyhow::{Context, Result};
use directories::{ProjectDirs, UserDirs};
use std::path::{Path, PathBuf};

pub fn resolve_data_dir(override_dir: Option<&Path>) -> Result<PathBuf> {
    if let Some(dir) = override_dir {
        if !dir.is_absolute() {
            anyhow::bail!("Data directory override must be an absolute path");
        }
        return Ok(dir.to_path_buf());
    }
    let dirs = ProjectDirs::from("com", "Ducky", "Ducky")
        .context("Could not determine platform application data directory")?;
    Ok(dirs.data_dir().to_path_buf())
}

pub fn resolve_default_workspace(override_ws: Option<&Path>, data_dir: &Path) -> PathBuf {
    if let Some(ws) = override_ws.filter(|p| p.is_absolute()) {
        return ws.to_path_buf();
    }
    UserDirs::new()
        .and_then(|d| d.document_dir().map(PathBuf::from))
        .unwrap_or_else(|| data_dir.to_path_buf())
        .join("Ducky")
}

pub fn open_browser(url: &str) -> Result<()> {
    open::that_detached(url).with_context(|| format!("Failed to open browser at {url}"))
}

pub fn open_folder(path: &Path) -> Result<()> {
    if !path.is_dir() {
        anyhow::bail!("Directory does not exist: {}", path.display());
    }
    open::that_detached(path).with_context(|| format!("Failed to open folder {}", path.display()))
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sig) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            sig.recv().await;
        } else {
            std::future::pending::<()>().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
