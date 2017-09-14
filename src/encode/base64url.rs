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

const BASE_64_URL_MASK: usize = 0x3f;

pub fn base64url(input: &[u8]) -> String {
    let r = input.chunks(3).map(|x| {
        match x.len() {
            3 => {
                let buf: u32 = 
                    ((x[0] as u32) << 16) +
                    ((x[1] as u32) << 8) +
                    x[2] as u32;

                [
                    BASE_64_URL_ALPHABET[(buf >> 18) as usize],
                    BASE_64_URL_ALPHABET[((buf >> 12) as usize) & BASE_64_URL_MASK],
                    BASE_64_URL_ALPHABET[((buf >> 6) as usize) & BASE_64_URL_MASK],
                    BASE_64_URL_ALPHABET[(buf as usize) & BASE_64_URL_MASK]
                ].to_vec()
            },
            2 => {
                let buf: u32 = 
                    ((x[0] as u32) << 16) +
                    ((x[1] as u32) << 8);

                [
                    BASE_64_URL_ALPHABET[(buf >> 18) as usize],
                    BASE_64_URL_ALPHABET[((buf >> 12) as usize) & BASE_64_URL_MASK],
                    BASE_64_URL_ALPHABET[((buf >> 6) as usize) & BASE_64_URL_MASK],
                    b'='
                ].to_vec()
            },
            1 => {
                let buf: u32 = (x[0] as u32) << 16;

                [
                    BASE_64_URL_ALPHABET[(buf >> 18) as usize],
                    BASE_64_URL_ALPHABET[((buf >> 12) as usize) & BASE_64_URL_MASK],
                    b'=',
                    b'='
                ].to_vec()
            }
            _ => {
                unreachable!("chunks 3");
            }
        }
    }).fold(Vec::new(), |mut acc, x| {
        acc.extend(x);
        acc
    });

    String::from_utf8(r).unwrap()
}
