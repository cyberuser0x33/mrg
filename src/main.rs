mod commands;
mod config;
mod overview;
mod tokenizer;
mod utils;

use crate::commands::{
    CombineOptions, run_combine, run_file, run_get, run_init, run_structure, run_tokenize,
    run_tokenize_batch,
};
use crate::utils::select_directory;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mrg")]
#[command(about = "Project merger tool", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Combine project files (shortcut for combine subcommand)
    #[arg(short = 'c', long = "combine", value_name = "DIR")]
    combine: Option<Option<PathBuf>>,

    /// Clone repository and combine its files (shortcut for get subcommand)
    #[arg(short = 'g', long = "get", num_args(1..=2), value_names = ["URL", "DIR"])]
    get: Option<Vec<String>>,

    /// Show project structure (shortcut for structure subcommand)
    #[arg(short = 'S', long = "structure")]
    structure: bool,

    /// Set custom file size warning limit (e.g. 50b, 100kb, 2mb, 1gb)
    #[arg(short = 's', long = "size", value_name = "LIMIT", global = true)]
    size: Option<String>,

    /// Only include files matching these patterns, ignoring all others
    #[arg(short = 'o', long = "only", num_args(1..), value_name = "PATTERNS", global = true)]
    only: Option<Vec<String>>,

    /// Show merged file contents (shortcut for file subcommand)
    #[arg(short = 'f', long = "file")]
    file: bool,

    /// Update an existing merge file (shortcut for update subcommand)
    #[arg(short = 'u', long = "update", value_name = "DIR")]
    update: Option<Option<PathBuf>>,

    /// Tokenize a file using a selected tokenizer (interactive model selection)
    #[arg(short = 't', long = "tokenize", value_name = "FILE", num_args(0..=1))]
    tokenize: Option<Option<PathBuf>>,

    /// Tokenize a file using all available tokenizers (batch ASCII table)
    #[arg(long = "tb", value_name = "FILE", num_args(0..=1))]
    tokenize_batch: Option<Option<PathBuf>>,

    /// Split option: if token limit is exceeded, split into parts. Value for limit (e.g. 350K, 1.2M, default 500K)
    #[arg(long = "split", value_name = "LIMIT", default_missing_value = "500K", num_args = 0..=1, global = true)]
    split: Option<String>,

    /// Do not split option: ignore limit, write all to one file (takes precedence over split check)
    #[arg(long = "notsplit", global = true)]
    notsplit: bool,

    /// Ignore size check for individual files (> 100 KB)
    #[arg(short = 'i', long = "ignore", global = true)]
    ignore: bool,

    /// Prompt to choose processing pattern interactively
    #[arg(short = 'p', long = "pattern", global = true)]
    pattern: bool,

    /// Use Full processing mode (default)
    #[arg(long = "pattern-full", global = true)]
    pattern_full: bool,

    /// Use Minify processing mode (removes comments and extra spaces)
    #[arg(long = "pattern-min", global = true)]
    pattern_min: bool,

    /// Use Maximize processing mode (signatures/skeletons only). filters: d="dir" f="file"
    #[arg(long = "pattern-max", num_args(0..), value_name = "FILTERS", global = true)]
    pattern_max: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize .mrgignore file
    Init {
        /// Project name for the ignore file
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
    /// Combine project files
    Combine {
        /// Target directory
        #[arg(value_name = "DIR")]
        dir: Option<PathBuf>,
    },
    /// Clone repository and combine its files
    Get {
        /// Repository URL
        url: String,
        /// Target directory for output file
        dir: Option<PathBuf>,
    },
    /// Show project structure
    Structure,
    /// Show merged file contents
    File,
    /// Update an existing merge file
    Update {
        /// Target directory
        #[arg(value_name = "DIR")]
        dir: Option<PathBuf>,
    },
}

fn parse_size_limit(s: &str) -> Result<u64> {
    let s = s.trim().to_uppercase();
    if s.is_empty() {
        return Err(anyhow::anyhow!("Empty size limit"));
    }

    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    let num_str = &s[..num_end];
    let suffix = s[num_end..].trim();

    let val: f64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid number in size limit: {}", num_str))?;

    let bytes = match suffix {
        "B" | "" => val as u64,
        "KB" | "K" => (val * 1024.0) as u64,
        "MB" | "M" => (val * 1024.0 * 1024.0) as u64,
        "GB" | "G" => (val * 1024.0 * 1024.0 * 1024.0) as u64,
        _ => return Err(anyhow::anyhow!("Unknown size limit suffix: {}", suffix)),
    };
    Ok(bytes)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let size_limit = if let Some(ref s) = cli.size {
        match parse_size_limit(s) {
            Ok(limit) => Some(limit),
            Err(e) => {
                eprintln!("[!] Error parsing size limit: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let options = CombineOptions {
        is_update: false,
        split: cli.split.clone(),
        notsplit: cli.notsplit,
        ignore_size: cli.ignore,
        pattern: cli.pattern,
        pattern_full: cli.pattern_full,
        pattern_min: cli.pattern_min,
        pattern_max: cli.pattern_max.clone(),
        custom_project_name: None,
        custom_output_dir: None,
        size_limit,
        only: cli.only.clone(),
    };

    // Handle shortcuts
    if let Some(tokenize_batch_opt) = cli.tokenize_batch {
        let file_path = match tokenize_batch_opt {
            Some(path) => Some(path),
            None => None,
        };
        return run_tokenize_batch(file_path);
    }

    if let Some(tokenize_opt) = cli.tokenize {
        let file_path = match tokenize_opt {
            Some(path) => Some(path),
            None => None,
        };
        return run_tokenize(file_path);
    }

    if let Some(get_args) = cli.get {
        let url = get_args[0].clone();
        let dir = if get_args.len() > 1 {
            Some(PathBuf::from(&get_args[1]))
        } else {
            None
        };
        return run_get(&url, dir, options);
    }

    if let Some(dir_opt) = cli.combine {
        let dir = match dir_opt {
            Some(d) => d,
            None => select_directory()?,
        };
        return run_combine(dir, options);
    }
    if let Some(dir_opt) = cli.update {
        let dir = match dir_opt {
            Some(d) => d,
            None => select_directory()?,
        };
        let mut opts = options.clone();
        opts.is_update = true;
        return run_combine(dir, opts);
    }
    if cli.structure {
        return run_structure();
    }
    if cli.file {
        return run_file();
    }

    match cli.command {
        Some(Commands::Init { name }) => run_init(name),
        Some(Commands::Combine { dir }) => {
            let dir = match dir {
                Some(d) => d,
                None => select_directory()?,
            };
            run_combine(dir, options)
        }
        Some(Commands::Get { url, dir }) => run_get(&url, dir, options),
        Some(Commands::Update { dir }) => {
            let dir = match dir {
                Some(d) => d,
                None => select_directory()?,
            };
            let mut opts = options.clone();
            opts.is_update = true;
            run_combine(dir, opts)
        }
        Some(Commands::Structure) => run_structure(),
        Some(Commands::File) => run_file(),
        None => {
            println!("Use 'mrg --help' for usage info.");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_limit() {
        assert_eq!(parse_size_limit("100b").unwrap(), 100);
        assert_eq!(parse_size_limit("100 B").unwrap(), 100);
        assert_eq!(parse_size_limit("1.5kb").unwrap(), 1536);
        assert_eq!(parse_size_limit("2mb").unwrap(), 2097152);
        assert_eq!(parse_size_limit("1GB").unwrap(), 1073741824);
        assert!(parse_size_limit("invalid").is_err());
    }
}
