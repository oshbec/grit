use libflate::zlib::{Decoder, Encoder};
use std::io::{self, Read};

pub fn compress(bytes: &Vec<u8>) -> Vec<u8> {
    let mut bytes: &[u8] = bytes.as_ref();
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    io::copy(&mut bytes, &mut encoder).unwrap();
    encoder
        .finish()
        .into_result()
        .expect("Couldn't compress the data")
}

fn _decompress(compressed_bytes: &Vec<u8>) -> Vec<u8> {
    let compressed_bytes: &[u8] = compressed_bytes;
    let mut decoder = Decoder::new(compressed_bytes).unwrap();
    let mut bytes = Vec::new();
    decoder.read_to_end(&mut bytes).unwrap();
    bytes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compresses_some_data() {
        let some_text = "Hello 👋";
        let some_text_compressed = compress(&some_text.as_bytes().to_vec());
        assert_eq!(
            some_text_compressed,
            vec![
                120, 156, 5, 192, 49, 17, 0, 0, 8, 2, 192, 42, 198, 177, 136, 27, 119, 6, 97, 34,
                1, 21, 137, 192, 239, 1, 63, 177, 88, 23, 63, 4, 192
            ]
        );
    }

    #[test]
    fn decompresses_some_data() {
        let some_text_compressed = vec![
            120, 156, 5, 192, 49, 17, 0, 0, 8, 2, 192, 42, 198, 177, 136, 27, 119, 6, 97, 34, 1,
            21, 137, 192, 239, 1, 63, 177, 88, 23, 63, 4, 192,
        ];
        let some_text = String::from_utf8(_decompress(&some_text_compressed)).unwrap();
        assert_eq!(some_text, "Hello 👋".to_string());
    }
}
