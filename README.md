# Temidaradev Portfolio (Rust)

A personal portfolio website built with Rust, [Leptos](https://leptos.dev/) (SSR + hydration), and TailwindCSS.

<img width="743" height="1076" alt="image" src="https://github.com/user-attachments/assets/2659e414-1497-42fb-a621-12e6e5f45e24" />

## Features

- **Home** – hero section with a "now playing" widget backed by [ListenBrainz](https://listenbrainz.org/).
- **Blog** – project write-ups rendered from GitHub `README.md` files fetched at request time and parsed with `pulldown-cmark`.
- **Experiences** – work and project history.
- **Contact** – form that sends email over SMTP via `lettre`.
- **Mandelbrot** – an interactive WASM viewer embedded in the blog.
- Analytics via [Umami](https://umami.is/) (optional).

## Project Structure

- `src/main.rs`: Entry point for the server-side rendering (SSR) application (Axum server).
- `src/lib.rs`: Library entry point, handles client-side hydration.
- `src/app.rs`: Main application component; sets up the router and layout.
- `src/pages/`: Page components — `home.rs`, `blog.rs`, `experiences.rs`, `contact.rs`.
- `src/components/`: Reusable UI components (`navbar.rs`, `footer.rs`, `mandelbrot.rs`).
- `src/api/`: Server functions and integrations — `music.rs` (ListenBrainz), `blog.rs` (GitHub README fetch), `contact.rs` (SMTP email).

## Usage

### Prerequisites
- Rust (latest stable)
- `cargo-leptos` — `cargo install cargo-leptos`
- The `wasm32-unknown-unknown` target — `rustup target add wasm32-unknown-unknown`

### Environment Variables

Create a `.env` file (loaded via `dotenvy` in SSR mode):

| Variable | Purpose |
| --- | --- |
| `LISTENBRAINZ_USERNAME` | ListenBrainz user for the "now playing" widget |
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
