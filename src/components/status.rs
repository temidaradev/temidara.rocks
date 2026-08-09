use leptos::prelude::*;

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

#[component]
pub fn StatusSection() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);
    let server_status = Resource::new(
        move || refresh.get(),
        |_| async { crate::api::status::get_server_status().await.ok() },
    );

    Effect::new(move |_| {
        let handle = set_interval_with_handle(
            move || set_refresh.update(|value| *value += 1),
            std::time::Duration::from_secs(60),
        )
        .ok();
        on_cleanup(move || {
            if let Some(handle) = handle {
                handle.clear();
            }
        });
    });

    view! {
        <section id="status" class="space-y-4">
            <h2 class="section-heading">"Status"</h2>
            <div class="status-grid grid gap-4 text-xs font-mono sm:grid-cols-3">
                <div>
                    <p class="text-[10px] uppercase tracking-wider text-gray-600">"focus"</p>
                    <a href="https://github.com/temidaradev/kopuz" class="mt-1 block text-gray-300 hover:text-white hover:underline">
                        "Kopuz"
                    </a>
                </div>
                <Transition fallback=move || view! {
                    <div>
                        <p class="text-[10px] uppercase tracking-wider text-gray-600">"server"</p>
                        <p class="mt-1 text-gray-700">"checking..."</p>
                    </div>
                }>
                    {move || server_status.get().flatten().map(|status| view! {
                        <div>
                            <p class="text-[10px] uppercase tracking-wider text-gray-600">"server"</p>
                            <p class="mt-1 text-gray-300">
                                {format!("{} / {}", status.operating_system, status.architecture)}
                            </p>
                        </div>
                        <div>
                            <p class="text-[10px] uppercase tracking-wider text-gray-600">"uptime"</p>
                            <p class="mt-1 text-gray-300">
                                {status.uptime_seconds.map(format_uptime).unwrap_or_else(|| "unavailable".to_string())}
                            </p>
                        </div>
                    })}
                </Transition>
            </div>
        </section>
    }
}
