use crate::components::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
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

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background relative overflow-hidden selection:bg-primary/30">
            // Subtle animated background mesh
            <div class="fixed inset-0 z-0 opacity-30">
                <div class="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-primary/20 rounded-full blur-[120px] animate-float"></div>
                <div class="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-secondary/20 rounded-full blur-[120px] animate-float" style="animation-delay: -3s;"></div>
            </div>

            <NavBar />

            <main class="relative z-10 max-w-7xl mx-auto px-6 py-32 lg:py-48">
                <div class="grid lg:grid-cols-2 gap-16 items-center">
                    // Hero Text
                    <div class="space-y-8">
                        <div class="space-y-2">
                            <span class="font-mono text-accent text-sm tracking-wider">"Hello World, I am"</span>
                            <h1 class="text-6xl lg:text-8xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-br from-white via-white to-white/50">
                                "Temidara"
                            </h1>
                            <Text variant="gradient" size="xl" weight="bold" class="text-2xl lg:text-3xl opacity-90">
                                "Systems Engineer & Rustacean"
                            </Text>
                        </div>

                        <p class="text-muted text-lg leading-relaxed max-w-xl">
                            "Crafting high-performance systems with a touch of elegance.
                            Specializing in Rust, distributed systems, and cute UI interactions."
                        </p>

                        <div class="flex gap-4 pt-4">
                            <Button text="View Projects" variant="primary" size="lg" />
                            <Button text="Contact Me" variant="outline" size="lg" />
                        </div>

                        // Terminal-style decoration
                        <div class="pt-12 font-mono text-sm text-muted">
                            <div class="flex gap-2 items-center">
                                <span class="text-secondary">"$"</span>
                                <span>"cargo run --release"</span>
                            </div>
                            <div class="flex gap-2 items-center text-green-400 mt-1">
                                <span>"Compiling"</span>
                                <span class="text-muted">"portfolio v2.0.0"</span>
                            </div>
                            <div class="flex gap-2 items-center text-green-400 mt-1">
                                <span>"Finished"</span>
                                <span class="text-muted">"release [optimized] target(s) in 0.42s"</span>
                            </div>
                        </div>
                    </div>

                    // Hero Visual / Bento Grid
                    <div class="hidden lg:flex justify-center items-center relative perspective-1000">
                         <div class="relative w-[500px] h-[500px] flex justify-center items-center">
                            // Glow effect behind
                            <div class="absolute inset-0 bg-gradient-to-tr from-primary/20 to-secondary/20 rounded-full blur-[80px] animate-pulse"></div>

                            // Bento Grid Container
                            <div class="relative z-10 grid grid-cols-3 gap-3 w-full max-w-[480px]">
                                // Row 1: Primary Langs + Experience
                                <LanguageCard
                                    name="Rust"
                                    meta="Systems"
                                    class="col-span-2 bg-[#b7410e]/10 border-[#b7410e]/20 hover:border-[#b7410e]/50"
                                />
                                <StatCard
                                    value="5+"
                                    label="Years"
                                    class="bg-primary/5 border-primary/10"
                                />

                                // Row 2: Location (Wide) + TypeScript
                                <InfoCard
                                    text="Turkiye"
                                    subtext="UTC+3"
                                    class="col-span-2 bg-accent/5 border-accent/10"
                                />
                                <LanguageCard
                                    name="TS"
                                    meta="Web"
                                    class="bg-[#3178c6]/10 border-[#3178c6]/20"
                                />

                                // Row 3: Plex (Prominent Center)
                                <PlexCard
                                    song="Starboy"
                                    artist="The Weeknd"
                                    class="col-span-3 bg-background/80 backdrop-blur-md shadow-lg border-white/5"
                                />

                                // Row 4: Stats & Secondary Tech
                                <StatCard
                                    value="20+"
                                    label="Projects"
                                    class="bg-secondary/5 border-secondary/10"
                                />
                                <LanguageCard
                                    name="Axum"
                                    meta="Backend"
                                />
                                <LanguageCard
                                    name="Docker"
                                    meta="DevOps"
                                />

                                // Row 5: Footer Tech
                                <LanguageCard
                                    name="React"
                                    meta="UI"
                                    class="col-span-1"
                                />
                                <LanguageCard
                                    name="SQL"
                                    meta="Data"
                                    class="col-span-2"
                                />
                            </div>
                        </div>
                    </div>
                </div>

            </main>
        </div>
    }
}
