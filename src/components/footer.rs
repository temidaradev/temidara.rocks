use leptos::prelude::*;

use crate::components::button::Button;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full py-10 flex flex-col items-center justify-center relative z-10">

            <div class="relative group">
                <a href="mailto:temidaradev@temidara.rocks">
                    <Button text="Contact Me !" variant="Primary" />
                </a>
            </div>

            <div class="mt-8 p-8 rounded-2xl bg-surface/80 backdrop-blur-md border border-white/5 shadow-xl flex flex-col items-center gap-6 max-w-sm w-full mx-4">
                <div class="flex gap-8 text-2xl">
                    <a href="https://github.com/temidaradev" target="_blank" class="text-foreground hover:text-primary transition-colors duration-300">
                        <i class="fa-brands fa-github"></i>
                    </a>
                    <a href="mailto:temidaradev@temidara.rocks" class="text-foreground hover:text-primary transition-colors duration-300">
                        <i class="fa-solid fa-envelope"></i>
                    </a>
                </div>

                <div class="flex flex-col gap-3 text-sm w-full">
                    <div class="flex items-center justify-between">
                        <span class="text-muted">"Email:"</span>
                        <a href="mailto:temidaradev@temidara.rocks" class="text-primary font-medium hover:underline">
                            "temidaradev@temidara.rocks"
                        </a>
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-muted">"GitHub:"</span>
                        <a href="https://github.com/temidaradev" target="_blank" class="text-primary font-medium hover:underline">
                            "@temidaradev"
                             <i class="fa-solid fa-arrow-up-right-from-square text-xs ml-1"></i>
                        </a>
                    </div>
                </div>
            </div>
            <p class="text-muted text-xs mt-4">"this website is written in rust :3"</p>
        </footer>
    }
}
