use color_eyre::eyre::Result;
use structopt::StructOpt;

/// A CLI for the growing and curation of a digital garden
/// 
/// Visit https://egghead.io/courses/creating-a-digital-garden-cli-with-rust-34b8 for more!
#[derive(StructOpt, Debug)]
#[structopt(name = "dg")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Write something in your garden
    /// 
    /// This command will open you $EDITOR, wait for you
    /// to write something, and then save the file to your
    /// garden
    Write {
        /// Optionally set a title for what you are going to write about
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(opt);
    todo!();
}
