# Temidaradev Portfolio (Rust)

A personal portfolio website built with Rust, Leptos, and TailwindCSS.

<img width="749" height="989" alt="image" src="https://github.com/user-attachments/assets/8f601912-04aa-4fd0-9b13-3c913bbc3c7a" />

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
