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

const BASE_64_MASK: u8 = 0x3F;

pub fn base64(input: &str) -> Vec<u8> {
    input.as_bytes().chunks(4).map(|x| {
        if x[2] == b'=' && x[3] == b'=' {
            let buf: u32 = 
                (((x[0] & BASE_64_MASK) as u32) << 18) +
                (((x[1] & BASE_64_MASK) as u32) << 12);

            vec![
                (buf >> 16) as u8
            ]
        }
        else if x[3] == b'=' {
            let buf: u32 = 
                (((x[0] & BASE_64_MASK) as u32) << 18) +
                (((x[1] & BASE_64_MASK) as u32) << 12) +
                (((x[2] & BASE_64_MASK) as u32) << 6);

            vec![
                (buf >> 16) as u8,
                (buf >> 8) as u8
            ]
        }
        else {
            let buf: u32 = 
                (((x[0] & BASE_64_MASK) as u32) << 18) +
                (((x[1] & BASE_64_MASK) as u32) << 12) +
                (((x[2] & BASE_64_MASK) as u32) << 6) +
                ((x[3] & BASE_64_MASK) as u32);

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
