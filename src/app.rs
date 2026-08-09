use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let stylesheet = format!("/pkg/temidaradev-rust.css?v={}", env!("ASSET_VERSION"));

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <style>
                    "body { background-color: #0b0d10; color: #e2e5e8; }"
                </style>
                <link rel="stylesheet" id="leptos" href=stylesheet/>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" />
                <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
                <link rel="preconnect" href="https://fonts.bunny.net" />
                <link href="https://fonts.bunny.net/css?family=inter:400,500,600,700,800|jetbrains-mono:400,500,600" rel="stylesheet" />
                {
                    // Live-reload client (development builds only). Mirrors the
                    // `#[cfg(debug_assertions)]`-gated /reload-events SSE endpoint
                    // in main.rs, which does not exist in release builds.
                    #[cfg(debug_assertions)]
                    let reload = view! {
                        <script>
                            "use strict";
                            (function() {
                                const evtSource = new EventSource("/reload-events");
                                evtSource.onmessage = function(event) {
                                    console.log("Reloading due to server change...");
                                    window.location.reload();
                                };
                                evtSource.onerror = function(err) {
                                    console.error("EventSource failed:", err);
                                };
                            })();
                        </script>
                    }.into_any();
                    #[cfg(not(debug_assertions))]
                    let reload = ().into_any();
                    reload
                }
                <HydrationScripts options/>
                <MetaTags/>
                {
                    match (std::env::var("UMAMI_SCRIPT_URL"), std::env::var("UMAMI_WEBSITE_ID")) {
                        (Ok(url), Ok(id)) if !url.is_empty() && !id.is_empty() => view! {
                            <script defer src=url data-website-id=id></script>
                        }.into_any(),
                        _ => ().into_any()
                    }
                }
            </head>
            <body class="text-white antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(RwSignal::new(false));

    view! {
        <Title text="temidaradev"/>

        <Router>
            <div class="nix-field" aria-hidden="true">
                <img src="/nixos-logo.png" alt="" />
                <img src="/nixos-logo.png" alt="" />
                <img src="/nixos-logo.png" alt="" />
                <img src="/nixos-logo.png" alt="" />
                <img src="/nixos-logo.png" alt="" />
                <img src="/nixos-logo.png" alt="" />
                <img src="/nixos-logo.png" alt="" />
            </div>
            <div class="site-shell mx-auto min-h-screen max-w-4xl px-4 py-5 sm:px-8 sm:py-6">
                <crate::components::navbar::NavBar />

                <main class="py-4">
                    <Routes fallback=|| "404 - Not Found".into_view()>
                        <Route path=StaticSegment("") view=crate::pages::HomePage/>
                        <Route path=StaticSegment("blog") view=crate::pages::BlogPage/>
                        <Route path=path!("blog/:slug") view=crate::pages::BlogPostPage/>
                        <Route path=StaticSegment("projects") view=crate::pages::ProjectsPage/>
                        <Route path=path!("projects/:slug") view=crate::pages::ProjectPage/>
                        <Route path=StaticSegment("experiences") view=crate::pages::ExperiencePage/>
                        <Route path=StaticSegment("uses") view=crate::pages::UsesPage/>
                        <Route path=StaticSegment("guestbook") view=crate::pages::GuestbookPage/>
                        <Route path=path!("guestbook/moderate") view=crate::pages::GuestbookModerationPage/>
                        <Route path=StaticSegment("contact") view=crate::pages::ContactPage/>
                    </Routes>
                </main>

                <crate::components::footer::Footer />
                <crate::components::CommandPalette />
            </div>
        </Router>
    }
}
