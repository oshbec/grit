use libflate::zlib::{Decoder, Encoder};
use std::io::{self, Read};

/// Eats bytes (u8) and poops out compressed bytes (u8)
pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes: &[u8] = bytes;
    let mut encoder = Encoder::new(Vec::new())?;
    io::copy(&mut bytes, &mut encoder)?;
    Ok(encoder.finish().into_result()?)
}

pub fn decompress(compressed_bytes: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = Decoder::new(compressed_bytes)?;
    let mut bytes = Vec::new();
    decoder.read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compresses_some_data() {
        let some_text = "Hello 👋";
        let some_text_compressed = compress(&some_text.as_bytes().to_vec()).unwrap();
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
        let some_decompressed_text = decompress(&some_text_compressed).unwrap();
        let some_decompressed_text = some_decompressed_text.as_slice();
        let some_text = String::from_utf8_lossy(some_decompressed_text);
        assert_eq!(some_text, "Hello 👋".to_string());
    }
}
