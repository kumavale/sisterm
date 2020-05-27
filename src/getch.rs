
#[cfg(windows)]
use libc::c_int;
#[cfg(windows)]
extern "C" {
    fn _getch() -> c_int;
}

#[cfg(not(windows))]
use nix::sys::termios;
#[cfg(not(windows))]
use std::io::Read;


#[cfg(windows)]
pub struct Getch {}

#[cfg(not(windows))]
pub struct Getch {
    orig_term: termios::Termios,
}


impl Getch {
    #[cfg(windows)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
    #[cfg(not(windows))]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Quering original as a separate, since `Termios` does not implement copy
        let orig_term       = termios::tcgetattr(0).unwrap();
        let mut raw_termios = termios::tcgetattr(0).unwrap();

        // Unset canonical mode, so we get characters immediately
        raw_termios.local_flags.remove(termios::LocalFlags::ICANON);
        // Don't generate signals on Ctrl-C and friends
        raw_termios.local_flags.remove(termios::LocalFlags::ISIG);
        // Disable local echo
        raw_termios.local_flags.remove(termios::LocalFlags::ECHO);

        termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &raw_termios).unwrap();

        Self {
            orig_term
        }
    }

    #[cfg(windows)]
    pub fn getch(&self) -> Result<u8, std::io::Error> {
        loop {
            unsafe {
                let key = _getch();
                if key == 0 {
                    // Ignore next input
                    _getch();
                } else {
                    return Ok(key as u8)
                }
            }
        }
    }
    #[cfg(not(windows))]
    pub fn getch(&self) -> Result<u8, std::io::Error> {
        let mut r: [u8; 1] = [0];

        if let Err(e) = std::io::stdin().read(&mut r[..]) {
            Err(e)
        } else {
            Ok(r[0])
        }
    }
}

impl Drop for Getch {
    #[cfg(windows)]
    fn drop(&mut self) {}

    #[cfg(not(windows))]
    fn drop(&mut self) {
        termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &self.orig_term).unwrap();
    }

}
