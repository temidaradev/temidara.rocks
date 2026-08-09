use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavBar() -> impl IntoView {
    let command_palette_open = use_context::<RwSignal<bool>>().expect("command palette context");

    view! {
        <header class="site-header mb-8">
            <a href="/" class="flex items-center gap-2 font-mono text-xs text-gray-400 hover:text-white">
                <img src="/nixos-logo.png" alt="" class="h-5 w-5" />
                <span>"temidaradev"</span>
            </a>

            <nav class="site-nav flex gap-x-5 text-[11px] font-bold uppercase tracking-wider text-gray-600">
                <A href="/" exact=true attr:class="hover:text-white">"Index"</A>
                <A href="/blog" attr:class="hover:text-white">"Blog"</A>
                <A href="/projects" attr:class="hover:text-white">"Projects"</A>
                <A href="/experiences" attr:class="hover:text-white">"Exp"</A>
                <A href="/uses" attr:class="hover:text-white">"Uses"</A>
                <A href="/guestbook" attr:class="hover:text-white">"Guestbook"</A>
                <A href="/contact" attr:class="hover:text-white">"Contact"</A>
            </nav>

            <button
                type="button"
                class="font-mono text-[10px] uppercase tracking-wider text-gray-600 hover:text-white"
                on:click=move |_| command_palette_open.set(true)
            >
                "Search /"
            </button>
        </header>
    }
}
