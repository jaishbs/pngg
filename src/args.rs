use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pngme")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, StructOpt)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, StructOpt)]
pub struct EncodeArgs {
    pub chunk_type: String,
    pub message: String,
}

#[derive(Debug, StructOpt)]
pub struct DecodeArgs {
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {}