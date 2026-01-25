# Temidaradev Portfolio (Rust)

A personal portfolio website built with Rust, Leptos, and TailwindCSS.

<img width="743" height="1076" alt="image" src="https://github.com/user-attachments/assets/2659e414-1497-42fb-a621-12e6e5f45e24" />

## Project Structure

- `src/main.rs`: Entry point for the server-side rendering (SSR) application.
- `src/lib.rs`: Library entry point, handles hydration for the client-side.
- `src/app.rs`: Main application component, establishes the router and layout.
- `src/pages/`: Contains individual page components.
    - `home.rs`: The landing page with the hero section and Plex integration.
    - `blog.rs`, `contact.rs`, `experiences.rs`: Placeholder modules for future content.
- `src/components/`: Reusable UI components (NavBar, Cards, etc.).
- `src/api/`: Backend API integrations (e.g., Plex).

## Usage

### Prerequisites
- Rust (latest stable)
- `cargo-leptos`

### Running Development Server
```bash
cargo leptos watch
```

### Building for Release
```bash
cargo leptos build --release
```
