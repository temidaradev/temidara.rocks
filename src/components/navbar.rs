use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <header class="mb-12">
            <nav class="flex gap-4 text-xs font-bold uppercase tracking-wider">
                <a href="/" class="hover:text-white text-gray-500 transition-colors">"Index"</a>
                <a href="/blog" class="hover:text-white text-gray-500 transition-colors">"Blog"</a>
                <a href="/experiences" class="hover:text-white text-gray-500 transition-colors">"Exp"</a>
                <a href="/contact" class="hover:text-white text-gray-500 transition-colors">"Contact"</a>
            </nav>
        </header>
    }
}
