/*
    This file is part of kilo-rs.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::io::Read;
use std::os::unix::io::{AsRawFd, RawFd};
use termios::Termios;

fn main() {
    let _restore_termios = enable_raw_mode(std::io::stdin());

    let mut c : [u8; 1] = [0; 1];
    while std::io::stdin().read_exact(&mut c).is_ok() {
        let char = char::from(c[0]);
        if char == 'q' {
            break;
        } else if char.is_control() {
            print!("0x{:02x}\r\n", c[0]);
        } else {
            print!("0x{:02x} ({1})\r\n", c[0], char);
        }
    };
}

fn enable_raw_mode<T: AsRawFd>(t : T) -> RestoreTermios {
    let raw_fd = t.as_raw_fd();
    let mut termios = Termios::from_fd(raw_fd).unwrap();
    let orig_termios = termios;
    /* Disable software control flow: Ctrl-S and Ctrl-Q */
    termios.c_iflag &= !termios::IXON;
    /* Disable CR translation: Ctrl-M */
    termios.c_iflag &= !termios::ICRNL;
    /* Disable terminal echo */
    termios.c_lflag &= !termios::ECHO;
    /* Disable canonical mode */
    termios.c_lflag &= !termios::ICANON;
    /* Disable signals: Ctrl-C and Ctrl-Z */
    termios.c_lflag &= !termios::ISIG;
    /* Disable verbatim input: Ctrl-V */
    termios.c_lflag &= !termios::IEXTEN;
    /* Disable output processing, particularly newline translation */
    termios.c_oflag &= !termios::OPOST;
    /* Do not send SIGINT for break conditions */
    termios.c_iflag &= !termios::BRKINT;
    /* Disable input parity checking */
    termios.c_iflag &= !termios::INPCK;
    /* Set the character size to 8-bit bytes */
    termios.c_iflag &= !termios::ISTRIP;
    termios.c_cflag |= termios::CS8;
    termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &termios).unwrap();
    RestoreTermios { orig_termios, raw_fd }
}

/*
    RestoreTermios captures the original Termios and RawFd before setting the
    terminal to raw mode; when RestoreTermios is drop()-ed, the original
    terminal attributes are restored.
 */
struct RestoreTermios {
    orig_termios: Termios,
    raw_fd: RawFd,
}

impl Drop for RestoreTermios {
    fn drop(&mut self) {
        termios::tcsetattr(self.raw_fd, termios::TCSAFLUSH, &self.orig_termios)
            .unwrap();
    }
}
