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

pub mod encode;
pub mod decode;

#[cfg(test)]
mod tests {
    use super::encode;
    // use super::decode;

    #[test]
    fn base_64_section_9_example_1() {
        let result = encode::base64(&[0x14, 0xfb, 0x9c, 0x3, 0xd9, 0x7e]);

        assert_eq!(String::from("FPucA9l+"), result);

        // assert_eq!(&[0x14, 0xfb, 0x9c, 0x3, 0xd9, 0x7e], decode::base64(result.as_str()).as_slice());
    }

    #[test]
    fn base_64_section_9_example_2() {
        let result = encode::base64(&[0x14, 0xfb, 0x9c, 0x3, 0xd9]);

        assert_eq!(String::from("FPucA9k="), result);
    }

    #[test]
    fn base_64_section_9_example_3() {
        let result = encode::base64(&[0x14, 0xfb, 0x9c, 0x3]);

        assert_eq!(String::from("FPucAw=="), result);
    }

    #[test]
    fn base_64_section_10_example_1() {
        let result = encode::base64(String::from("").as_bytes());

        assert_eq!(String::from(""), result);
    }

    #[test]
    fn base_64_section_10_example_2() {
        let result = encode::base64(String::from("f").as_bytes());

        assert_eq!(String::from("Zg=="), result);
    }

    #[test]
    fn base_64_section_10_example_3() {
        let result = encode::base64(String::from("fo").as_bytes());

        assert_eq!(String::from("Zm8="), result);
    }

    #[test]
    fn base_64_section_10_example_4() {
        let result = encode::base64(String::from("foo").as_bytes());

        assert_eq!(String::from("Zm9v"), result);
    }

    #[test]
    fn base_64_section_10_example_5() {
        let result = encode::base64(String::from("foob").as_bytes());

        assert_eq!(String::from("Zm9vYg=="), result);
    }

    #[test]
    fn base_64_section_10_example_6() {
        let result = encode::base64(String::from("fooba").as_bytes());

        assert_eq!(String::from("Zm9vYmE="), result);
    }

    #[test]
    fn base_64_section_10_example_7() {
        let result = encode::base64(String::from("foobar").as_bytes());

        assert_eq!(String::from("Zm9vYmFy"), result);
    }
}
