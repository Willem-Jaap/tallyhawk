/// Output format options for displaying results
#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}