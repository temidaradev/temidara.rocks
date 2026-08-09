use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rustc-env=ASSET_VERSION={:016x}", asset_hash());

    let blog_dir = PathBuf::from("content/blog");
    println!("cargo:rerun-if-changed={}", blog_dir.display());

    let mut markdown_files = fs::read_dir(&blog_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        })
        .collect::<Vec<_>>();

    markdown_files.sort_by_key(|path| {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
    });

    let mut generated = String::from("const BLOG_SOURCES: &[(&str, &str)] = &[\n");
    for path in markdown_files {
        let absolute_path = fs::canonicalize(&path).unwrap_or(path.clone());
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        let _ = writeln!(
            generated,
            "    ({:?}, include_str!({:?})),",
            file_name,
            absolute_path.to_string_lossy()
        );
    }
    generated.push_str("];\n");

    let mut asset_files = fs::read_dir(&blog_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| blog_asset_mime(path).is_some())
        .collect::<Vec<_>>();

    asset_files.sort_by_key(|path| {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
    });

    generated
        .push_str("#[cfg(feature = \"ssr\")]\nconst BLOG_ASSETS: &[(&str, &str, &[u8])] = &[\n");
    for path in asset_files {
        let absolute_path = fs::canonicalize(&path).unwrap_or(path.clone());
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        let mime = blog_asset_mime(&path).expect("filtered blog asset should have a MIME type");
        let _ = writeln!(
            generated,
            "    ({:?}, {:?}, include_bytes!({:?})),",
            file_name,
            mime,
            absolute_path.to_string_lossy()
        );
    }
    generated.push_str("];\n");

    let output_path =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set")).join("blog_posts.rs");
    fs::write(output_path, generated).expect("failed to generate the blog post index");
}

fn asset_hash() -> u64 {
    let mut paths = vec![
        PathBuf::from("input.css"),
        PathBuf::from("tailwind.config.js"),
    ];
    collect_files(&PathBuf::from("src"), &mut paths);
    paths.sort();

    let mut hash = 0xcbf29ce484222325_u64;
    for path in paths {
        update_hash(&mut hash, path.to_string_lossy().as_bytes());
        if let Ok(bytes) = fs::read(path) {
            update_hash(&mut hash, &bytes);
        }
    }
    hash
}

fn collect_files(directory: &std::path::Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, paths);
        } else {
            paths.push(path);
        }
    }
}

fn update_hash(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}

fn blog_asset_mime(path: &std::path::Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "avif" => Some("image/avif"),
        "gif" => Some("image/gif"),
        "jpeg" | "jpg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "svg" => Some("image/svg+xml"),
        "webp" => Some("image/webp"),
        _ => None,
    }
}
