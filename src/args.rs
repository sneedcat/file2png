use argh::FromArgs;

#[derive(FromArgs)]
/// Arguments of the program
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Command {
    /// Store
    Store(Store),
    /// Restore
    Restore(Restore),
    /// Info
    Info(Info),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Store file into png
#[argh(subcommand, name = "store")]
pub struct Store {
    #[argh(positional)]
    pub input: String,
    #[argh(positional)]
    pub output: String,
    /// comments for the output file
    #[argh(option)]
    pub comments: Vec<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Restore png
#[argh(subcommand, name = "restore")]
pub struct Restore {
    #[argh(positional)]
    pub input: String,
    /// output for Restore (optional)
    #[argh(option)]
    pub output: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Info from png
#[argh(subcommand, name = "info")]
pub struct Info {
    #[argh(positional)]
    pub input: String,
}
