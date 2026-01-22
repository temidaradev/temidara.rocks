use leptos::prelude::*;

use crate::components::*;

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
        <div class="relative z-10 min-h-screen">
            <main class="max-w-2xl mx-auto px-6 py-32 lg:py-40">
                <div class="text-center mb-12 space-y-4">
                    <h1 class="text-5xl lg:text-6xl font-black tracking-tight text-gradient">
                        "Contact Form"
                    </h1>
                    <div class="w-32 h-1 mx-auto bg-gradient-to-r from-primary via-accent to-secondary rounded-full"></div>
                    <Text variant="default" size="lg" weight="bold" class="pt-4">
                        "Get in Touch!"
                    </Text>
                    <Text variant="dimmed" size="md">
                        "Have a project in mind or just want to say hi? Drop me a message!"
                    </Text>
                </div>

                <div class="glass rounded-2xl p-8 shadow-xl border border-white/10">
                    <form on:submit=on_submit class="space-y-6">
                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-foreground">
                                "Name"
                            </label>
                            <input
                                type="text"
                                required
                                placeholder="Your name..."
                                class="w-full px-4 py-3 bg-surface/80 border border-white/10 rounded-xl text-foreground placeholder-muted/50 focus:outline-none focus:border-primary/50 focus:ring-2 focus:ring-primary/20 transition-all duration-300"
                                prop:value=move || name.get()
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                            />
                        </div>

                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-foreground">
                                "Email"
                            </label>
                            <input
                                type="email"
                                required
                                placeholder="your@email.com"
                                class="w-full px-4 py-3 bg-surface/80 border border-white/10 rounded-xl text-foreground placeholder-muted/50 focus:outline-none focus:border-primary/50 focus:ring-2 focus:ring-primary/20 transition-all duration-300"
                                prop:value=move || email.get()
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                            />
                        </div>

                        <div class="space-y-2">
                            <label class="block text-sm font-medium text-foreground">
                                "Message"
                            </label>
                            <textarea
                                required
                                rows="5"
                                placeholder="Write your message here..."
                                class="w-full px-4 py-3 bg-surface/80 border border-white/10 rounded-xl text-foreground placeholder-muted/50 focus:outline-none focus:border-primary/50 focus:ring-2 focus:ring-primary/20 transition-all duration-300 resize-none"
                                prop:value=move || message.get()
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                            ></textarea>
                        </div>

                        <button
                            type="submit"
                            disabled=move || is_submitting.get()
                            class="w-full py-4 bg-gradient-to-r from-primary via-accent to-secondary text-background font-bold text-lg rounded-xl shadow-[0_0_30px_-5px_rgba(245,194,231,0.4)] hover:shadow-[0_0_40px_-5px_rgba(245,194,231,0.6)] hover:scale-[1.02] active:scale-[0.98] transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
                        >
                            {move || {
                                if is_submitting.get() {
                                    "Sending..."
                                } else {
                                    "Send Message"
                                }
                            }}
                        </button>

                        {move || {
                            submit_status.get().map(|success| {
                                if success {
                                    view! {
                                        <div class="text-center p-4 rounded-xl bg-green-500/10 border border-green-500/30 text-green-400">
                                            <i class="fa-solid fa-check-circle mr-2"></i>
                                            "Message sent successfully!"
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="text-center p-4 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400">
                                            <i class="fa-solid fa-times-circle mr-2"></i>
                                            "Failed to send. Please try again."
                                        </div>
                                    }
                                }
                            })
                        }}
                    </form>
                </div>

                <div class="flex justify-center gap-8 mt-12 text-2xl">
                    <a 
                        href="https://twitter.com/temidaradev" 
                        target="_blank"
                        class="text-foreground hover:text-primary transition-all duration-300 hover:scale-110 hover:-translate-y-1"
                    >
                        <i class="fa-brands fa-x-twitter"></i>
                    </a>
                    <a 
                        href="https://github.com/temidaradev" 
                        target="_blank"
                        class="text-foreground hover:text-primary transition-all duration-300 hover:scale-110 hover:-translate-y-1"
                    >
                        <i class="fa-brands fa-github"></i>
                    </a>
                </div>
            </main>
        </div>
    }
}