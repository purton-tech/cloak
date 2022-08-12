use crate::ructe::templates::statics::{index_css_map, index_js_map, StaticFile};
use axum::body::{self, Body, Empty};
use axum::extract::Path;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(body::boxed(Body::from(data.content)))
            .unwrap()
    } else {
        // Perhaps there is no hash, like index.css.map and index.js.map
        if path == "index.css.map" {
            let data = &index_css_map;
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(data.mime.as_ref()).unwrap(),
                )
                .body(body::boxed(Body::from(data.content)))
                .unwrap()
        } else if path == "index.js.map" {
            let data = &index_js_map;
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(data.mime.as_ref()).unwrap(),
                )
                .body(body::boxed(Body::from(data.content)))
                .unwrap()
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap()
        }
    }
}
