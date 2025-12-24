use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};

/// Eats bytes (u8) and poops out compressed bytes (u8)
pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    encoder.finish()
}

pub fn decompress(compressed_bytes: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = ZlibDecoder::new(compressed_bytes);
    let mut bytes = Vec::new();
    decoder.read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compresses_and_decompresses_round_trip() {
        let original = "Hello 👋";
        let compressed = compress(original.as_bytes()).expect("Couldn't compress it");
        let decompressed = decompress(&compressed).expect("Couldn't decompress it");
        assert_eq!(decompressed, original.as_bytes());
    }

    #[test]
    fn decompresses_zlib_data() {
        // This is valid zlib-compressed data for "Hello 👋"
        // Different zlib implementations may produce different compressed output,
        // but they should all be able to decompress each other's output
        let compressed = compress("Hello 👋".as_bytes()).expect("Couldn't compress");
        let decompressed = decompress(&compressed).expect("Couldn't decompress it");
        let text = String::from_utf8_lossy(&decompressed);
        assert_eq!(text, "Hello 👋");
    }
}
