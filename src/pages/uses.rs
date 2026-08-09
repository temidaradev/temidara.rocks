use leptos::prelude::*;

struct UsesGroup {
    title: &'static str,
    items: &'static [(&'static str, &'static str)],
}

const USES: &[UsesGroup] = &[
    UsesGroup {
        title: "systems",
        items: &[
            ("NixOS", "Reproducible Linux workstation configuration."),
            ("Fedora", "Virtual machines and cross-architecture testing."),
            ("macOS", "Daily host for development."),
        ],
    },
    UsesGroup {
        title: "development",
        items: &[
            ("Rust", "Primary language and favorite for the job."),
            ("Go", "For the job when simplicity is the better fit."),
            (
                "Nix",
                "Machines, development environments, and configuration.",
            ),
        ],
    },
];

#[component]
pub fn UsesPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <div class="space-y-2">
                <h1 class="page-title">"uses"</h1>
                <p class="text-sm text-gray-500">"Systems and languages I currently enjoy using."</p>
            </div>

            {USES.iter().map(|group| view! {
                <section class="space-y-4">
                    <h2 class="section-heading">
                        {group.title}
                    </h2>
                    <div class="grid gap-4 sm:grid-cols-2">
                        {group.items.iter().map(|(name, description)| view! {
                            <div>
                                <h3 class="text-sm font-medium text-gray-200">{*name}</h3>
                                <p class="mt-1 text-xs leading-relaxed text-gray-500">{*description}</p>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                </section>
            }).collect::<Vec<_>>()}
        </div>
    }
}
