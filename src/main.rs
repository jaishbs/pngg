mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use structopt::StructOpt;
fn main() -> Result<()> {
    let opt = args::Opt::from_args();

    match opt {
        args::Opt {
            input,
            commands: args::Commands::Encode(args),
        } => commands::encode(input, args)?,
        args::Opt {
            input,
            commands: args::Commands::Decode(args),
        } => commands::decode(input, args)?,
        args::Opt {
            input,
            commands: args::Commands::Remove(args),
        } => commands::remove(input, args)?,
        args::Opt {
            input,
            commands: args::Commands::Print(_),
        } => commands::print(&input)?,
    }
    Ok(())
}