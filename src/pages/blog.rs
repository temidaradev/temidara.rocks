use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub description: String,
    pub slug: String,
    pub content: String,
}

#[derive(Default)]
struct PostMetadata {
    title: Option<String>,
    date: Option<String>,
    description: Option<String>,
    slug: Option<String>,
}

pub fn get_blog_posts() -> Vec<BlogPost> {
    let mut posts = BLOG_SOURCES
        .iter()
        .map(|(file_name, source)| parse_blog_post(file_name, source))
        .collect::<Vec<_>>();

    posts.sort_by(|left, right| right.date.cmp(&left.date));
    posts
}

#[cfg(feature = "ssr")]
pub fn get_blog_asset(file: &str) -> Option<(&'static str, &'static [u8])> {
    BLOG_ASSETS
        .iter()
        .find_map(|(name, mime, bytes)| (*name == file).then_some((*mime, *bytes)))
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = get_blog_posts();

    view! {
        <div class="space-y-8">
            <h1 class="page-title">"blog"</h1>

            {if posts.is_empty() {
                view! {
                    <p class="text-sm text-gray-500">"No posts yet. Check back soon."</p>
                }.into_any()
            } else {
                view! {
                    <div class="grid gap-6">
                        {posts.into_iter().map(|post| {
                            let href = format!("/blog/{}", post.slug);
                            view! {
                                <article class="listing-row group relative">
                                    <div class="mb-1 flex flex-col gap-1 sm:flex-row sm:items-baseline sm:justify-between sm:gap-4">
                                        <A href=href attr:class="font-bold text-gray-200 hover:text-white hover:underline">
                                            {post.title}
                                        </A>
                                        <span class="text-xs font-mono text-gray-600 whitespace-nowrap">
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
                }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        {move || {
            let slug = params.get().get("slug").unwrap_or_default();
            match get_blog_posts().into_iter().find(|post| post.slug == slug) {
                Some(post) => view! {
                    <article class="max-w-none">
                        <div class="mb-12 border-b border-white/10 pb-8">
                            <A href="/blog" attr:class="no-underline text-xs font-mono text-gray-500 hover:text-white mb-6 block">
                                "<- back"
                            </A>
                            <h1 class="text-3xl lg:text-4xl font-bold text-white mb-2 tracking-tight">
                                {post.title}
                            </h1>
                            <time class="text-xs font-mono text-gray-500">{post.date}</time>
                        </div>

                        <div
                            class="blog-content text-gray-300 leading-relaxed"
                            inner_html=markdown_to_html(&post.content)
                        ></div>
                    </article>
                }.into_any(),
                None => view! {
                    <div class="py-20 space-y-4">
                        <h2 class="text-xl font-bold text-white">"404: post not found"</h2>
                        <A href="/blog" attr:class="underline text-gray-400 hover:text-white">
                            "return to blog"
                        </A>
                    </div>
                }.into_any(),
            }
        }}
    }
}

fn parse_blog_post(file_name: &str, source: &str) -> BlogPost {
    let (metadata, body) = split_front_matter(source);
    let title = metadata
        .title
        .or_else(|| markdown_title(body))
        .unwrap_or_else(|| file_stem(file_name).replace(['-', '_'], " "));
    let content = strip_leading_title(body).trim().to_string();
    let description = metadata
        .description
        .unwrap_or_else(|| first_paragraph(&content));
    let slug = metadata
        .slug
        .unwrap_or_else(|| slugify(file_stem(file_name)));

    BlogPost {
        title,
        date: metadata.date.unwrap_or_default(),
        description,
        slug,
        content,
    }
}

fn split_front_matter(source: &str) -> (PostMetadata, &str) {
    let Some(rest) = source.strip_prefix("---\n") else {
        return (PostMetadata::default(), source);
    };
    let Some((front_matter, body)) = rest.split_once("\n---\n") else {
        return (PostMetadata::default(), source);
    };

    let mut metadata = PostMetadata::default();
    for line in front_matter.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim().trim_matches(['\'', '"']).to_string();
        match key.trim() {
            "title" => metadata.title = Some(value),
            "date" => metadata.date = Some(value),
            "description" => metadata.description = Some(value),
            "slug" => metadata.slug = Some(value),
            _ => {}
        }
    }

    (metadata, body)
}

fn markdown_title(markdown: &str) -> Option<String> {
    markdown
        .lines()
        .find_map(|line| line.trim().strip_prefix("# ").map(str::to_string))
}

fn strip_leading_title(markdown: &str) -> &str {
    let trimmed = markdown.trim_start();
    match trimmed.lines().next() {
        Some(line) if line.starts_with("# ") => trimmed
            .strip_prefix(line)
            .unwrap_or(trimmed)
            .trim_start_matches(['\r', '\n']),
        _ => markdown,
    }
}

fn first_paragraph(markdown: &str) -> String {
    let mut lines = Vec::new();

    for line in markdown.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !lines.is_empty() {
                break;
            }
            continue;
        }
        if lines.is_empty()
            && (line.starts_with('#')
                || line.starts_with("```")
                || line.starts_with("- ")
                || line.starts_with("* ")
                || line.starts_with('>'))
        {
            continue;
        }
        lines.push(line);
    }

    lines.join(" ")
}

fn file_stem(file_name: &str) -> &str {
    file_name.strip_suffix(".md").unwrap_or(file_name)
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;

    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_was_separator = false;
        } else if !slug.is_empty() && !last_was_separator {
            slug.push('-');
            last_was_separator = true;
        }
    }

    slug.trim_end_matches('-').to_string()
}

fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{CowStr, Event, Options, Parser, Tag};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) if is_relative_asset(&dest_url) => {
            let path = dest_url.trim_start_matches("./");
            Event::Start(Tag::Image {
                link_type,
                dest_url: CowStr::from(format!("/blog-assets/{path}")),
                title,
                id,
            })
        }
        event => event,
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

fn is_relative_asset(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.starts_with('#')
        && !path.starts_with("data:")
        && !path.contains("://")
        && !path.split('/').any(|part| part == "..")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_metadata_from_plain_markdown() {
        let post = parse_blog_post(
            "Fedora-VMs-Blog.md",
            "# Running Fedora everywhere\n\nThe first paragraph.\n\n## Details\n",
        );

        assert_eq!(post.title, "Running Fedora everywhere");
        assert_eq!(post.slug, "fedora-vms-blog");
        assert_eq!(post.description, "The first paragraph.");
        assert!(!post.content.starts_with("# "));
    }

    #[test]
    fn supports_optional_front_matter() {
        let post = parse_blog_post(
            "ignored.md",
            "---\ntitle: A title\ndate: 2026-08-09\ndescription: A summary\nslug: custom-slug\n---\n\nPost body.",
        );

        assert_eq!(post.title, "A title");
        assert_eq!(post.date, "2026-08-09");
        assert_eq!(post.description, "A summary");
        assert_eq!(post.slug, "custom-slug");
    }

    #[test]
    fn rewrites_relative_blog_images() {
        let html = markdown_to_html("![layout](image-1.png)");
        assert!(html.contains("src=\"/blog-assets/image-1.png\""));
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn embeds_blog_images() {
        let (mime, bytes) = get_blog_asset("image-1.png").expect("embedded blog image");
        assert_eq!(mime, "image/png");
        assert!(!bytes.is_empty());
    }
}
