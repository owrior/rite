use std::{io, io::Read, os::fd::{AsRawFd, RawFd}};
use anyhow::Error;
use termios::{self, Termios, tcsetattr, tcgetattr};


fn main() -> Result<(), Error>{
    let stdin = io::stdin();
    let handle = stdin.lock();
    let raw_fd = handle.as_raw_fd();
    let mut term = Termios::from_fd(raw_fd)?;

    raw_mode_on(raw_fd, &mut term)?;

    for i in handle.bytes() {
        let i = i?;

        if i == b'q' {
            break
        }

        println!("{}", i);
    }
    
    Ok(())
}

fn raw_mode_on(raw_fd: RawFd, term: &mut Termios) -> Result<(), Error> {
    tcgetattr(raw_fd, term)?;

    term.c_lflag &= !(termios::ECHO);

    tcsetattr(raw_fd, termios::TCSAFLUSH, term)?;

    Ok(())
}