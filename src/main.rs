use clap::{ArgAction, Parser};
use colored::Colorize;
use std::collections::HashSet;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::{env, fs};

const PYCACHE: &str = "__pycache__";

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(
        short = 'l',
        long = "loc",
        value_parser,
        help = "Runs the program starting from other path than the current dir"
    )]
    localization: Option<String>,
    #[clap(short = 'n', value_parser)]
    max_depth: Option<i32>,
    #[clap(long = "dry", action = ArgAction::SetTrue, help = "Makes it dry run")]
    dry: bool,
    #[clap(long = "dirname", short = 'd', value_parser)]
    dirname: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Arguments::parse();
    let loc = cli.localization;
    let max_depth = cli.max_depth;
    let dry = cli.dry;
    let dirname = match cli.dirname {
        Some(dirname) => dirname,
        None => PYCACHE.into(),
    };

    remove_pycache_directories(loc, max_depth, dry, &dirname)?;
    Ok(())
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct PathBufWithDepth {
    path: PathBuf,
    depth: i32,
}

fn remove_pycache_directories(
    loc: Option<String>,
    max_depth: Option<i32>,
    dry: bool,
    dirname: &str,
) -> std::io::Result<()> {
    let current_dir: PathBuf;
    if let Some(loc) = loc {
        current_dir = canonicalize(loc)?;
    } else {
        current_dir = env::current_dir()?;
    }
    println!("current dir: {}", current_dir.to_str().unwrap().blue());

    let mut closed: HashSet<PathBufWithDepth> = HashSet::new();
    let mut paths_queue: Vec<PathBufWithDepth> = Vec::new();
    paths_queue.push(PathBufWithDepth {
        path: current_dir,
        depth: 0, // base
    });
    while !paths_queue.is_empty() {
        let current = paths_queue.pop().unwrap();
        closed.insert(current.clone());
        if let Some(max_depth) = max_depth {
            if current.depth > max_depth {
                continue;
            }
        }
        for entry in fs::read_dir(current.path)? {
            let entry = entry?;
            let path = entry.path();
            let pbwd = PathBufWithDepth {
                path: path.clone(),
                depth: current.depth + 1,
            };

            if path.is_dir() && !path.is_symlink() && !closed.contains(&pbwd) {
                if entry.file_name().eq_ignore_ascii_case(dirname) {
                    println!("removing {}", path.to_str().unwrap().yellow());
                    if !dry {
                        fs::remove_dir_all(path)?;
                    }
                } else {
                    // add path to queue
                    paths_queue.push(pbwd);
                }
            }
        }
    }
    Ok(())
}
