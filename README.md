# Temidaradev Portfolio (Rust)

A personal portfolio website built with Rust, [Leptos](https://leptos.dev/) (SSR + hydration), and TailwindCSS.

<img width="819" height="1293" alt="image" src="https://github.com/user-attachments/assets/05a1a9a6-7e8c-4ee5-8c06-9369d87e2eac" />

## Features

- **Home** – hero section with a "now playing" widget backed by [ListenBrainz](https://listenbrainz.org/).
- **Blog** – a dedicated space for articles and personal writing.
- **Projects** – project write-ups rendered from GitHub `README.md` files fetched at request time and parsed with `pulldown-cmark`.
- **Uses** – current operating systems and development languages.
- **Command palette** – search pages, projects, and posts with `/` or `Ctrl`/`Cmd` + `K`.
- **Guestbook** – moderated visitor messages with validation and rate limiting.
- **Status** – live server architecture and uptime.
- **Experiences** – work and project history.
- **Contact** – form that sends email over SMTP via `lettre`.
- **Mandelbrot** – an interactive WASM viewer embedded in its project page.
- Analytics via [Umami](https://umami.is/) (optional).

## Project Structure

- `src/main.rs`: Entry point for the server-side rendering (SSR) application (Axum server).
- `src/lib.rs`: Library entry point, handles client-side hydration.
- `src/app.rs`: Main application component; sets up the router and layout.
- `src/pages/`: Page components — `home.rs`, `blog.rs`, `projects.rs`, `experiences.rs`, `contact.rs`.
- `src/components/`: Reusable UI components (`navbar.rs`, `footer.rs`, `mandelbrot.rs`).
- `src/api/`: Server functions and integrations — `music.rs` (ListenBrainz), `projects.rs` (GitHub README fetch), `contact.rs` (SMTP email).

## Usage

### Adding a blog post

Add a Markdown file to `content/blog/`. The first `#` heading is used as the
title, the first paragraph as the description, and the filename as the URL slug.
For example, `Fedora-VMs-Blog.md` is available at `/blog/fedora-vms-blog`.

Optional metadata can be added at the top of a post:

```yaml
---
title: Running Fedora everywhere
date: 2026-08-09
description: Notes on emulating Fedora across architectures.
slug: fedora-vms
---
```

Blog files are embedded at build time, so rebuild the site after adding or
editing a post.

### Prerequisites
- Rust (latest stable)
- `cargo-leptos` — `cargo install cargo-leptos`
- The `wasm32-unknown-unknown` target — `rustup target add wasm32-unknown-unknown`

### Environment Variables

Create a `.env` file (loaded via `dotenvy` in SSR mode):

| Variable | Purpose |
| --- | --- |
| `LISTENBRAINZ_USERNAME` | ListenBrainz user for the "now playing" widget |
| `COVER_CACHE_DIR` | Album-cover cache directory (defaults to `data/covers`) |
| `GUESTBOOK_PATH` | Guestbook JSON file (defaults to `data/guestbook.json`) |
| `GUESTBOOK_ADMIN_TOKEN` | Token used at `/guestbook/moderate` to approve or delete messages |
| `GUESTBOOK_AUTO_APPROVE` | Set to `true` to publish new messages without moderation |
| `SMTP_HOST` | SMTP server host for the contact form |
| `SMTP_USERNAME` | SMTP username / sender address |
| `SMTP_PASSWORD` | SMTP password |
| `CONTACT_RECEIVER` | Recipient for contact submissions (defaults to `SMTP_USERNAME`) |
| `UMAMI_SCRIPT_URL` | Umami analytics script URL (optional) |
| `UMAMI_WEBSITE_ID` | Umami website ID (optional) |

### Running Development Server
```bash
cargo leptos watch
```
The site is served at `http://127.0.0.1:3006`.

### Building for Release
```bash
cargo leptos build --release
```
