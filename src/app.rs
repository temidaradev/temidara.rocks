use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" id="leptos" href="/pkg/temidaradev-rust.css"/>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" />
                <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
                <link rel="preconnect" href="https://fonts.bunny.net" />
                <link href="https://fonts.bunny.net/css?family=inter:400,500,600,700|jetbrains-mono:400,500,600|outfit:400,500,600,700" rel="stylesheet" />
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
            <body class="bg-black text-white antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/temidaradev-rust.css"/>

        <Title text="temidaradev"/>

        <Router>
            <div class="min-h-screen p-6 max-w-3xl mx-auto">
                <crate::components::navbar::NavBar />

                <main class="py-8">
                    <Routes fallback=|| "404 - Not Found".into_view()>
                        <Route path=StaticSegment("") view=crate::pages::HomePage/>
                        <Route path=StaticSegment("blog") view=crate::pages::BlogPage/>
                        <Route path=path!("blog/:slug") view=crate::pages::BlogPostPage/>
                        <Route path=StaticSegment("experiences") view=crate::pages::ExperiencePage/>
                        <Route path=StaticSegment("contact") view=crate::pages::ContactPage/>
                    </Routes>
                </main>

                <crate::components::footer::Footer />
            </div>
        </Router>
    }
}
