use crate::api::blog::get_repo_readme;
use crate::components::MandelbrotViewer;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
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
            title: "Mandelbrot-rust".to_string(),
            date: "2026-02-03".to_string(),
            description: "A Rust implementation of the Mandelbrot set".to_string(),
            slug: "mandelbrot-rust".to_string(),
            readme_url: "https://raw.githubusercontent.com/temidaradev/mandelbrot-rust/refs/heads/master/README.md".to_string(),
        },
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
        <div class="space-y-8">
             <h1 class="text-xl font-bold tracking-tight text-white">"blog"</h1>

            <div class="grid gap-6">
                {posts.into_iter().map(|post| {
                    view! {
                         <article class="group relative">
                            <div class="flex items-baseline justify-between mb-1">
                                <a href=format!("/blog/{}", post.slug) class="font-bold text-gray-200 hover:text-white hover:underline">
                                    {post.title}
                                </a>
                                <span class="text-xs font-mono text-gray-600">
                                    {post.date}
                                </span>
                            </div>
                            <p class="text-sm text-gray-500 leading-snug line-clamp-2">
                                {post.description}
                            </p>
                        </article>
                    }
                }).collect::<Vec<_>>()}
            </div>
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
                let content = get_repo_readme(post.readme_url.clone())
                    .await
                    .unwrap_or_default();
                Some((post, content))
            } else {
                None
            }
        },
    );

    view! {
        <Suspense fallback=move || view! { <div class="font-mono text-sm text-gray-500">"loading..."</div> }>
            {move || match post_resource.get().flatten() {
                Some((p, content)) => view! {
                    <article class="max-w-none">
                        <div class="mb-12 border-b border-white/10 pb-8">
                            <A href="/blog" attr:class="no-underline text-xs font-mono text-gray-500 hover:text-white mb-6 block">
                                "<- back"
                            </A>
                            <h1 class="text-3xl lg:text-4xl font-bold text-white mb-2 tracking-tight">
                                {p.title.clone()}
                            </h1>
                            <time class="text-xs font-mono text-gray-500">{p.date}</time>
                        </div>

                        {if p.slug == "mandelbrot-rust" {
                            view! { <MandelbrotViewer /> }.into_any()
                        } else {
                            view! { <></> }.into_any()
                        }}

                        <div class="blog-content text-gray-300 leading-relaxed" inner_html=md_to_html(&content)>
                        </div>
                    </article>
                }.into_any(),
                None => view! {
                    <div class="py-20 space-y-4">
                        <h2 class="text-xl font-bold text-white">"404: post not found"</h2>
                         <A href="/blog" attr:class="underline text-gray-400 hover:text-white">
                            "return to blog"
                        </A>
                    </div>
                }.into_any()
            }}
        </Suspense>
    }
}

fn md_to_html(md: &str) -> String {
    use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(md, options);
    let mut new_events = Vec::new();
    let mut in_video_link = false;

    for event in parser {
        if in_video_link {
            if let Event::End(TagEnd::Link) = event {
                in_video_link = false;
            }
            continue;
        }

        let mut replaced = false;
        match &event {
            Event::Text(text) => {
                if (text.starts_with("http://") || text.starts_with("https://"))
                    && is_video_url(text)
                {
                    new_events.push(Event::Html(CowStr::from(video_player_html(text))));
                    replaced = true;
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                if is_video_url(dest_url) {
                    new_events.push(Event::Html(CowStr::from(video_player_html(dest_url))));
                    in_video_link = true;
                    replaced = true;
                }
            }
            _ => {}
        }

        if !replaced {
            new_events.push(event);
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, new_events.into_iter());
    html_output
}

fn is_video_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    lower.ends_with(".mp4")
        || lower.ends_with(".mov")
        || lower.ends_with(".webm")
        || lower.ends_with(".m4v")
        || lower.contains("user-attachments/assets")
}

fn video_player_html(url: &str) -> String {
    format!(
        r#"<video controls playsinline preload="metadata" class="w-full rounded-lg border border-white/10 my-4 shadow-lg bg-black/50" style="max-height: 600px;">
            <source src="{}" type="video/mp4">
            <p class="text-sm text-gray-500 text-center py-2">
                 Unable to play video. <a href="{}" class="text-blue-400 hover:underline" target="_blank">Download here</a>
            </p>
        </video>"#,
        url, url
    )
}
