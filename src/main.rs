use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Args {
    root: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    walk(&args.root)
}

fn walk(path: &Path) -> anyhow::Result<()> {
    let Ok(dir) = std::fs::read_dir(path) else {
        return Ok(());
    };

    for e in dir.flatten() {
        let path = e.path();

        let Ok(m) = e.metadata() else {
            continue;
        };

        if !m.is_dir() {
            continue;
        };

        let Some(file_name) = path.file_name() else {
            continue;
        };

        if file_name == ".git" {
            Command::new("git")
                .arg("gc")
                .current_dir(&path)
                .spawn()
                .with_context(|| format!("git gc {path:?}"))?;
            continue;
        }

        walk(&path)?;
    }

    Ok(())
}
