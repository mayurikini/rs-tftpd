use std::error::Error;
use std::{env, net::SocketAddr, process};
use tftpd::{Client, ClientConfig, Mode};

fn main() {
    client(env::args()).unwrap_or_else(|err| {
            eprintln!("{err}");
    })
}

fn client<T: Iterator<Item = String>>(args: T) -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    let mut client = Client::new(&config).unwrap_or_else(|err| {
        eprintln!("Problem creating client: {err}");
        process::exit(1)
    });

    if config.mode == Mode::Upload {
        println!(
            "Starting TFTP Client, uploading {} to {}",
            config.filename.display(),
            SocketAddr::new(config.remote_ip_address, config.port),
        );
    } else {
        println!(
            "Starting TFTP Client, downloading {} to {}",
            config.filename.display(),
            SocketAddr::new(config.remote_ip_address, config.port),
        );
    }

    client.run()
}
