use std::path::Iter;

/**
const CONT_MASK: u8 = 0b0011_1111;

Unicode符号范围      |        UTF-8编码方式
  (十六进制)         |         （二进制）
--------------------+---------------------------------------------
0000 0000-0000 007F | 0xxxxxxx
0000 0080-0000 07FF | 110xxxxx 10xxxxxx
0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
 */

const CONT_MASK: u32 = 0b0011_1111 as u32;

fn convert_bytes_to_unicode<T>(mut bytes: T) -> Option<u32>
where
    T: Iterator<Item = u8>,
{
    let a = bytes.next()? as u32;
    // len 1
    if a < 0x80 {
        let x = a as u32;
        return Some(x);
    }
    // len 2
    if a < 0xE0 {
        let b = bytes.next()? as u32;
        let x = ((a & 0b001_11111) << 6) | (b & CONT_MASK);
        return Some(x);
    }
    // len 3
    if a < 0xF0 {
        let b = bytes.next()? as u32;
        let c = bytes.next()? as u32;
        let x = (((a & 0b0001_1111) << 12) | (b & CONT_MASK) << 6 | (c & CONT_MASK)) as u32;
        return Some(x);
    }
    // len 4
    let b = bytes.next()? as u32;
    let c = bytes.next()? as u32;
    let d = bytes.next()? as u32;

    let x = (((a & 0b00001_111) << 18)
        | (b & CONT_MASK) << 12
        | (c & CONT_MASK) << 6
        | (d & CONT_MASK)) as u32;

    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p() {
        println!("len1 {} {:x}", 0b10000000, 0b10000000); //128 0x80
        println!("len2 {} {:x}", 0b11100000, 0b11100000); //224 0xe0
        println!("len3 {} {:x}", 0b11110000, 0b11110000); //240 0xf0
        println!("len4 {} {:x}", 0b11111000, 0b11111000); //248 0xf8
        println!("len4 {} {:x}", 0b11111111, 0b11111111); //255 0xff
    }

    #[test]
    fn pp() {
        let a = "\u{2764}";
        println!("\"{}\"", a);
    }

    #[test]
    fn test_utf8() {
        // let a = "abcd";
        let a = "ɖ";
        let a = "नमस्ते";

        let c = convert_bytes_to_unicode(a.bytes()).unwrap();
        let ch = unsafe { char::from_u32_unchecked(c) };
        println!("char: {} {}", ch, c);
    }
}
