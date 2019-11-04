//! Irrigatron.


use async_std::io;
use async_std::prelude::*;
use async_std::task;


use structopt::*;

/// The various kinds of commands that `wasm-pack` can execute.
#[derive(Debug, StructOpt)]
pub enum Command {
   ///  📈‍♀ run this command to confirm that your configuration is appropriately set up.
    #[structopt(name = "check", about = "check current sensor")]
    Check,
   ///  ⥄‍♀ run pipe mod.
    #[structopt(name = "pipe", about = "run in pipe mod")]
    Pipe,
}

/// 🔧 Output JSON instead of human readable messages
    // #[structopt(name = "json", long = "json")]
    // json: bool,



/// 🧰  Irrigatron
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// 📪  Address
    #[structopt(name = "workdir", long = "address", default_value = ".")]
    address: String,
    /// 📪  linux socket file or ip addresse
    #[structopt(name = "port", long = "port")]
    port: u64,
     /// The subcommand to run.
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: Command,
 
}



fn main() -> io::Result<()> {
    
    femme::pretty::Logger::new().start(log::LevelFilter::Trace).unwrap();
    task::block_on(async {
        // irrigatron::server::server().await;
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut line = String::new();

        loop {
            // Read a line from stdin.
            let n = stdin.read_line(&mut line).await?;

            // If this is the end of stdin, return.
            if n == 0 {
                return Ok(());
            }

            // Write the line to stdout.
            stdout.write_all(line.as_bytes()).await?;
            stdout.flush().await?;
            line.clear();
        }
    })
}
