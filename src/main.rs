use clap::Parser;
mod cit_file;
use cit_file::CitFile;

#[derive(Parser, Debug)]
#[command(author, version, about = "Single file version control tool")]
struct Args {
    /// The filename or repository to operate on
    filename: String,

    /// Initialize the file/repository
    #[arg(long)]
    init: bool,

    /// Create a new version
    #[arg(long)]
    add: Option<String>,

    /// Commit changes to current version
    #[arg(long)]
    commit: bool,

    /// List all versions
    #[arg(long)]
    list: bool,

    /// Clear the repository or file
    #[arg(long)]
    clear: bool,

    /// Switch to a specific version
    #[arg(long)]
    switch: Option<String>,


    /// Remove a specific version
    #[arg(long)]
    remove: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    let mut cit_file = CitFile::new(&args.filename);

    if args.init {
        cit_file.init();
    } else if let Some(version) = args.add {
        cit_file.add_version(&version);
    } else if args.list {
        cit_file.list_versions();
    } else if args.clear {
        cit_file.clear();
    } else if let Some(version) = args.switch {
        cit_file.switch(&version, false);
    } else if args.commit {
        cit_file.commit();
    } else if let Some(version) = args.remove {
        cit_file.remove(&version);
    }  else {
        println!("No valid option provided. Use --help for usage.");
    }
}