#![recursion_limit = "512"]
#[cfg(feature = "ssr")]
use axum::routing::{get, post};
#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use temidaradev_rust::app::*;

#[cfg(feature = "ssr")]
async fn cached_cover(
    axum::extract::Path(file): axum::extract::Path<String>,
) -> axum::response::Response {
    use axum::http::{header, StatusCode};
    use axum::response::IntoResponse;

    if !file.ends_with(".img")
        || file.len() != 20
        || !file[..16]
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return StatusCode::NOT_FOUND.into_response();
    }

    let directory = std::env::var("COVER_CACHE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("data/covers"));
    let image_path = directory.join(&file);
    let mime_path = directory.join(file.replace(".img", ".mime"));
    match std::fs::read(image_path) {
        Ok(bytes) => {
            let mime =
                std::fs::read_to_string(mime_path).unwrap_or_else(|_| "image/jpeg".to_string());
            (
                [
                    (header::CONTENT_TYPE, mime),
                    (
                        header::CACHE_CONTROL,
                        "public, max-age=31536000, immutable".to_string(),
                    ),
                ],
                bytes,
            )
                .into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(feature = "ssr")]
async fn blog_asset(
    axum::extract::Path(file): axum::extract::Path<String>,
) -> axum::response::Response {
    use axum::http::{header, StatusCode};
    use axum::response::IntoResponse;

    match temidaradev_rust::pages::blog::get_blog_asset(&file) {
        Some((mime, bytes)) => (
            [
                (header::CONTENT_TYPE, mime.to_string()),
                (header::CACHE_CONTROL, "public, max-age=3600".to_string()),
            ],
            axum::body::Bytes::from_static(bytes),
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(all(feature = "ssr", debug_assertions))]
async fn sse_handler(
    axum::Extension(tx): axum::Extension<tokio::sync::broadcast::Sender<String>>,
) -> impl axum::response::IntoResponse {
    use axum::response::sse::{Event, Sse};
    use tokio_stream::wrappers::BroadcastStream;
    use tokio_stream::StreamExt;

    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|msg| -> Result<Event, std::convert::Infallible> {
        match msg {
            Ok(msg) => Ok(Event::default().data(msg)),
            Err(_) => Ok(Event::default().comment("keep-alive")),
        }
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/{*fn_name}", post(leptos_axum::handle_server_fns))
        .route("/cover-cache/{file}", get(cached_cover))
        .route("/blog-assets/{*file}", get(blog_asset))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Live-reload (development builds only). A filesystem watcher pushes
    // "reload" events to the browser over a persistent SSE connection.
    //
    // This is deliberately excluded from release builds: SSE connections never
    // close, so each one holds a socket/file descriptor open for its lifetime.
    // Left exposed in production it is a cheap DoS vector — an attacker opens
    // many concurrent connections and exhausts the process's file-descriptor
    // limit (`ulimit -n`), after which no legitimate client can connect.
    #[cfg(debug_assertions)]
    let app = {
        use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
        use std::path::Path;

        let (tx, _rx) = tokio::sync::broadcast::channel(100);
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let (mut watcher_tx, mut watcher_rx) = tokio::sync::mpsc::channel(100);

            let mut watcher = RecommendedWatcher::new(
                move |res| {
                    let _ = watcher_tx.blocking_send(res);
                },
                Config::default(),
            )
            .unwrap();

            let watch_dirs = ["src", "style", "public", "input.css"];
            for dir in &watch_dirs {
                let path = Path::new(dir);
                if path.exists() {
                    if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
                        eprintln!("Watcher error for {}: {:?}", dir, e);
                    }
                }
            }

            while let Some(res) = watcher_rx.recv().await {
                match res {
                    Ok(event) => {
                        if matches!(
                            event.kind,
                            notify::EventKind::Modify(_)
                                | notify::EventKind::Create(_)
                                | notify::EventKind::Remove(_)
                        ) {
                            let _ = tx_clone.send("reload".to_string());
                        }
                    }
                    Err(e) => eprintln!("watch error: {:?}", e),
                }
            }
        });

        app.route("/reload-events", axum::routing::get(sse_handler))
            .layer(axum::Extension(tx))
    };

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
