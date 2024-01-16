

use std::io;
use std::time::Duration;
// use std::str::String;

use serial::prelude::*;

fn main() {
    let serial_port = "/dev/tty.usbmodemF412FA649B242";

    let mut port = serial::open(serial_port).unwrap();
    interact(&mut port).unwrap();

    //for arg in env::args_os().skip(1) {
        //let mut port = serial::open(&arg).unwrap();
        //interact(&mut port).unwrap();
    //}
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud19200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(3000))?;

    // prompt for an input
    let buf: Vec<u8> = b"I command you!\n".to_vec();
    port.write(&buf[..])?;


    let bufsize = 64;
    loop {
        let mut sbuf: Vec<u8> = vec![0; bufsize];
        let n = port.read(&mut sbuf)?;

        let s = String::from_utf8(sbuf).unwrap();
        print!("n: {} -> {}", n, s);

        if n < bufsize {
            break;
        }
    }

    Ok(())
}
