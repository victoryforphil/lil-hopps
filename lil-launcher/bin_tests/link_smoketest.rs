use lil_link::mavlink::core::QuadLinkCore;
use tracing::error;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SmoketestArgs {
    #[clap(short, long, value_parser, help = "Mavlink connection string")]
    connection_string: String,
}

fn main() {
    fmt()
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();

    let args = SmoketestArgs::parse();
    info!("Running 'link_smoketest' with args: {:#?}", args);

    let mut quadlink = QuadLinkCore::new(args.connection_string.as_str()).unwrap();

    quadlink.start_thread().unwrap();

    loop {
        let msgs = quadlink.recv();
        match msgs {
            Ok(msgs) => {
                for msg in msgs {
                    info!("Message: {:#?}", msg);
                }
            }
            Err(e) => {
                error!("Error receiving messages: {}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }
}
