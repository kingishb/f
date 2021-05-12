use clap::Clap;
use std::env;

use ignore::{ WalkBuilder};

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // parse options
    let opt = Opt::parse();

    // set walker to root or current working directory
    // if none specified
    // let walker = WalkDir::new(opt.root.unwrap_or_else(cwd));
    let walker = WalkBuilder::new(opt.root.unwrap_or_else(cwd))
        .threads(6)
        .build_parallel();

    // parse the user's pattern of files to include
    let search = regex::Regex::new(&opt.pattern)?;

    
        walker.run(|| {
            Box::new( |result| {
                use ignore::WalkState::*;
                match result {
                    Ok(entry) => {
                        let s = entry.path().display().to_string();
                        if search.is_match(&s) {
                            println!("{}", s);
                        }
                    },
                    Err(e) => println!("{}", e),
                };
                Continue
            })
        });

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


    /// Regexp to search for
    #[clap(name = "PATTERN")]
    pattern: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempdir;


    #[test]
    fn test_cwd() {
        let current_dir = cwd();
        assert!(current_dir.contains("/f"));
    }
}
