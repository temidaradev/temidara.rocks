use leptos::html::Input;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::prelude::*;
use leptos::{ev, wasm_bindgen::JsCast};

#[derive(Clone)]
struct CommandItem {
    title: String,
    description: String,
    href: String,
    kind: &'static str,
}

fn command_items() -> Vec<CommandItem> {
    let mut items = vec![
        CommandItem {
            title: "Index".to_string(),
            description: "Home and current activity".to_string(),
            href: "/".to_string(),
            kind: "page",
        },
        CommandItem {
            title: "Blog".to_string(),
            description: "Technical writing".to_string(),
            href: "/blog".to_string(),
            kind: "page",
        },
        CommandItem {
            title: "Projects".to_string(),
            description: "Things I have built".to_string(),
            href: "/projects".to_string(),
            kind: "page",
        },
        CommandItem {
            title: "Experiences".to_string(),
            description: "Work and project history".to_string(),
            href: "/experiences".to_string(),
            kind: "page",
        },
        CommandItem {
            title: "Uses".to_string(),
            description: "Systems and development setup".to_string(),
            href: "/uses".to_string(),
            kind: "page",
        },
        CommandItem {
            title: "Guestbook".to_string(),
            description: "Leave a message".to_string(),
            href: "/guestbook".to_string(),
            kind: "page",
        },
        CommandItem {
            title: "Contact".to_string(),
            description: "Email and social links".to_string(),
            href: "/contact".to_string(),
            kind: "page",
        },
    ];

    items.extend(
        crate::pages::projects::get_projects()
            .into_iter()
            .map(|project| CommandItem {
                title: project.title,
                description: project.description,
                href: format!("/projects/{}", project.slug),
                kind: "project",
            }),
    );
    items.extend(
        crate::pages::blog::get_blog_posts()
            .into_iter()
            .map(|post| CommandItem {
                title: post.title,
                description: post.description,
                href: format!("/blog/{}", post.slug),
                kind: "post",
            }),
    );
    items
}

fn filtered_items(items: &[CommandItem], query: &str) -> Vec<CommandItem> {
    let query = query.trim().to_lowercase();
    items
        .iter()
        .filter(|item| {
            query.is_empty()
                || item.title.to_lowercase().contains(&query)
                || item.description.to_lowercase().contains(&query)
                || item.kind.contains(&query)
        })
        .take(10)
        .cloned()
        .collect()
}

fn is_typing_target(event: &leptos::web_sys::KeyboardEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<leptos::web_sys::Element>().ok())
        .is_some_and(|element| {
            matches!(element.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
                || element.get_attribute("contenteditable").as_deref() == Some("true")
        })
}

#[component]
pub fn CommandPalette() -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().expect("command palette context");
    let (query, set_query) = signal(String::new());
    let (selected, set_selected) = signal(0_usize);
    let input_ref = NodeRef::<Input>::new();
    let items = StoredValue::new(command_items());

    Effect::new(move |_| {
        let handle = window_event_listener(ev::keydown, move |event| {
            let key = event.key();
            let command_shortcut =
                (event.meta_key() || event.ctrl_key()) && key.eq_ignore_ascii_case("k");
            let slash_shortcut = key == "/" && !is_typing_target(&event);

            if command_shortcut || slash_shortcut {
                event.prevent_default();
                set_query.set(String::new());
                set_selected.set(0);
                open.set(true);
            } else if key == "Escape" && open.get_untracked() {
                open.set(false);
            }
        });
        on_cleanup(move || handle.remove());
    });

    Effect::new(move |_| {
        if open.get() {
            set_selected.set(0);
            request_animation_frame(move || {
                if let Some(input) = input_ref.get() {
                    let _ = input.focus();
                }
            });
        }
    });

    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 z-50 flex justify-center p-3 sm:px-4 sm:pt-[14vh]">
                <button
                    type="button"
                    aria-label="Close search"
                    class="absolute inset-0 bg-black/85"
                    on:click=move |_| open.set(false)
                ></button>

                <div role="dialog" aria-modal="true" aria-label="Site search" class="relative flex h-fit max-h-[calc(100dvh-1.5rem)] w-full max-w-lg flex-col overflow-hidden border border-white/15 bg-surface font-mono">
                    <div class="flex items-center gap-3 border-b border-white/10 px-3">
                        <span class="text-xs text-gray-600">"/"</span>
                        <input
                            node_ref=input_ref
                            type="search"
                            placeholder="search pages, projects, posts..."
                            aria-label="Search pages, projects, and posts"
                            aria-controls="command-results"
                            class="h-12 min-w-0 flex-1 bg-transparent text-base text-gray-200 outline-none placeholder:text-gray-700 sm:text-xs"
                            prop:value=move || query.get()
                            on:input=move |event| {
                                set_query.set(event_target_value(&event));
                                set_selected.set(0);
                            }
                            on:keydown=move |event| {
                                let results = filtered_items(&items.get_value(), &query.get_untracked());
                                match event.key().as_str() {
                                    "ArrowDown" => {
                                        event.prevent_default();
                                        if !results.is_empty() {
                                            set_selected.update(|index| {
                                                *index = (*index + 1).min(results.len() - 1);
                                            });
                                        }
                                    }
                                    "ArrowUp" => {
                                        event.prevent_default();
                                        set_selected.update(|index| *index = index.saturating_sub(1));
                                    }
                                    "Enter" => if let Some(item) = results.get(selected.get_untracked()) {
                                        event.prevent_default();
                                        open.set(false);
                                        if let Some(window) = leptos::web_sys::window() {
                                            let _ = window.location().set_href(&item.href);
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        />
                        <button
                            type="button"
                            class="border border-white/10 px-1.5 py-0.5 text-[9px] text-gray-700 hover:border-white/20 hover:text-gray-400"
                            on:click=move |_| open.set(false)
                        >
                            "esc"
                        </button>
                    </div>

                    <div id="command-results" role="listbox" class="min-h-0 flex-1 overflow-y-auto py-1 sm:max-h-[52vh]">
                        {move || {
                            let results = filtered_items(&items.get_value(), &query.get());
                            if results.is_empty() {
                                view! {
                                    <div class="px-4 py-10 text-center">
                                        <p class="text-xs text-gray-500">"nothing found"</p>
                                        <p class="mt-1 text-[10px] text-gray-700">"try a page, project, or post title"</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div>
                                        {results.into_iter().enumerate().map(|(index, item)| view! {
                                            <a
                                                href=item.href
                                                role="option"
                                                aria-selected=move || selected.get() == index
                                                class=move || if selected.get() == index {
                                                    "group flex items-center justify-between gap-4 border-l border-primary bg-primary/[0.05] px-3 py-2.5"
                                                } else {
                                                    "group flex items-center justify-between gap-4 border-l border-transparent px-3 py-2.5 hover:bg-white/[0.03]"
                                                }
                                                on:mouseenter=move |_| set_selected.set(index)
                                                on:click=move |_| open.set(false)
                                            >
                                                <span class="min-w-0">
                                                    <span class="block truncate text-xs text-gray-300 group-hover:text-white">{item.title}</span>
                                                    <span class="mt-0.5 block truncate text-[10px] text-gray-600">{item.description}</span>
                                                </span>
                                                <span class="shrink-0 text-[9px] uppercase tracking-wider text-gray-700">
                                                    {format!("[{}]", item.kind)}
                                                </span>
                                            </a>
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>

                    <div class="hidden gap-4 border-t border-white/10 px-3 py-2 text-[9px] text-gray-700 sm:flex">
                        <span>"↑↓ select"</span>
                        <span>"enter open"</span>
                        <span>"esc close"</span>
                    </div>
                </div>
            </div>
        </Show>
    }
}
