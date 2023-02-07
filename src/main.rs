use std::{net::{SocketAddr, TcpListener, TcpStream}, error::Error, io::{Read, Write}, fs::File};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SftnArgs {
    #[arg(short, long)]
    addr: SocketAddr,
    #[arg(short, long)]
    fname: String,
    #[arg(short, long, action, help = "The sending program must be ran before the receiving one.")]
    send: bool
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: SftnArgs = SftnArgs::parse();
    if args.send {
        println!("Creating TCP listener...");
        let listener: TcpListener = TcpListener::bind(args.addr)?;
        println!("TCP Listener created. Accepting next TCP connection.");
        let (mut stream, _addr): (TcpStream, SocketAddr) = listener.accept()?;
        println!("TCP stream opened.");
        let mut fstream: File = File::open(&args.fname).expect(&format!("File {} not accessible!", args.fname));
        println!("File stream opened.");
        
        let mut buf: [u8; 1024] = [0u8; 1024];
        let mut size: usize;
        
        println!("Sending file...");
        loop {
            size = fstream.read(&mut buf)?;
            if size == 0 { break; }
            stream.write(&buf[..size])?;
        }
        println!("File sent!");
        Ok(())
    } else {
        println!("Connecting to `{}`...", args.addr);
        let mut stream: TcpStream = TcpStream::connect(args.addr)?;
        let mut fstream: File = File::create(&args.fname).expect(&format!("Could not create file `{}`!", args.fname));

        let mut buf: [u8; 1024] = [0u8; 1024];
        let mut size: usize;

        loop {
            size = stream.read(&mut buf)?;
            if size == 0 { break; }
            fstream.write(&buf[..size])?;
        }

        Ok(())
    }
}
