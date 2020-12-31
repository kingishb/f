use clap::Clap;
use std::env;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Box<dyn Error>> {
    // parse options
    let opt = Opt::parse();

    // set walker to root or current working directory
    // if none specified
    let walker = WalkDir::new(opt.root.unwrap_or_else(cwd));

    // parse the user's pattern of files to include
    let search = regex::Regex::new(&opt.pattern)?;

    // parse the user's pattern of files to ignore (if they exist)
    let to_ignore = opt.ignore.unwrap_or_else(|| "".to_string());
    let ig = regex::Regex::new(&to_ignore)?;

    // walk the directory, checking for hidden files or programming
    // libraries almost certainly not being searched for
    // matching regexes for user's input
    for entry in walker
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !ignore_libraries(e))
    {
        match entry {
            Ok(e) => {
                if let Some(x) = e.file_name().to_str() {
                    if search.is_match(x) {
                        if to_ignore != "" && ig.is_match(e.path().to_str().unwrap()) {
                            continue;
                        }
                        println!("{}", e.path().display());
                    }
                }
            }
            Err(_) => {
                continue;
            }
        };
    }
    Ok(())
}

// returns the current working directory if exists
fn cwd() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

#[derive(Clap)]
#[clap(name = "f", author = "File finding utility")]
struct Opt {
    /// Optional root directory to use
    #[clap(short, long)]
    root: Option<String>,

    /// Optional regexp to ignore
    #[clap(short, long)]
    ignore: Option<String>,

    /// Regexp to search for
    #[clap(name = "PATTERN")]
    pattern: String,
}

// ignore common programming folders containing third party libraries
fn ignore_libraries(entry: &DirEntry) -> bool {
    let ignore_list = vec![".git", "node_modules", "venv"];
    entry
        .file_name()
        .to_str()
        .map(|s| {
            for i in ignore_list.iter() {
                if s.contains(i) {
                    return true;
                }
            }
            false
        })
        .unwrap_or(false)
}

// ignore hidden directories
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
