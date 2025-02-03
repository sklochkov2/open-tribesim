use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run in web server mode (default - false)
    #[arg(short, long, default_value_t = false)]
    pub launch_server: bool,
}
