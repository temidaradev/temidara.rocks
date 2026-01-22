use leptos::prelude::*;

use crate::components::*;

#[component]
fn ExperienceCard(
    #[prop(into)] icon: &'static str,
    #[prop(into)] title: &'static str,
    #[prop(into)] company: Option<&'static str>,
    #[prop(into)] company_link: Option<&'static str>,
    #[prop(into)] description: &'static str,
    #[prop(into)] date_range: &'static str,
    #[prop(into)] skills: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="glass rounded-2xl p-6 lg:p-8 border border-white/10 hover:border-primary/30 transition-all duration-300 group">
            <div class="flex items-start gap-4">
                <div class="text-2xl">{icon}</div>
                <div class="flex-1 space-y-4">
                    <div>
                        <h3 class="text-xl lg:text-2xl font-bold text-primary group-hover:text-gradient transition-all duration-300">
                            {title}
                        </h3>
                        {company.map(|c| {
                            view! {
                                <p class="text-foreground mt-1">
                                    "at "
                                    {company_link.map(|link| view! {
                                        <a href=link target="_blank" class="text-secondary hover:text-primary underline transition-colors">
                                            {c}
                                        </a>
                                    }.into_any()).unwrap_or_else(|| view! {
                                        <span class="text-secondary">{c}</span>
                                    }.into_any())}
                                </p>
                            }
                        })}
                    </div>
                    
                    <p class="text-foreground/80 leading-relaxed">
                        {description}
                    </p>
                    
                    <p class="text-muted text-sm font-mono">
                        {date_range}
                    </p>
                    
                    <div class="pt-2">
                        <p class="text-primary text-sm font-medium mb-3">"Skills"</p>
                        <div class="flex flex-wrap gap-2">
                            {skills.into_iter().map(|skill| view! {
                                <span class="px-3 py-1 text-xs font-medium bg-surface/80 border border-white/10 rounded-full text-foreground/80 hover:border-primary/30 hover:text-primary transition-all duration-200">
                                    {skill}
                                </span>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ExperiencePage() -> impl IntoView {
    view! {
        <div class="relative z-10 min-h-screen">
            <main class="max-w-4xl mx-auto px-6 py-32 lg:py-40">
                <div class="text-center mb-16 space-y-4">
                    <h1 class="text-5xl lg:text-6xl font-black tracking-tight text-gradient">
                        "Experience"
                    </h1>
                    <div class="w-32 h-1 mx-auto bg-gradient-to-r from-primary via-accent to-secondary rounded-full"></div>
                    <Text variant="dimmed" size="md">
                        "My journey through the tech world"
                    </Text>
                </div>

                <div class="space-y-12">
                    <section>
                        <h2 class="text-2xl font-bold text-foreground mb-6 flex items-center gap-3">
                            <i class="fa-solid fa-briefcase text-primary"></i>
                            "Work Experience"
                        </h2>
                        <div class="space-y-6">
                            <ExperienceCard
                                icon="🎮"
                                title="Game Developer"
                                company=Some("jinchuugames")
                                company_link=Some("https://github.com/jinchuugames")
                                description="Working on interactive games and experiences. Started with GDScript, now building with Odin and Raylib for better performance and control."
                                date_range="September 2024 - Present"
                                skills=vec!["Odin", "Raylib", "GDScript", "Game Development", "Godot Engine"]
                            />

                            <ExperienceCard
                                icon="🤖"
                                title="Telegram Bot Developer"
                                company=None
                                company_link=None
                                description="Built a Telegram bot for Solana trading with features like order placement, copy trading, portfolio tracking, and leaderboards."
                                date_range="July 2025 - October 2025"
                                skills=vec!["Python", "Telegram Bot API", "Solana Blockchain", "DeFi Integration"]
                            />
                        </div>
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-foreground mb-6 flex items-center gap-3">
                            <i class="fa-solid fa-seedling text-secondary"></i>
                            "Personal Growth"
                        </h2>
                        <div class="space-y-6">
                            <ExperienceCard
                                icon="📦"
                                title="Open Source Contributor"
                                company=None
                                company_link=None
                                description="Contributing to open source projects and building my own tools. Created NeuraTalk (AI library) and Drawniverse (creative platform) among other projects."
                                date_range="May 2023 - Present"
                                skills=vec!["Rust", "Go", "TypeScript", "Open Source", "API Development"]
                            />

                            <ExperienceCard
                                icon="📚"
                                title="Self-Taught Developer"
                                company=None
                                company_link=None
                                description="Started learning to code in 5th grade with C#. After a break to focus on school exams, picked up Go and continued expanding my skills. Always learning through building real projects."
                                date_range="2018 - Present"
                                skills=vec!["C#", "Golang", "C/C++", "Rust", "Self-learning"]
                            />
                        </div>
                    </section>
                </div>
            </main>
        </div>
    }
}