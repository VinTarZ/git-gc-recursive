use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Args {
    root: PathBuf,

    #[arg(long)]
    dry_run: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut gits = vec![];

    println!("finding gits");

    find_gits(&mut gits, &args.root).context("find gits")?;

    let mut current = 0usize;
    let total = gits.len();

    for git in gits {
        current += 1;

        if !args.dry_run {
            println!();
        }

        println!("{current}/{total}: {git:?}");

        if args.dry_run {
            continue;
        }

        Command::new("git")
            .arg("gc")
            .current_dir(&git)
            .spawn()
            .context("run git gc")?
            .wait()
            .context("wait git gc")?;
    }

    Ok(())
}

fn find_gits(gits: &mut Vec<PathBuf>, dir: &Path) -> anyhow::Result<()> {
    let Ok(iter) = std::fs::read_dir(dir) else {
        return Ok(());
    };

    for e in iter.flatten() {
        let path = e.path();

        let Some(file_name) = path.file_name() else {
            continue;
        };

        if !e.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }

        if file_name == ".git" {
            gits.push(dir.to_owned());
            continue;
        }

        find_gits(gits, &path)?;
    }

    Ok(())
}
