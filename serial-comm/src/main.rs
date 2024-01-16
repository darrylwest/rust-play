

use std::io;
use std::io::Write;
use std::time::Duration;
use std::env;

use serial::prelude::*;

fn main() {
    // let serial_port = "/dev/tty.usbmodemF412FA649B242";
    // let mut port = serial::open(serial_port).unwrap();
    // interact(&mut port).unwrap();

    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap();
    }
}

fn prompt(prefix: &str) -> String {
    let mut line = String::new();

    print!("{}", prefix);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).expect("error: bad line");

    line.to_string()
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

    port.set_timeout(Duration::from_millis(1000))?;

    // prompt for an input
    loop {
    
        let sin = prompt("> ");
        if sin.starts_with("quit") {
            break;
        }

        let buf = sin.as_bytes();
        port.write(&buf[..])?;

        let bufsize = 512;
        loop {
            let mut sbuf: Vec<u8> = vec![0; bufsize];
            let n = port.read(&mut sbuf)?;

            let s = String::from_utf8(sbuf).unwrap();
            print!("{}", s);

            if n < bufsize {
                break;
            }
        }
    }

    Ok(())
}
