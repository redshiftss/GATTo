use clap::Parser;
use itertools::Itertools;
use expectrl::{spawn, Error, process::unix::{UnixProcess, PtyStream}, Session};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// mac address to write to
    #[clap(short, long)]
    mac: String,
    /// value to write
    #[clap(short, long)]
    value: String
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    let args = Args::parse();
    let mac = args.mac;
    let mut kitty = spawn(format!("btgatt-client -d {mac}"))?;
    
    let val = args.value.chars().chunks(2).into_iter().map(|c| c.collect::<String>()).collect::<Vec<String>>();
    
    kitty.expect("GATT discovery procedures complete")?;
    send_static_rgb(kitty, val);

    Ok(())
}

fn send_static_rgb(mut ses : Session<UnixProcess, PtyStream>, rgb : Vec<String>) {
    let command = format!("write-value 0x000d 0xc3 0x00 0x03 0x{v1} 0x{v2} 0x{v3}", v1=rgb[0], v2=rgb[1], v3=rgb[2]);
    ses.send_line(command).unwrap();
    ses.expect("Write").unwrap();
}
