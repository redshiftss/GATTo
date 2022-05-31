use std::io::Read;

use expectrl::{spawn, Regex, Eof, WaitStatus, Error, process::unix::{UnixProcess, PtyStream}, Session};

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mac = "00:14:41:30:6B:28";
    let mut kitty = spawn(format!("btgatt-client -d {mac}"))?;

    kitty.expect("GATT discovery procedures complete")?;
    send_static_rgb(kitty, ["ff", "00", "ff"]);

    Ok(())
}

fn send_static_rgb(mut ses : Session<UnixProcess, PtyStream>, rgb : [&str; 3]) {
    let command = format!("write-value 0x000d 0xc3 0x00 0x03 0x{v1} 0x{v2} 0x{v3}", v1=rgb[0], v2=rgb[1], v3=rgb[2]);
    ses.send_line(command).unwrap();
    ses.expect("Write").unwrap();
}
