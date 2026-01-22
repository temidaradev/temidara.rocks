use crate::components::*;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (trigger, set_trigger) = signal(0);

    Effect::new(move |_| {
        let handle = set_interval_with_handle(
            move || set_trigger.update(|n| *n += 1),
            std::time::Duration::from_secs(2),
        )
        .ok();

        on_cleanup(move || {
            if let Some(h) = handle {
                h.clear();
            }
        });
    });

    let plex_resource = Resource::new(
        move || trigger.get(),
        |_| async move {
            crate::api::plex::get_plex_current_track()
                .await
                .unwrap_or(None)
        },
    );

    view! {
        <div class="relative z-10">
            <main class="max-w-7xl mx-auto px-6 py-32 lg:py-48">
                <div class="grid lg:grid-cols-2 gap-16 items-start">
                    <div class="space-y-8">
                        <div class="space-y-2">
                            <span class="font-mono text-accent text-sm tracking-wider">"Hello World, I am"</span>
                            <h1 class="text-6xl lg:text-8xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-br from-white via-white to-white/50">
                                "Temidaradev"
                            </h1>
                            <Text variant="gradient" size="xl" weight="bold" class="text-2xl lg:text-3xl opacity-90">
                                "Systems Enjoyer & Rustacean"
                            </Text>
                        </div>

                        <p class="text-muted text-lg leading-relaxed max-w-xl">
                            "Building stuff with Rust and Go, breaking things along the way.
                            I like making tools that actually work and UIs that don't hurt to look at."
                        </p>

                        <div class="flex gap-4 pt-4">
                            <a href="https://github.com/temidaradev"><Button text="View Projects" variant="primary" size="lg" /></a>
                            <a href="/contact"><Button text="Contact Me" variant="outline" size="lg" /></a>
                        </div>

                        <div class="pt-12 font-mono text-sm text-muted space-y-4">
                             <div class="border border-primary/20 bg-surface/30 backdrop-blur-sm rounded-xl p-4 max-w-sm">
                                <div class="flex items-center gap-2 text-xs text-muted mb-3 pb-2 border-b border-white/5">
                                    <div class="flex gap-1.5">
                                        <span class="w-2.5 h-2.5 rounded-full bg-primary/80"></span>
                                        <span class="w-2.5 h-2.5 rounded-full bg-secondary/80"></span>
                                        <span class="w-2.5 h-2.5 rounded-full bg-accent/80"></span>
                                    </div>
                                    <span class="font-mono opacity-60">"status"</span>
                                </div>
                                <div class="space-y-3">
                                    <div class="text-xs text-muted space-y-1 pt-1">
                                        <p>"17 / Rust and Go / Türkiye 🇹🇷"</p>
                                        <p class="text-foreground/60">"Coding and psychology enjoyer"</p>
                                    </div>
                                    <div class="flex items-center gap-4 pt-1">
                                        <a href="https://github.com/temidaradev" class="text-muted hover:text-primary transition-all duration-200 hover:-translate-y-0.5">
                                            <i class="fa-brands fa-github text-lg"></i>
                                        </a>
                                        <a href="https://x.com/temidaradev" class="text-muted hover:text-[#1DA1F2] transition-all duration-200 hover:-translate-y-0.5">
                                            <i class="fa-brands fa-x-twitter text-lg"></i>
                                        </a>
                                        <a href="https://discord.gg/" class="text-muted hover:text-[#5865F2] transition-all duration-200 hover:-translate-y-0.5">
                                            <i class="fa-brands fa-discord text-lg"></i>
                                        </a>
                                    </div>
                                </div>
                            </div>

                             <div class="pt-8">
                                <Transition fallback=move || view! { <div class="text-muted animate-pulse">"Scanning frequencies..."</div> }>
                                    {
                                        move || {
                                            plex_resource.get().map(|track| view! {
                                                    <PlexCard track=track class="max-w-sm w-full" />
                                            })
                                        }
                                    }
                                </Transition>
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-col gap-8">
                        <div>
                             <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-2">
                                <i class="fa-solid fa-code text-primary"></i>
                                "Languages & Tech"
                             </h3>
                             <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
                                <LanguageCard name="Rust" meta="Systems" class="border-orange-500/30 bg-orange-500/5" />
                                <LanguageCard name="TypeScript" meta="Frontend" class="border-blue-500/30 bg-blue-500/5" />
                                <LanguageCard name="Go" meta="Backend" class="border-cyan-500/30 bg-cyan-500/5" />
                                <LanguageCard name="Python" meta="Scripting" class="border-yellow-500/30 bg-yellow-500/5" />
                                <LanguageCard name="Docker" meta="DevOps" class="border-blue-600/30 bg-blue-600/5" />
                                <LanguageCard name="Linux" meta="Environment" class="border-white/20 bg-white/5" />
                             </div>
                        </div>

                        <div>
                            <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-2">
                                <i class="fa-solid fa-flask text-secondary"></i>
                                "Featured Projects"
                             </h3>
                            <div class="grid grid-cols-2 gap-3">
                                <ProjectCard
                                    title="Temidara.rocks"
                                    description="Personal portfolio built with Rust (Leptos) and TailwindCSS."
                                    link="https://github.com/temidaradev/temidara.rocks"
                                    link_text="View Source"
                                />
                                <ProjectCard
                                    title="mdif"
                                    description="My Disk Is Full - disk space analyzer."
                                    link="https://crates.io/crates/mdif"
                                    link_text="View Site"
                                />
                                <ProjectCard
                                    title="NeuralRust"
                                    description="A neural network implementation written in Rust, designed for learning and experimentation with machine learning concepts."
                                    link="https://github.com/temidaradev/NeuralRust"
                                    link_text="View Source"
                                />
                                <ProjectCard
                                    title="Nixos"
                                    description="My nixos config."
                                    link="https://github.com/temidaradev/nixos"
                                    link_text="View Source"
                                />
                                <ProjectCard
                                    title="pico2-riscv-badapple"
                                    description="Badapple but on risc-v activated pi pico 2"
                                    link="https://github.com/temidaradev/pico2-riscv-badapple"
                                    link_text="View Source"
                                />
                                <ProjectCard
                                    title="Deneyap1AV2-BadApple"
                                    description="Badapple but on made in Turkiye esp32-s3 based board (with sound)"
                                    link="https://github.com/temidaradev/DeneyapA1V2-BadApple"
                                    link_text="View Source"
                                />
                                <ProjectCard
                                    title="fastfetchus"
                                    description="A KDE Plasma (Plasma 6) widget that renders fastfetch output inside QML."
                                    link="https://github.com/temidaradev/fastfetchus"
                                    link_text="View Source"
                                />
                                <ProjectCard
                                    title="More Projects"
                                    description="Check out all my work on GitHub."
                                    link="https://github.com/temidaradev"
                                    link_text="GitHub"
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
