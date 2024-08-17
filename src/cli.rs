use clap::Parser;

/// Encrypt or decrypt a file or text from stdin
#[derive(Parser)]
#[command(author = "Neosb <neosb@nuteksecurity.com>", version, about = "Check files for changes using SHA256 hash function using user provided lists file\n\nABSOLUTE_PATH_TO_FILE1\nABSOLUTE_PATH_TO_FILE2", long_about = None, help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
", arg_required_else_help = true)]

pub struct Cli {
    /// List of files you want to check for changes
    /// and at the same time storage file for SHA256 hash values
    /// of files
    #[arg(short, long)]
    pub list_file: Option<String>,

    /// Create list of files in directory
    #[arg(short, long)]
    pub create_list: Option<String>,

    /// Exclude directories, especially useful when creating a list delimited by ',' - comma
    #[arg(long, value_delimiter = ',')]
    pub exclude_dirs: Option<Vec<String>>,

    /// Exclude files, especially useful when creating a list delimited by ',' - comma
    #[arg(long, value_delimiter = ',')]
    pub exclude_files: Option<Vec<String>>,

    /// Should it echo files and status as it checks
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,
}
