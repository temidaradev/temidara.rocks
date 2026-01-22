#![recursion_limit = "512"]
#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use axum::routing::post;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use temidaradev_rust::app::*;

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

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
