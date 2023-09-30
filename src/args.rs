use clap::Parser;

#[derive(Parser, Default)]
#[clap(author, version, about)]
pub struct Arguments {
    /// File to move to trash
    pub file: String,
}
