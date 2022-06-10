use std::str::FromStr;

use clap::Parser;
use expectrl::{
    process::unix::{PtyStream, UnixProcess},
    spawn, Error, Session,
};
use itertools::Itertools;

#[derive(Debug)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return Err(String::from("invalid hex length"));
        }

        let a: Result<Vec<u8>, _> = s
            .chars()
            .chunks(2)
            .into_iter()
            .map(|mut n| u8::from_str_radix(&n.join(""), 16))
            .collect();

        let b = a.map_err(|_| String::from("invalid hex"))?;

        Ok(Rgb {
            r: b[0],
            g: b[1],
            b: b[2],
        })
    }
}

impl ToString for Rgb {
    fn to_string(&self) -> String {
        format!("0x{:x} 0x{:x} 0x{:x}",self.r,self.g,self.b)
    }
} 

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// mac address to write to
    #[clap(short, long)]
    mac: String,
    /// mac address to write to
    #[clap(short, long)]
    setting: String,
    /// value to write
    #[clap(short, long)]
    value: Option<Rgb>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mac = args.mac;
    let val = args.value;
    let setting= args.setting;
    let mut kitty = spawn(format!("btgatt-client -d {mac}"))?;

    kitty.expect("GATT discovery procedures complete")?;
    
    match &*setting {
        // Match a single value
        "rainbow" => send_rainbow(kitty),
        "beat" => send_beat(kitty, val.unwrap()),
        "color" => send_static_rgb(kitty, val.unwrap()),
        _ => println!("Please provide a proper mode")
    }

    Ok(())
}

fn send_static_rgb(mut ses: Session<UnixProcess, PtyStream>, rgb: Rgb) {
    dbg!(rgb.to_string());
    let command = format!(
        "write-value 0x000d 0xc3 0x00 0x03 {val}",
        val = rgb.to_string()
    );
    ses.send_line(command).unwrap();
    ses.expect("Write").unwrap();
}   

fn send_rainbow(mut ses: Session<UnixProcess, PtyStream>) {
    let command = "write-value 0x000d 0xc0 0x00 0x01 0x01";
    ses.send_line(command).unwrap();
    ses.expect("Write").unwrap();
}

fn send_beat(mut ses: Session<UnixProcess, PtyStream>, rgb: Rgb) {
    dbg!(rgb.to_string());
    let command = format!(
        "write-value 0x000d 0xc5 0x00 0x03 {val}",
        val = rgb.to_string()
    );
    ses.send_line(command).unwrap();
    ses.expect("Write").unwrap();
}
