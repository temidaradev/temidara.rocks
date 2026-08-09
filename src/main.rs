#![recursion_limit = "512"]
#[cfg(feature = "ssr")]
use axum::routing::post;
#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use temidaradev_rust::app::*;

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
