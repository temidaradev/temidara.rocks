use crate::api::guestbook::GuestbookEntry;
use leptos::prelude::*;

#[component]
pub fn GuestbookPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (website, set_website) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (notice, set_notice) = signal(None::<(bool, String)>);
    let entries = Resource::new(
        || (),
        |_| async {
            crate::api::guestbook::list_guestbook_entries()
                .await
                .unwrap_or_default()
        },
    );

    let submit = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        if submitting.get_untracked() {
            return;
        }
        set_submitting.set(true);
        set_notice.set(None);
        let submitted_name = name.get_untracked();
        let submitted_message = message.get_untracked();
        let submitted_website = website.get_untracked();

        leptos::task::spawn_local(async move {
            match crate::api::guestbook::submit_guestbook_entry(
                submitted_name,
                submitted_message,
                submitted_website,
            )
            .await
            {
                Ok(result) => {
                    set_name.set(String::new());
                    set_message.set(String::new());
                    set_website.set(String::new());
                    let text = if result.pending {
                        "Message received and waiting for moderation."
                    } else {
                        entries.refetch();
                        "Message published."
                    };
                    set_notice.set(Some((true, text.to_string())));
                }
                Err(error) => set_notice.set(Some((false, error.to_string()))),
            }
            set_submitting.set(false);
        });
    };

    view! {
        <div class="space-y-10">
            <div class="space-y-2">
                <h1 class="page-title">"guestbook"</h1>
                <p class="text-sm text-gray-500">"Leave a short message. New entries may be reviewed before appearing."</p>
            </div>

            <form on:submit=submit class="space-y-4 border-b border-white/10 pb-10">
                <div>
                    <label for="guestbook-name" class="mb-2 block text-xs font-mono uppercase tracking-wider text-gray-600">"name"</label>
                    <input
                        id="guestbook-name"
                        type="text"
                        maxlength="40"
                        required
                        prop:value=move || name.get()
                        on:input=move |event| set_name.set(event_target_value(&event))
                        class="w-full border border-white/10 bg-white/[0.03] px-3 py-2.5 text-sm text-white outline-none placeholder:text-gray-700 focus:border-primary/40"
                        placeholder="Your name"
                    />
                </div>
                <div>
                    <label for="guestbook-message" class="mb-2 block text-xs font-mono uppercase tracking-wider text-gray-600">"message"</label>
                    <textarea
                        id="guestbook-message"
                        maxlength="400"
                        required
                        rows="4"
                        prop:value=move || message.get()
                        on:input=move |event| set_message.set(event_target_value(&event))
                        class="w-full resize-y border border-white/10 bg-white/[0.03] px-3 py-2.5 text-sm leading-relaxed text-white outline-none placeholder:text-gray-700 focus:border-primary/40"
                        placeholder="Say hello..."
                    ></textarea>
                    <p class="mt-1 text-right text-[10px] font-mono text-gray-700">
                        {move || format!("{}/400", message.get().chars().count())}
                    </p>
                </div>
                <div class="absolute -left-[10000px]" aria-hidden="true">
                    <label for="guestbook-website">"Website"</label>
                    <input
                        id="guestbook-website"
                        type="text"
                        tabindex="-1"
                        autocomplete="off"
                        prop:value=move || website.get()
                        on:input=move |event| set_website.set(event_target_value(&event))
                    />
                </div>
                <div class="flex items-center justify-between gap-4">
                    <button
                        type="submit"
                        disabled=move || submitting.get()
                        class="border border-white/10 bg-white/5 px-4 py-2 text-xs font-medium text-gray-200 hover:border-primary/30 hover:text-white disabled:cursor-wait disabled:opacity-50"
                    >
                        {move || if submitting.get() { "sending..." } else { "sign guestbook" }}
                    </button>
                    {move || notice.get().map(|(success, text)| view! {
                        <p class=if success { "text-xs text-green-500" } else { "text-xs text-red-400" }>{text}</p>
                    })}
                </div>
            </form>

            <section class="space-y-6">
                <h2 class="section-heading">"messages"</h2>
                <Suspense fallback=move || view! {
                    <p class="text-xs font-mono text-gray-700">"loading messages..."</p>
                }>
                    {move || {
                        let messages = entries.get().unwrap_or_default();
                        if messages.is_empty() {
                            view! {
                                <p class="text-sm text-gray-600">"No messages yet."</p>
                            }.into_any()
                        } else {
                            view! {
                                <div class="grid gap-6">
                                    {messages.into_iter().map(|entry| view! {
                                        <article>
                                            <p class="text-xs font-mono text-gray-500">{entry.name}</p>
                                            <p class="mt-2 whitespace-pre-wrap text-sm leading-relaxed text-gray-300">{entry.message}</p>
                                        </article>
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        }
                    }}
                </Suspense>
            </section>

            <a href="/guestbook/moderate" class="block text-right text-[10px] font-mono text-gray-800 hover:text-gray-500">"moderate"</a>
        </div>
    }
}

#[component]
pub fn GuestbookModerationPage() -> impl IntoView {
    let (token, set_token) = signal(String::new());
    let (entries, set_entries) = signal(Vec::<GuestbookEntry>::new());
    let (status, set_status) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let load = move || {
        set_loading.set(true);
        set_status.set(None);
        let submitted_token = token.get_untracked();
        leptos::task::spawn_local(async move {
            match crate::api::guestbook::list_pending_guestbook_entries(submitted_token).await {
                Ok(pending) => set_entries.set(pending),
                Err(error) => set_status.set(Some(error.to_string())),
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="space-y-8">
            <div class="space-y-2">
                <h1 class="page-title">"guestbook moderation"</h1>
                <p class="text-sm text-gray-500">"Review pending messages."</p>
            </div>

            <div class="flex gap-2">
                <input
                    type="password"
                    placeholder="Admin token"
                    prop:value=move || token.get()
                    on:input=move |event| set_token.set(event_target_value(&event))
                    on:keydown=move |event| {
                        if event.key() == "Enter" {
                            load();
                        }
                    }
                    class="min-w-0 flex-1 border border-white/10 bg-white/[0.03] px-3 py-2.5 text-sm text-white outline-none focus:border-primary/40"
                />
                <button
                    type="button"
                    on:click=move |_| load()
                    disabled=move || loading.get()
                    class="border border-white/10 bg-white/5 px-4 py-2 text-xs text-gray-300 hover:border-primary/30 disabled:opacity-50"
                >
                    {move || if loading.get() { "loading..." } else { "load" }}
                </button>
            </div>

            {move || status.get().map(|message| view! {
                <p class="text-xs text-red-400">{message}</p>
            })}

            <div class="grid gap-6">
                {move || entries.get().into_iter().map(|entry| {
                    let id = entry.id;
                    let approve = move |_| {
                        let submitted_token = token.get_untracked();
                        leptos::task::spawn_local(async move {
                            match crate::api::guestbook::moderate_guestbook_entry(
                                submitted_token,
                                id,
                                "approve".to_string(),
                            ).await {
                                Ok(()) => set_entries.update(|items| items.retain(|item| item.id != id)),
                                Err(error) => set_status.set(Some(error.to_string())),
                            }
                        });
                    };
                    let delete = move |_| {
                        let submitted_token = token.get_untracked();
                        leptos::task::spawn_local(async move {
                            match crate::api::guestbook::moderate_guestbook_entry(
                                submitted_token,
                                id,
                                "delete".to_string(),
                            ).await {
                                Ok(()) => set_entries.update(|items| items.retain(|item| item.id != id)),
                                Err(error) => set_status.set(Some(error.to_string())),
                            }
                        });
                    };

                    view! {
                        <article class="border border-white/10 p-4">
                            <p class="text-xs font-mono text-gray-500">{entry.name}</p>
                            <p class="mt-2 whitespace-pre-wrap text-sm leading-relaxed text-gray-300">{entry.message}</p>
                            <div class="mt-4 flex gap-2">
                                <button type="button" on:click=approve class="text-xs text-green-500 hover:text-green-400">"approve"</button>
                                <button type="button" on:click=delete class="text-xs text-red-500 hover:text-red-400">"delete"</button>
                            </div>
                        </article>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
