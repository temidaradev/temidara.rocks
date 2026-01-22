use crate::components::{text::Text, cards::Card};
use crate::api::blog::get_repo_readme;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub description: String,
    pub slug: String,
    pub readme_url: String,
}

pub fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            title: "mdif".to_string(),
            date: "2026-01-21".to_string(),
            description: "A terminal-based disk usage analyzer written in Rust. It helps identify storage consumption with a clear and intuitive interface.".to_string(),
            slug: "mdif".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/mdif/refs/heads/master/README.md".to_string(),
        },
        BlogPost {
            title: "XMRig Dashboard".to_string(),
            date: "2026-01-21".to_string(),
            description: "A web-based dashboard for monitoring and controlling XMRig cryptocurrency miner, featuring real-time hashrate monitoring and live CPU control.".to_string(),
            slug: "xmrig-dashboard".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/xmrig-dashboard/refs/heads/master/README.md".to_string(),
        },
        BlogPost {
            title: "PiFan - Raspberry Pi 5 Fan Controller".to_string(),
            date: "2026-01-21".to_string(),
            description: "A modern GUI application to take full control of your Raspberry Pi 5's active cooler fan speed using hardware PWM.".to_string(),
            slug: "pifan-controller".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/PiFan/refs/heads/master/README.md".to_string(),
        },
        BlogPost {
            title: "Bad Apple on RP2350 (RISC-V)".to_string(),
            date: "2026-01-21".to_string(),
            description: "Playing the iconic 'Bad Apple' video on a Raspberry Pi Pico 2 (RP2350) using the RISC-V hazard3 cores and SSD1306.".to_string(),
            slug: "bad-apple-rp2350".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/pico2-riscv-badapple/refs/heads/master/README.md".to_string(),
        },
        BlogPost {
            title: "fastfetchus".to_string(),
            date: "2026-01-21".to_string(),
            description: "A custom system information fetching tool inspired by fastfetch/neofetch, designed for efficiency and aesthetics.".to_string(),
            slug: "fastfetchus".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/fastfetchus/refs/heads/master/README.md".to_string(),
        },
        BlogPost {
            title: "DeneyapA1V2 Bad Apple".to_string(),
            date: "2026-01-21".to_string(),
            description: "Bad Apple video player for ESP32 with SSD1306 OLED, utilising Heatshrink compression for efficient video storage.".to_string(),
            slug: "deneyapa1v2-bad-apple".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/DeneyapA1V2-BadApple/master/README.md".to_string(),
        },
         BlogPost {
            title: "NixOS".to_string(),
            date: "2025-09-30".to_string(),
            description: "A comprehensive NixOS configuration with Hyprland window manager, development tools, and reproducible system management".to_string(),
            slug: "nixos".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/nixos/refs/heads/master/README.md".to_string(),
        },
         BlogPost {
            title: "NeuralRust".to_string(),
            date: "2025-09-5".to_string(),
            description: "A neural network implementation written in Rust, designed for learning and experimentation with machine learning concepts.".to_string(),
            slug: "neuralrust".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/NeuralRust/refs/heads/master/README.md".to_string(),
        },
         BlogPost {
            title: "rust-p2p-chat".to_string(),
            date: "2025-08-22".to_string(),
            description: "A peer-to-peer chat application built with Rust.".to_string(),
            slug: "rust-p2p-chat".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/rust-p2p-chat/refs/heads/main/README.md".to_string(),
        },
    ]
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = get_blog_posts();

    view! {
        <div class="relative z-10">
            <main class="max-w-4xl mx-auto px-6 py-32 lg:py-40">
                <div class="text-center mb-16 space-y-4">
                    <h1 class="text-5xl lg:text-7xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-primary via-white to-secondary animate-fade-in">
                        "Blog"
                    </h1>
                     <Text variant="muted" size="lg" class="max-w-xl mx-auto">
                        "Explorations in Rust, Embedded Systems, and Web Development."
                    </Text>
                </div>

                <div class="grid gap-6">
                    {posts.into_iter().map(|post| {
                        view! {
                            <A href=format!("/blog/{}", post.slug) attr:class="block group">
                                <Card variant="Bordered" class="p-8 hover:bg-surface/50 transition-all duration-300 border-white/5 hover:border-primary/20">
                                    <div class="flex flex-col gap-3">
                                        <div class="flex justify-between items-start">
                                            <h2 class="text-2xl font-bold text-primary group-hover:text-white transition-colors duration-300">
                                                {post.title}
                                            </h2>
                                            <span class="text-xs font-mono text-muted bg-white/5 px-2 py-1 rounded">
                                                {post.date}
                                            </span>
                                        </div>
                                        <p class="text-muted text-base leading-relaxed group-hover:text-foreground/80 transition-colors duration-300">
                                            {post.description}
                                        </p>

                                        <div class="pt-4 flex items-center gap-2 text-sm text-secondary/0 group-hover:text-secondary/100 transition-all duration-300 transform translate-y-2 group-hover:translate-y-0 opacity-0 group-hover:opacity-100">
                                            <span>"Read Article"</span>
                                            <i class="fa-solid fa-arrow-right"></i>
                                        </div>
                                    </div>
                                </Card>
                            </A>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </main>
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").unwrap_or_default();
    
    let post_resource = Resource::new(
        move || slug(),
        |slug| async move {
             if let Some(post) = get_blog_posts().into_iter().find(|p| p.slug == slug) {
                let content = get_repo_readme(post.readme_url.clone()).await.unwrap_or_default();
                Some((post, content))
             } else {
                 None
             }
        },
    );

    view! {
        <Suspense fallback=move || view! { <div class="text-center py-20 text-muted">"Loading..."</div> }>
            <div class="relative z-10">
                <main class="max-w-3xl mx-auto px-6 py-32 lg:py-40">
                    {move || match post_resource.get().flatten() {
                        Some((p, content)) => view! {
                            <article class="prose prose-invert prose-lg max-w-none">
                                <div class="mb-12 border-b border-white/10 pb-8">
                                    <A href="/blog" attr:class="no-underline text-sm text-muted hover:text-primary mb-8 inline-flex items-center gap-2 transition-colors">
                                        <i class="fa-solid fa-arrow-left"></i>
                                        "Back to Blog"
                                    </A>
                                    <h1 class="text-4xl lg:text-6xl font-black tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-primary to-white mb-4">
                                        {p.title}
                                    </h1>
                                    <div class="flex items-center gap-4 text-sm text-muted font-mono">
                                        <span>{p.date}</span>
                                    </div>
                                </div>
                                
                                <div class="blog-content text-foreground/90 leading-relaxed space-y-6" inner_html=md_to_html(&content)>
                                </div>
                            </article>
                        }.into_any(),
                        None => view! {
                            <div class="text-center py-20">
                                <h2 class="text-3xl font-bold text-white mb-4">"Post not found"</h2>
                                 <A href="/blog" attr:class="text-primary hover:text-secondary underline">
                                    "Return to Blog"
                                </A>
                            </div>
                        }.into_any()
                    }}
                </main>
            </div>
        </Suspense>
    }
}

fn md_to_html(md: &str) -> String {
    let parser = pulldown_cmark::Parser::new(md);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
