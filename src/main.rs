use regex;
use std::env;
use std::error;
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// returns the current working directory if exists
fn cwd() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "f", about = "File finding utility")]
struct Opt {
    /// Optional root directory to use
    #[structopt(short, long)]
    root: Option<String>,

    /// Optional regexp to ignore
    #[structopt(short, long)]
    ignore: Option<String>,

    /// Regexp to search for
    #[structopt(name = "PATTERN")]
    pattern: String,
}

fn ignore_libraries(entry: &DirEntry) -> bool {
    let ignore_list = vec!["node_modules", "venv"];
    entry
        .file_name()
        .to_str()
        .map(|s| {
            for i in ignore_list.iter() {
                if s.contains(i) {
                    return true;
                }
            }
            return false;
        })
        .unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() -> Result<()> {
    // parse options
    let opt = Opt::from_args();

    // set walker to root or current working directory 
    // if none specified
    let walker = WalkDir::new(opt.root.unwrap_or(cwd()));

    // parse the user's pattern of files to include
    let r = regex::Regex::new(&opt.pattern)?;

    // parse the user's pattern of files to ignore (if they exist)
    let to_ignore = opt.ignore.unwrap_or("".to_string());
    let ig = regex::Regex::new(&to_ignore)?;

    // walk the directory, checking for hidden files or programming
    // libraries almost certainly not being searched for
    // matching regexes for user's input
    for entry in walker
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !ignore_libraries(e))
    {

        let entry = entry?;
        if let Some(x) = entry.file_name().to_str() {
            if r.is_match(x) {
                if to_ignore != "" {
                    if ig.is_match(entry.path().to_str().unwrap()) {
                        continue;
                    }
                }
                println!("{}", entry.path().display());
            }
        }
    }
    Ok(())
}
