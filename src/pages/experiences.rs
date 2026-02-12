use leptos::prelude::*;

#[component]
pub fn ExperiencePage() -> impl IntoView {
    view! {
        <div class="space-y-12">
            <h1 class="text-xl font-bold tracking-tight text-white">"experiences"</h1>

            <div class="space-y-10">
                <section class="space-y-6">
                    <h2 class="text-xs font-bold uppercase tracking-widest text-white/50">"Work"</h2>
                    <div class="space-y-8">
                        <div class="group">
                            <div class="flex justify-between items-baseline mb-1">
                                <h3 class="font-bold text-white">"Course Tester / Reviewer"</h3>
                                <span class="text-xs font-mono text-gray-500">"Feb 2026 - Present"</span>
                            </div>
                            <div class="text-xs text-gray-500 mb-2">
                                "at " <a href="https://threedots.tech" target="_blank" class="hover:text-white hover:underline">"Three Dots Labs"</a>
                            </div>
                            <p class="text-gray-400 text-sm leading-relaxed mb-2">
                                "Early access alpha tester. Focusing on Domain-Driven Design and Go Backend patterns."
                            </p>
                             <div class="text-[10px] font-mono text-gray-600">
                                "[Go, DDD, Architecture]"
                            </div>
                        </div>
                         <div class="group">
                            <div class="flex justify-between items-baseline mb-1">
                                <h3 class="font-bold text-white">"Game Developer"</h3>
                                <span class="text-xs font-mono text-gray-500">"Sep 2024 - Present"</span>
                            </div>
                            <div class="text-xs text-gray-500 mb-2">
                                "at " <a href="https://github.com/jinchuugames" target="_blank" class="hover:text-white hover:underline">"jinchuugames"</a>
                            </div>
                            <p class="text-gray-400 text-sm leading-relaxed mb-2">
                                "Building interactive experiences. Transitioning from GDScript to Odin/Raylib."
                            </p>
                             <div class="text-[10px] font-mono text-gray-600">
                                "[Odin, Raylib, Godot]"
                            </div>
                        </div>

                         <div class="group">
                            <div class="flex justify-between items-baseline mb-1">
                                <h3 class="font-bold text-white">"Telegram Bot Dev"</h3>
                                <span class="text-xs font-mono text-gray-500">"Jul 2025 - Oct 2025"</span>
                            </div>
                             <div class="text-xs text-gray-500 mb-2">"Freelance"</div>
                            <p class="text-gray-400 text-sm leading-relaxed mb-2">
                                "Solana trading bot with copy trading and portfolio tracking."
                            </p>
                            <div class="text-[10px] font-mono text-gray-600">
                                "[Python, Solana]"
                            </div>
                        </div>
                    </div>
                </section>

                <section class="space-y-6">
                    <h2 class="text-xs font-bold uppercase tracking-widest text-white/50">"Growth"</h2>
                    <div class="space-y-8">
                         <div class="group">
                            <div class="flex justify-between items-baseline mb-1">
                                <h3 class="font-bold text-white">"Open Source"</h3>
                                <span class="text-xs font-mono text-gray-500">"May 2023 - Present"</span>
                            </div>
                            <p class="text-gray-400 text-sm leading-relaxed mb-2">
                                "Building tools like NeuraTalk and Drawniverse."
                            </p>
                             <div class="text-[10px] font-mono text-gray-600">
                                "[Rust, Go, TS]"
                            </div>
                        </div>

                         <div class="group">
                            <div class="flex justify-between items-baseline mb-1">
                                <h3 class="font-bold text-white">"Self-Taught"</h3>
                                <span class="text-xs font-mono text-gray-500">"2018 - Present"</span>
                            </div>
                            <p class="text-gray-400 text-sm leading-relaxed mb-2">
                                "Coding since 5th grade. Always building."
                            </p>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}
