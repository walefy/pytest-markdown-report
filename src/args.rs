use clap::{self, ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author = "Walefy", version = "0.0.1", about = "pytest markdown report", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "report.md")]
    pub output: String,

    #[arg(long, action = ArgAction::SetTrue)]
    /// Turn off auto run command pytest.
    ///
    /// Usage example:
    /// ```
    /// pytest -v | pytest_markdown_report --no-auto
    /// ```
    pub no_auto: bool,

    #[arg(short, long, default_value = "./")]
    /// directory where the tests are
    pub target_folder: String,

    #[arg(short, long, default_value = "")]
    /// set a config file
    ///
    /// usage example:
    /// ```
    /// pytest_markdown_report -c src/examples/config.txt
    /// ```
    pub config_file: String,
}
