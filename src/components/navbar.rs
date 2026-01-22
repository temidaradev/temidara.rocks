use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[derive(Clone)]
struct Tab {
    name: &'static str,
    link: &'static str,
}

#[component]
pub fn NavBar() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let location = use_location();

    let tabs = [
        Tab {
            name: "Blog",
            link: "/blog",
        },
        Tab {
            name: "Experiences",
            link: "/experiences",
        },
        Tab {
            name: "Contact",
            link: "/contact",
        },
    ];

    let toggle_menu = move |_| set_menu_open.update(|n| *n = !*n);

    view! {
        <header class="fixed z-50 top-0 w-full glass border-none transition-all duration-300">
            <div class="max-w-7xl mx-auto px-6 h-16 flex justify-between items-center">
                <a href="/" class="group flex items-center gap-2">
                    <span class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-primary via-accent to-secondary hover:from-secondary hover:to-primary transition-all duration-500">
                        "temidaradev"
                    </span>
                    <span class="text-xs font-mono text-muted group-hover:text-primary transition-colors duration-300">
                        {move || format!("~{}", location.pathname.get())}
                    </span>
                </a>

                <button
                    class="lg:hidden flex items-center text-primary hover:text-secondary transition-colors duration-300"
                    on:click=toggle_menu
                    aria-label="Toggle menu"
                >
                    {move || if menu_open.get() {
                        view! { <i class="fa-solid fa-xmark text-xl"></i> }
                    } else {
                        view! { <i class="fa-solid fa-bars text-xl"></i> }
                    }}
                </button>

                <nav class="hidden lg:flex items-center gap-8">
                    {tabs.iter().map(|tab| {
                        view! {
                            <a
                                href=tab.link
                                class="text-sm font-medium text-muted hover:text-primary transition-colors duration-300 flex items-center gap-1 group"
                            >
                                <span class="text-accent/50 group-hover:text-accent opacity-0 group-hover:opacity-100 transition-all duration-300">
                                    "cd "
                                </span>
                                {tab.name}
                            </a>
                        }
                    }).collect::<Vec<_>>()}
                </nav>
            </div>

            {move || if menu_open.get() {
                view! {
                    <div class="lg:hidden absolute w-full bg-background/95 backdrop-blur-lg shadow-lg shadow-primary/10 animate-fade-in-down z-20">
                        <nav class="flex flex-col p-4">
                            {tabs.iter().map(|tab| {
                                view! {
                                    <a
                                        href=tab.link
                                        class="py-3 px-4 text-center text-muted hover:text-primary transition-colors duration-300 font-medium"
                                        on:click=move |_| set_menu_open.set(false)
                                    >
                                        {tab.name}
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </nav>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </header>
    }
}
