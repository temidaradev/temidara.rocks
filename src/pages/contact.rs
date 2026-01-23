use leptos::prelude::*;



#[component]
pub fn ContactPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (submit_status, set_submit_status) = signal(Option::<bool>::None);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_is_submitting.set(true);
        set_submit_status.set(None);

        let n = name.get();
        let e = email.get();
        let m = message.get();
        
        leptos::task::spawn_local(async move {
            match crate::api::contact::send_contact_email(n, e, m).await {
                Ok(_) => {
                    set_is_submitting.set(false);
                    set_submit_status.set(Some(true));
                    set_name.set(String::new());
                    set_email.set(String::new());
                    set_message.set(String::new());
                },
                Err(err) => {
                    leptos::logging::error!("Failed to send email: {}", err);
                    set_is_submitting.set(false);
                    set_submit_status.set(Some(false));
                }
            }
        });
    };

    view! {
        <div class="space-y-8">
            <h1 class="text-xl font-bold tracking-tight text-white">"contact"</h1>

            <form on:submit=on_submit class="max-w-md space-y-4">
                <div class="grid sm:grid-cols-2 gap-4">
                    <div class="space-y-1">
                        <label class="block text-xs font-bold text-gray-500 uppercase">"name"</label>
                        <input
                            type="text"
                            required
                            class="w-full bg-[#111] border border-white/10 p-2 text-sm text-white focus:border-white/30 focus:outline-none transition-colors"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="space-y-1">
                        <label class="block text-xs font-bold text-gray-500 uppercase">"email"</label>
                        <input
                            type="email"
                            required
                            class="w-full bg-[#111] border border-white/10 p-2 text-sm text-white focus:border-white/30 focus:outline-none transition-colors"
                            prop:value=move || email.get()
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="space-y-1">
                    <label class="block text-xs font-bold text-gray-500 uppercase">"message"</label>
                    <textarea
                        required
                        rows="4"
                        class="w-full bg-[#111] border border-white/10 p-2 text-sm text-white focus:border-white/30 focus:outline-none transition-colors resize-none"
                        prop:value=move || message.get()
                        on:input=move |ev| set_message.set(event_target_value(&ev))
                    ></textarea>
                </div>

                <button
                    type="submit"
                    disabled=move || is_submitting.get()
                    class="px-6 py-2 bg-white text-black text-xs font-bold uppercase tracking-wider hover:bg-gray-200 transition-colors disabled:opacity-50"
                >
                    {move || if is_submitting.get() { "sending..." } else { "send" }}
                </button>

                {move || {
                    submit_status.get().map(|success| {
                        if success {
                            view! {
                                <div class="text-green-500 font-mono text-sm">
                                    "-> message sent successfully."
                                </div>
                            }
                        } else {
                            view! {
                                <div class="text-red-500 font-mono text-sm">
                                    "-> failed to send message."
                                </div>
                            }
                        }
                    })
                }}
            </form>
            
            <div class="pt-8 border-t border-white/10">
                <h2 class="text-sm font-bold text-white uppercase tracking-widest mb-4">"socials"</h2>
                <ul class="space-y-2 text-sm text-gray-400">
                    <li><a href="https://x.com/temidaradev" target="_blank" class="hover:text-white hover:underline">"twitter / x"</a></li>
                    <li><a href="https://github.com/temidaradev" target="_blank" class="hover:text-white hover:underline">"github"</a></li>
                </ul>
            </div>
        </div>
    }
}