// Copyright 2017 ThetaSinner
//
// This file is part of base-encoding.
//
// base-encoding is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// base-encoding is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with base-encoding. If not, see <http://www.gnu.org/licenses/>.

const BASE_64_URL_ALPHABET: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'-', b'_'
];

const BASE_64_URL_MASK: u8 = 0x3F;

pub fn base64url(input: &str) -> Vec<u8> {
    input.as_bytes().chunks(4).map(|x| {
        if x[2] == b'=' && x[3] == b'=' {
            let buf: u32 = 
                (((lookup(x[0]).unwrap() & BASE_64_URL_MASK) as u32) << 18) +
                (((lookup(x[1]).unwrap() & BASE_64_URL_MASK) as u32) << 12);

            vec![
                (buf >> 16) as u8
            ]
        }
        else if x[3] == b'=' {
            let buf: u32 = 
                (((lookup(x[0]).unwrap() & BASE_64_URL_MASK) as u32) << 18) +
                (((lookup(x[1]).unwrap() & BASE_64_URL_MASK) as u32) << 12) +
                (((lookup(x[2]).unwrap() & BASE_64_URL_MASK) as u32) << 6);

            vec![
                (buf >> 16) as u8,
                (buf >> 8) as u8
            ]
        }
        else {
            let buf: u32 = 
                (((lookup(x[0]).unwrap() & BASE_64_URL_MASK) as u32) << 18) +
                (((lookup(x[1]).unwrap() & BASE_64_URL_MASK) as u32) << 12) +
                (((lookup(x[2]).unwrap() & BASE_64_URL_MASK) as u32) << 6) +
                ((lookup(x[3]).unwrap() & BASE_64_URL_MASK) as u32);

            vec![
                (buf >> 16) as u8,
                (buf >> 8) as u8,
                buf as u8
            ]
        }
    }).fold(Vec::new(), |mut acc, x| {
        acc.extend(x);
        acc
    })
}

fn lookup(b: u8) -> Result<u8, ()> {
    for (i, byte) in BASE_64_URL_ALPHABET.iter().enumerate() {
        if byte == &b {
            return Ok(i as u8)
        }
    }

    Err(())
}
