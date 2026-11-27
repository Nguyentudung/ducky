use axum::{
    Json,
    http::{HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::Embed;
use serde_json::json;

#[derive(Embed)]
#[folder = "../../frontend/dist"]
struct FrontendAssets;

pub async fn serve_static(uri: Uri) -> Response {
    let raw_path = uri.path().trim_start_matches('/');

    // Never fall back to index.html for unknown /api routes
    if raw_path == "api" || raw_path.starts_with("api/") {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": {
                    "code": "not_found",
                    "message": "API endpoint not found"
                }
            })),
        )
            .into_response();
    }

    let path = if raw_path.is_empty() {
        "index.html"
    } else {
        raw_path
    };

    if let Some(asset) = FrontendAssets::get(path) {
        return asset_response(path, asset.data.into_owned());
    }

    // SPA fallback for client-side routes without an explicit file extension
    if !path.contains('.')
        && let Some(index) = FrontendAssets::get("index.html")
    {
        return asset_response("index.html", index.data.into_owned());
    }

    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

fn asset_response(path: &str, bytes: Vec<u8>) -> Response {
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let cache = if path.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };

    let mut res = (StatusCode::OK, bytes).into_response();
    let headers = res.headers_mut();
    if let Ok(ct) = HeaderValue::from_str(mime.as_ref()) {
        headers.insert(header::CONTENT_TYPE, ct);
    }
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));
    res
}
