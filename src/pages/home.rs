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

    let music_resource = Resource::new(
        move || trigger.get(),
        |_| async move {
            crate::api::music::get_listenbrainz_current_track()
                .await
                .unwrap_or(None)
        },
    );

    let (thumb_url, set_thumb_url) = signal(None::<String>);
    let (current_track, set_current_track) = signal(None::<crate::api::music::TrackInfo>);
    Effect::new(move |_| {
        if let Some(Some(track)) = music_resource.get() {
            if current_track.get_untracked().as_ref() != Some(&track) {
                set_current_track.set(Some(track));
            }
        }
    });

    let cover_resource = Resource::new(
        move || {
            current_track
                .get()
                .map(|track| crate::api::music::CoverArtQuery {
                    title: track.title,
                    artist: track.artist,
                    album: track.album,
                    release_mbid: track.release_mbid,
                })
        },
        |query| async move {
            match query {
                Some(query) => crate::api::music::get_cover_art(query)
                    .await
                    .unwrap_or(None),
                None => None,
            }
        },
    );

    Effect::new(move |_| {
        if let Some(Some(url)) = cover_resource.get() {
            if thumb_url.get_untracked().as_ref() != Some(&url) {
                set_thumb_url.set(Some(url));
            }
        }
    });

    view! {
        <div class="space-y-12">
            <section class="home-intro">
                <div class="max-w-2xl">
                    <h1 class="text-3xl font-semibold tracking-tight text-white">"temidaradev"</h1>
                    <p class="mt-2 font-mono text-xs text-primary">"systems engineer & student"</p>
                    <p class="mt-6 text-base leading-7 text-gray-300">
                        "I build systems software and developer tools, mostly in Rust and Go. Lately I have been working on distributed backends, embedded systems, and emulating hardware I do not own."
                    </p>
                    <div class="mt-5 flex flex-wrap gap-x-4 gap-y-1 font-mono text-[10px] text-gray-600">
                        <span>"17"</span>
                        <span>"türkiye"</span>
                        <span>"rust / go / nix"</span>
                    </div>
                </div>

                <div class="now-playing grid grid-cols-[5rem_minmax(0,1fr)] items-center gap-4 sm:grid-cols-[6rem_minmax(0,1fr)] sm:gap-5">
                    <div class="relative flex-shrink-0">
                        {move || match thumb_url.get() {
                            Some(url) => view! {
                                <div class="relative h-20 w-20 overflow-hidden border border-white/10 bg-white/[0.03] sm:h-24 sm:w-24">
                                    <div class="absolute inset-0 flex items-center justify-center">
                                        <i class="fa-solid fa-music text-2xl text-gray-700"></i>
                                    </div>
                                    <img
                                        src=url
                                        alt=""
                                        loading="eager"
                                        fetchpriority="high"
                                        class="relative w-full h-full object-cover"
                                        on:error=|ev| {
                                            use leptos::wasm_bindgen::JsCast;
                                            if let Some(img) = ev.target().and_then(|t| t.dyn_into::<leptos::web_sys::HtmlElement>().ok()) {
                                                let _ = img.style().set_property("display", "none");
                                            }
                                        }
                                    />
                                </div>
                            }.into_any(),
                            None => view! {
                                <div class="flex h-20 w-20 items-center justify-center border border-white/10 bg-white/[0.03] sm:h-24 sm:w-24">
                                    <i class="fa-solid fa-music text-2xl text-gray-700"></i>
                                </div>
                            }.into_any()
                        }}
                    </div>

                    <div class="min-w-0">
                        {move || match current_track.get() {
                            Some(track) => {
                                let status_view = if track.status == "playing" {
                                    view! {
                                        <div class="flex items-center gap-2 font-mono text-green-400">
                                            <span class="text-[10px] uppercase tracking-wider">"listening now"</span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="flex items-center gap-2 font-mono text-gray-500">
                                            <span class="w-1.5 h-1.5 rounded-full bg-gray-500"></span>
                                            <span class="text-[10px] uppercase tracking-wider">"last listened"</span>
                                        </div>
                                    }.into_any()
                                };

                                view! {
                                    <div class="min-w-0">
                                        {status_view}
                                        <p class="mt-2 break-words text-base font-semibold leading-snug text-white sm:mt-3 sm:text-xl">
                                            {track.title}
                                        </p>
                                        <p class="mt-1 font-mono text-[11px] text-gray-500">
                                            {track.artist} <span class="text-gray-700">" / "</span> {track.album}
                                        </p>
                                    </div>
                                }.into_any()
                            },
                            _ => view! {
                                <div class="flex items-center gap-2 font-mono text-gray-600">
                                    <span class="w-1.5 h-1.5 rounded-full bg-gray-700"></span>
                                    <span class="text-[10px] uppercase tracking-wider">"music unavailable"</span>
                                </div>
                            }.into_any()
                        }}
                    </div>

                </div>
            </section>

            <section class="space-y-4">
                <h2 class="section-heading">"Projects"</h2>
                <div class="divide-y divide-white/[0.07] border-y border-white/[0.07]">
                    <a href="https://github.com/temidaradev/kopuz" target="_blank" rel="noopener noreferrer" class="project-row group block py-3">
                        <div class="flex flex-col items-start gap-1 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                            <span class="flex items-center gap-2 font-medium text-white group-hover:underline">
                                "Kopuz"
                                <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
                            </span>
                            <span class="font-mono text-[9px] text-gray-700">"rust / github ↗"</span>
                        </div>
                        <p class="mt-1 text-sm text-gray-500">"Music player, but written in Rust."</p>
                    </a>

                    <a href="https://crates.io/crates/mdif" target="_blank" rel="noopener noreferrer" class="project-row group block py-3">
                        <div class="flex flex-col items-start gap-1 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                            <span class="font-medium text-white group-hover:underline">"mdif"</span>
                            <span class="font-mono text-[9px] text-gray-700">"rust / crates.io ↗"</span>
                        </div>
                        <p class="mt-1 text-sm text-gray-500">"Terminal-based disk usage analyzer."</p>
                    </a>

                    <a href="https://github.com/temidaradev/NeuralRust" target="_blank" rel="noopener noreferrer" class="project-row group block py-3">
                        <div class="flex flex-col items-start gap-1 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                            <span class="font-medium text-white group-hover:underline">"neural-rust"</span>
                            <span class="font-mono text-[9px] text-gray-700">"rust / github ↗"</span>
                        </div>
                        <p class="mt-1 text-sm text-gray-500">"Neural network implementation from scratch."</p>
                    </a>

                    <a href="https://www.pling.com/p/2334389/" target="_blank" rel="noopener noreferrer" class="project-row group block py-3">
                        <div class="flex flex-col items-start gap-1 sm:flex-row sm:items-center sm:justify-between sm:gap-4">
                            <span class="font-medium text-white group-hover:underline">"fastfetchus"</span>
                            <span class="font-mono text-[9px] text-gray-700">"plasma / pling ↗"</span>
                        </div>
                        <p class="mt-1 text-sm text-gray-500">"KDE Plasma widget for fastfetch."</p>
                    </a>
                </div>
                <a href="/projects" class="inline-block font-mono text-[10px] text-gray-600 hover:text-white">"all projects ->"</a>
            </section>

            <crate::components::StatusSection />

            <section class="space-y-4">
                <h2 class="section-heading">"Socials"</h2>
                <div class="flex flex-wrap gap-4 text-sm font-mono text-gray-400">
                    <a href="mailto:temidaradev@temidara.rocks" class="hover:text-white hover:underline">"email"</a>
                    <a href="https://github.com/temidaradev" target="_blank" class="hover:text-white hover:underline">"github"</a>
                    <a href="https://x.com/temidaradev" target="_blank" class="hover:text-white hover:underline">"twitter"</a>
                </div>
            </section>

            <section class="space-y-4">
                <div>
                    <h2 class="section-heading">"Internet shelf"</h2>
                    <p class="mt-1 text-xs text-gray-700">"small pieces of the web I like"</p>
                </div>
                <div class="button-shelf flex flex-wrap gap-1">
                    <a href="https://rust-lang.org/" target="_blank" rel="noopener noreferrer">
                        <img src="https://cyber.dabamos.de/88x31/botao.gif" height="31" />
                    </a>
                    <a href="https://www.qbittorrent.org/" target="_blank" rel="noopener noreferrer">
                        <img src="https://meltingsnow.neocities.org/images/88x31piracy.gif" height="31" />
                    </a>
                    <a href="https://github.com/temidaradev" target="_blank" rel="noopener noreferrer">
                        <img src="https://cyber.dabamos.de/88x31/github.gif" height="31" />
                    </a>
                    <a href="https://88x31.nl/" target="_blank" rel="noopener noreferrer">
                        <img src="https://88x31.nl/gifs/nicesite.png" height="31" />
                    </a>
                    <a href="https://nixos.org/" target="_blank" rel="noopener noreferrer">
                        <img src="https://images.melonland.net/?url=https%3A%2F%2Fcrazyroostereye.de%2Fstuff%2Fnixos.png&w=1200&fit=inside&we&q=85&il&n=-1&default=1" height="31" />
                    </a>
                    <img src="https://2k2pea.ch/88x31/nixos.gif" height="31" />
                    <a href="https://duckduckgo.com/?q=hatsune+miku&ia=web" target="_blank" rel="noopener noreferrer">
                        <img src="https://chronocide.neocities.org/assets/88x31/miku.gif" height="31" />
                    </a>
                    <a href="https://duckduckgo.com/?q=kasane+teto&ia=web" target="_blank" rel="noopener noreferrer">
                        <img src="https://meltingsnow.neocities.org/images/tb_teto.gif" height="31" />
                    </a>
                    <a href="http://www.slsknet.org/" target="_blank" rel="noopener noreferrer">
                        <img src="https://cyber.dabamos.de/88x31/soulseek.gif" height="31" />
                    </a>
                    <img src="https://88x31.nl/gifs/168.gif" height="31" />
                    <img src="https://88x31.nl/gifs/blends.gif" height="31" />
                    <img src="https://88x31.nl/gifs/caramelldansen.gif" height="31" />
                    <img src="https://88x31.nl/gifs/ralseismokingadart.gif" height="31" />
                    <img src="https://88x31.nl/gifs/verine.gif" height="31" />
                    <img src="https://capstasher.neocities.org/88x31Buttons/dramaturgie.gif" height="31" />
                    <img src="https://capstasher.neocities.org/88x31Buttons/hekate.gif" height="31" />
                    <img src="https://capstasher.neocities.org/88x31Buttons/j04q1x.png" height="31" />
                    <img src="https://capstasher.neocities.org/88x31Buttons/nyaabanner.gif" height="31" />
                    <img src="https://capstasher.neocities.org/88x31Buttons/vocaloid.gif" height="31" />
                </div>
            </section>
        </div>
    }
}
