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
                <link rel="stylesheet" id="leptos" href="/pkg/leptos_tailwind.css"/>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" />
                <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
                <link rel="stylesheet" href="https://fonts.bunny.net/css?family=inter:400,700"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                {
                    match (std::env::var("UMAMI_SCRIPT_URL"), std::env::var("UMAMI_WEBSITE_ID")) {
                        (Ok(url), Ok(id)) if !url.is_empty() && !id.is_empty() => view! {
                             <script defer src=url data-website-id=id></script>
                        }.into_view(),
                        _ => ().into_view()
                    }
                }
            </head>
            <body class="bg-background text-foreground relative">
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

        <Title text="Temidaradev"/>

        <Router>
            <div class="min-h-screen bg-background relative overflow-hidden selection:bg-primary/30">
                <div class="fixed inset-0 z-0 opacity-30 pointer-events-none">
                    <div class="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-primary/20 rounded-full blur-[120px] animate-float"></div>
                    <div class="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-secondary/20 rounded-full blur-[120px] animate-float" style="animation-delay: -3s;"></div>
                </div>

                <crate::components::navbar::NavBar />

                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
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
