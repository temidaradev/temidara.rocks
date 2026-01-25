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
    let thumb_url = move || {
        plex_resource
            .get()
            .flatten()
            .and_then(|track| track.thumb_url)
    };
    view! {
        <div class="space-y-8">
            <section class="space-y-4">
                <h1 class="text-xl font-bold tracking-tight text-white">"temidaradev"</h1>
                <div class="text-sm space-y-1 text-gray-400 font-mono">
                    <p>"-> systems developer & student"</p>
                    <p>"-> 17 years old"</p>
                    <p>"-> located in türkiye"</p>
                </div>

                <p class="max-w-xl text-sm leading-relaxed text-gray-300">
                    "I enjoy building high-performance systems and tools. Currently exploring embedded development and OS design. Writing Rust, Go, and TypeScript."
                </p>

                <Transition fallback=move || view! {
                    <div class="flex items-center gap-4 pt-3 border-t border-white/10">
                        <div class="relative flex-shrink-0">
                            <div class="w-16 h-16 rounded-lg bg-[#e5a00d]/10 flex items-center justify-center border border-[#e5a00d]/20">
                                <i class="fa-solid fa-music text-[#e5a00d]/50 text-xl"></i>
                            </div>
                        </div>
                        <div class="text-[10px] uppercase tracking-wider font-mono text-gray-600">
                            "// media server offline"
                        </div>
                    </div>
                }>
                    {move || {
                        let art_view = move || match thumb_url() {
                            Some(url) => view! {
                                <div class="w-16 h-16 rounded-lg overflow-hidden shadow-lg group-hover:shadow-[#e5a00d]/20 transition-shadow duration-300">
                                    <img
                                        src=url
                                        alt="Album Art"
                                        class="w-full h-full object-cover"
                                    />
                                </div>
                            }.into_any(),
                            None => view! {
                                <div class="w-16 h-16 rounded-lg bg-[#e5a00d]/10 flex items-center justify-center border border-[#e5a00d]/20">
                                    <i class="fa-solid fa-music text-[#e5a00d]/50 text-xl"></i>
                                </div>
                            }.into_any()
                        };

                        match plex_resource.get() {
                            Some(Some(track)) => {
                                let status_view = if track.status == "playing" {
                                    view! {
                                        <div class="flex items-center gap-2 text-green-400">
                                            <span class="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse"></span>
                                            <span class="text-[10px] uppercase tracking-wider">"listening now"</span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="flex items-center gap-2 text-yellow-500">
                                            <span class="w-1.5 h-1.5 rounded-full bg-yellow-500"></span>
                                            <span class="text-[10px] uppercase tracking-wider">"paused"</span>
                                        </div>
                                    }.into_any()
                                };

                                view! {
                                    <div class="flex items-center gap-4 pt-3 border-t border-white/10">
                                        <div class="relative flex-shrink-0">
                                            {art_view}
                                        </div>
                                        <div class="text-xs font-mono">
                                            {status_view}
                                            <div class="mt-1 text-white truncate max-w-sm">
                                                {track.artist} " - " {track.title}
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            },
                            _ => view! {
                                <div class="flex items-center gap-4 pt-3 border-t border-white/10">
                                    <div class="relative flex-shrink-0">
                                        {art_view}
                                    </div>
                                    <div class="text-[10px] uppercase tracking-wider font-mono text-gray-600">
                                        "// music paused"
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </Transition>
            </section>

            <section class="space-y-4">
                <h2 class="text-xs font-bold uppercase tracking-widest text-white/50">"Projects"</h2>
                <div class="grid gap-4">
                    <div class="group">
                        <a href="https://crates.io/crates/mdif" class="text-white hover:underline font-medium">"mdif"</a>
                        <p class="text-gray-400 text-sm">"Terminal-based disk usage analyzer."</p>
                        <p class="text-gray-500 text-xs">"-> crates.io"</p>
                    </div>

                    <div class="group">
                        <a href="https://github.com/temidaradev/NeuralRust" class="text-white hover:underline font-medium">"neural-rust"</a>
                        <p class="text-gray-400 text-sm">"Neural network implementation from scratch."</p>
                        <p class="text-gray-500 text-xs">"-> github.com"</p>
                    </div>

                    <div class="group">
                        <a href="https://www.pling.com/p/2334389/" class="text-white hover:underline font-medium">"fastfetchus"</a>
                        <p class="text-gray-400 text-sm">"KDE Plasma widget for fastfetch."</p>
                        <p class="text-gray-500 text-xs">"-> pling.com"</p>
                    </div>

                    <div class="group">
                        <a href="https://github.com/temidaradev/nixos" class="text-white hover:underline font-medium">"dotfiles"</a>
                        <p class="text-gray-400 text-sm">"NixOS configuration and system setup."</p>
                        <p class="text-gray-500 text-xs">"-> github.com"</p>
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xs font-bold uppercase tracking-widest text-white/50">"Activity"</h2>
                <div class="group relative w-fit overflow-hidden rounded-lg bg-black/20 transition-colors">
                    <img 
                        src="https://github-readme-stats.hackclub.dev/api/wakatime?username=12057&api_domain=hackatime.hackclub.com&theme=transparent&custom_title=Hackatime+Stats&layout=compact&cache_seconds=0&langs_count=8&hide_border=true" 
                        alt="Wakatime Stats"
                        class="h-32 w-auto opacity-80 group-hover:opacity-100 transition-opacity"
                    />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xs font-bold uppercase tracking-widest text-white/50">"Socials"</h2>
                <div class="flex gap-4 text-sm font-mono text-gray-400">
                    <a href="mailto:temidaradev@temidara.rocks" class="hover:text-white hover:underline">"email"</a>
                    <a href="https://github.com/temidaradev" target="_blank" class="hover:text-white hover:underline">"github"</a>
                    <a href="https://x.com/temidaradev" target="_blank" class="hover:text-white hover:underline">"twitter"</a>
                </div>
            </section>
        </div>
    }
}
