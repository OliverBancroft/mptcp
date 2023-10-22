use clap::Parser;
use cli::FileTransferCommand;
use mptcp::{receiver::Receiver, sender::Sender};
use tokio::net::TcpStream;

#[derive(Debug, Parser)]
pub struct Cli {
    /// The amount of streams to connect
    pub streams: usize,
    /// The server address
    pub server: String,
    #[command(subcommand)]
    pub file_transfer: FileTransferCommand,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let mut write_streams = vec![];
    let mut read_streams = vec![];
    for _ in 0..args.streams {
        let stream = TcpStream::connect(&args.server).await.unwrap();
        let (read, write) = stream.into_split();
        write_streams.push(write);
        read_streams.push(read);
    }

    let async_write = Sender::new(write_streams).into_async_write();
    let async_read = Receiver::new(read_streams).into_async_read();

    let n = args
        .file_transfer
        .perform(async_read, async_write)
        .await
        .unwrap();
    match &args.file_transfer {
        FileTransferCommand::Push(_) => println!("Read {n} bytes"),
        FileTransferCommand::Pull(_) => println!("Wrote {n} bytes"),
    }
}