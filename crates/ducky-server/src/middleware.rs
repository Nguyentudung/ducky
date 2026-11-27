use crate::state::AppState;
use axum::{
    Json,
    extract::{Request, State},
    http::{HeaderValue, Method, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub async fn security_guard(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let port = state.port;

    // 1. Host header validation (mitigates DNS rebinding against localhost services)
    if let Some(host_val) = req
        .headers()
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        && !is_allowed_host(host_val, &state.host, port)
    {
        return forbidden("Invalid Host header");
    }

    let path = req.uri().path().to_string();
    let method = req.method().clone();
    let is_api = path.starts_with("/api");

    // 2. Origin & CSRF protection for API requests
    if is_api {
        let origin = req
            .headers()
            .get(header::ORIGIN)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);

        if let Some(ref orig) = origin
            && !is_allowed_origin(orig, &state.host, port, state.dev_origin.as_deref())
        {
            return forbidden("Cross-origin request blocked");
        }

        let is_state_changing = matches!(
            method,
            Method::POST | Method::PUT | Method::PATCH | Method::DELETE
        );

        if is_state_changing {
            if let Some(fetch_site) = req
                .headers()
                .get("sec-fetch-site")
                .and_then(|v| v.to_str().ok())
                && !matches!(fetch_site, "same-origin" | "none")
            {
                let allowed_dev = state.dev_origin.is_some()
                    && origin
                        .as_deref()
                        .is_some_and(|o| state.dev_origin.as_deref() == Some(o));
                if !allowed_dev {
                    return forbidden("Cross-site state-changing request blocked");
                }
            }

            let has_custom_header = req
                .headers()
                .get("x-ducky-request")
                .and_then(|v| v.to_str().ok())
                == Some("1");

            if origin.is_none() && !has_custom_header {
                return forbidden("Missing Origin or X-Ducky-Request header");
            }
        }
    }

    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("same-origin"),
    );

    if let Some(ct) = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        && ct.starts_with("text/html")
    {
        headers.insert(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'",
            ),
        );
    }

    res
}

fn is_allowed_host(host_header: &str, configured_host: &str, port: u16) -> bool {
    let lower = host_header.trim().to_ascii_lowercase();
    let expected = [
        format!("127.0.0.1:{port}"),
        format!("localhost:{port}"),
        format!("[::1]:{port}"),
        format!("{}:{port}", configured_host.to_ascii_lowercase()),
        "127.0.0.1".to_string(),
        "localhost".to_string(),
        "[::1]".to_string(),
    ];
    expected.iter().any(|allowed| allowed == &lower)
}

fn is_allowed_origin(
    origin: &str,
    configured_host: &str,
    port: u16,
    dev_origin: Option<&str>,
) -> bool {
    let lower = origin.trim().trim_end_matches('/').to_ascii_lowercase();
    if let Some(dev) = dev_origin
        && lower == dev.trim().trim_end_matches('/').to_ascii_lowercase()
    {
        return true;
    }
    let allowed = [
        format!("http://127.0.0.1:{port}"),
        format!("http://localhost:{port}"),
        format!("http://[::1]:{port}"),
        format!("http://{}:{port}", configured_host.to_ascii_lowercase()),
    ];
    allowed.iter().any(|a| a == &lower)
}

fn forbidden(msg: &str) -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "error": {
                "code": "forbidden",
                "message": msg
            }
        })),
    )
        .into_response()
}
