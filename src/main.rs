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
use std::os::unix::io::AsRawFd;
use termios::Termios;

fn main() {
    enable_raw_mode(std::io::stdin());

    let mut c : [u8; 1] = [0; 1];
    while std::io::stdin().read_exact(&mut c).is_ok() {
        if char::from(c[0]) == 'q' {
            break;
        }
    };
}

fn enable_raw_mode<T: AsRawFd>(t : T) {
    let fd = t.as_raw_fd();
    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag &= !termios::ECHO;
    termios::tcsetattr(fd, termios::TCSAFLUSH, &termios).unwrap();
}
