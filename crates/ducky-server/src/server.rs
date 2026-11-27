use crate::{handlers, middleware, state::AppState, static_files};
use axum::{
    Router, middleware as axum_mw,
    routing::{delete, get, patch, post},
};

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/health", get(handlers::health))
        .route("/dashboard", get(handlers::get_dashboard))
        .route(
            "/problems",
            get(handlers::problems_list).post(handlers::problems_create),
        )
        .route(
            "/problems/{id}",
            get(handlers::problems_detail)
                .patch(handlers::problems_update)
                .delete(handlers::problems_delete),
        )
        .route("/problems/{id}/files", post(handlers::problems_add_file))
        .route(
            "/problems/{id}/readme",
            post(handlers::problems_generate_readme),
        )
        .route(
            "/problems/{id}/open-folder",
            post(handlers::problems_open_folder),
        )
        .route("/recent", delete(handlers::recent_clear))
        .route(
            "/languages",
            get(handlers::languages_list).post(handlers::languages_create),
        )
        .route(
            "/languages/{id}",
            patch(handlers::languages_update).delete(handlers::languages_delete),
        )
        .route(
            "/templates",
            get(handlers::templates_list).post(handlers::templates_create),
        )
        .route(
            "/templates/{id}",
            patch(handlers::templates_update).delete(handlers::templates_delete),
        )
        .route(
            "/settings",
            get(handlers::settings_get).patch(handlers::settings_patch),
        )
        .route("/workspace", get(handlers::workspace_get))
        .route("/workspace/validate", post(handlers::workspace_validate))
        .route("/workspace/open", post(handlers::workspace_open));

    Router::new()
        .nest("/api", api)
        .fallback(static_files::serve_static)
        .layer(axum_mw::from_fn_with_state(
            state.clone(),
            middleware::security_guard,
        ))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
    };
    use ducky_core::db::init_memory_pool;
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tower::ServiceExt;

    fn temp_ws() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("ducky-server-test-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    async fn test_app(ws: PathBuf) -> Router {
        let pool = init_memory_pool().await.unwrap();
        let state = AppState {
            pool,
            default_workspace: ws,
            host: "127.0.0.1".into(),
            port: 8090,
            dev_origin: None,
            version: "0.1.0",
        };
        build_router(state)
    }

    async fn body_json(res: axum::response::Response) -> Value {
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_and_static_routes_work() {
        let ws = temp_ws();
        let app = test_app(ws.clone()).await;

        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .header(header::HOST, "127.0.0.1:8090")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = body_json(res).await;
        assert_eq!(body["status"], "ok");
        assert_eq!(body["database"], true);

        let unknown_api = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/nonexistent")
                    .header(header::HOST, "127.0.0.1:8090")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unknown_api.status(), StatusCode::NOT_FOUND);

        let spa_route = app
            .oneshot(
                Request::builder()
                    .uri("/problems/42")
                    .header(header::HOST, "127.0.0.1:8090")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(spa_route.status(), StatusCode::OK);

        std::fs::remove_dir_all(&ws).unwrap();
    }

    #[tokio::test]
    async fn security_guard_blocks_dns_rebinding_and_csrf() {
        let ws = temp_ws();
        let app = test_app(ws.clone()).await;

        // Foreign Host header rejected
        let bad_host = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .header(header::HOST, "evil.example:8090")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(bad_host.status(), StatusCode::FORBIDDEN);

        // Foreign Origin rejected
        let bad_origin = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/problems")
                    .header(header::HOST, "127.0.0.1:8090")
                    .header(header::ORIGIN, "https://evil.com")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(bad_origin.status(), StatusCode::FORBIDDEN);

        // State-changing request without Origin or X-Ducky-Request rejected
        let missing_csrf = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/problems")
                    .header(header::HOST, "127.0.0.1:8090")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(missing_csrf.status(), StatusCode::FORBIDDEN);

        std::fs::remove_dir_all(&ws).unwrap();
    }

    #[tokio::test]
    async fn api_problem_crud_and_settings_flow() {
        let ws = temp_ws();
        let app = test_app(ws.clone()).await;

        let create_payload = json!({
            "title": "Binary Search",
            "platform": "LeetCode",
            "difficulty": "easy",
            "languages": ["rust"],
            "status": "not_started"
        });
        let created = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/problems")
                    .header(header::HOST, "127.0.0.1:8090")
                    .header(header::ORIGIN, "http://127.0.0.1:8090")
                    .header("x-ducky-request", "1")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(create_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(created.status(), StatusCode::CREATED);
        let created_json = body_json(created).await;
        let id = created_json["problem"]["id"].as_i64().unwrap();

        let patched = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("/api/problems/{id}"))
                    .header(header::HOST, "127.0.0.1:8090")
                    .header("x-ducky-request", "1")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({ "status": "solved" }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patched.status(), StatusCode::OK);
        let patched_json = body_json(patched).await;
        assert_eq!(patched_json["problem"]["status"], "solved");

        let deleted = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("/api/problems/{id}"))
                    .header(header::HOST, "127.0.0.1:8090")
                    .header("x-ducky-request", "1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(deleted.status(), StatusCode::OK);

        std::fs::remove_dir_all(&ws).unwrap();
    }
}
