//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

#![allow(clippy::unwrap_used, reason = "It's OK to panic in build scripts")]

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let minified_templates = out.join("templates");

    minify_templates(current_dir.join("templates"), &minified_templates);

    {
        let mut askama_config = File::create(current_dir.join("askama.toml")).unwrap();

        write!(
            askama_config,
            "[general]\ndirs = [\"{}\"]",
            minified_templates
                .strip_prefix(current_dir)
                .unwrap()
                .display()
                .to_string()
                .replace("\\", "/")
        )
        .unwrap();
    }

    // Put linker scripts in our output directory and ensure it's
    // on the linker search path.
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying the linker
    // scripts here, we ensure the build script is only re-run when
    // they are changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
}

fn minify_templates(templates_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    for entry in WalkDir::new(&templates_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().is_file()
            && matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("askama" | "html" | "js" | "css")
            )
        {
            println!("cargo:rerun-if-changed={}", path.display());

            let contents = std::fs::read(entry.path()).unwrap();

            let mut cfg = minify_html::Cfg::new();
            cfg.keep_closing_tags = true;
            cfg.keep_html_and_head_opening_tags = true;
            cfg.minify_css = true;
            cfg.minify_js = true;
            cfg.preserve_brace_template_syntax = true;

            let minified = minify_html::minify(&contents, &cfg);

            let out_dir = out_dir
                .as_ref()
                .join(path.parent().unwrap().strip_prefix(&templates_dir).unwrap());

            if !out_dir.exists() {
                fs::create_dir_all(&out_dir).unwrap();
            }

            let out_path = out_dir.join(path.file_name().unwrap());
            fs::write(out_path, minified).unwrap();
        }
    }
}
