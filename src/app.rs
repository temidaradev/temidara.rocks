use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
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
                <script src="https://cdn.tailwindcss.com"></script>
                <script>
                    "tailwind.config = {\
                        theme: {\
                            extend: {\
                                fontFamily: {\
                                    sans: ['Inter', 'system-ui', 'sans-serif'],\
                                    heading: ['Outfit', 'sans-serif'],\
                                    mono: ['JetBrains Mono', 'monospace'],\
                                },\
                                colors: {\
                                    primary: 'rgb(var(--primary) / <alpha-value>)',\
                                    secondary: 'rgb(var(--secondary) / <alpha-value>)',\
                                    accent: 'rgb(var(--accent) / <alpha-value>)',\
                                    background: 'rgb(var(--background) / <alpha-value>)',\
                                    foreground: 'rgb(var(--foreground) / <alpha-value>)',\
                                    muted: 'rgb(var(--muted) / <alpha-value>)',\
                                    surface: 'rgb(var(--surface) / <alpha-value>)',\
                                },\
                            },\
                        },\
                    }"
                </script>
                <style type="text/tailwindcss">
                    ":root {\
                        --background: 5 5 10;\
                        --surface: 15 15 25;\
                        --primary: 99 102 241;\
                        --primary-dark: 67 56 202;\
                        --secondary: 168 85 247;\
                        --accent: 14 165 233;\
                        --foreground: 226 232 240;\
                        --muted: 148 163 184;\
                        --font-heading: 'Outfit', sans-serif;\
                        --font-body: 'Inter', sans-serif;\
                        --font-mono: 'JetBrains Mono', monospace;\
                    }\
                    @layer base {\
                        html {\
                            font-family: var(--font-body);\
                            scroll-behavior: smooth;\
                        }\
                        h1, h2, h3, h4, h5, h6 {\
                            font-family: var(--font-heading);\
                        }\
                    }\
                    @layer utilities {\
                    }\
                    @layer components {\
                        .blog-content h1 { @apply text-4xl font-bold mt-12 mb-6 text-white tracking-tight; }\
                        .blog-content h2 { @apply text-2xl font-semibold mt-10 mb-5 text-white tracking-tight; }\
                        .blog-content h3 { @apply text-xl font-medium mt-8 mb-4 text-white; }\
                        .blog-content p { @apply text-lg leading-8 text-muted mb-6; }\
                        .blog-content ul { @apply list-disc list-outside ml-6 space-y-2 text-muted mb-6; }\
                        .blog-content ol { @apply list-decimal list-outside ml-6 space-y-2 text-muted mb-6; }\
                        .blog-content a { @apply text-primary hover:text-accent underline underline-offset-4 decoration-2 decoration-primary/30 hover:decoration-accent transition-colors; }\
                        .blog-content pre { @apply bg-[#0d1117] border border-white/10 rounded-lg p-6 my-8 overflow-x-auto; }\
                        .blog-content code { \
                            @apply font-mono text-sm bg-white/10 px-1.5 py-0.5 rounded text-accent;\
                            font-family: var(--font-mono);\
                        }\
                        .blog-content pre code { @apply bg-transparent p-0 text-gray-300; }\
                        .blog-content blockquote { @apply border-l-4 border-primary/50 pl-6 italic text-muted my-8; }\
                        .blog-content img { @apply rounded-lg shadow-2xl border border-white/5 my-8 w-full; }\
                    }"
                </style>
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
