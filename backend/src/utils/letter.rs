use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockDecrypt;
use aes::cipher::BlockEncrypt;
use aes::cipher::KeyInit;

use aes::Aes128;

use anyhow::anyhow;
use anyhow::Result;

use flate2::write::ZlibDecoder;
use flate2::{write::ZlibEncoder, Compression};

use std::io::Write;
use std::time::Instant;

use crate::vec_to_sized;

fn defined_output(n: usize) -> String {
    let n_str = n.to_string();
    let n_len = n_str.len();
    let gap = usize::MAX.to_string().len() - n_len;
    let mut output = String::new();
    for _ in 0..gap {
        output.push('0');
    }
    output.push_str(&n_str);
    output
}

pub async fn decrypt_message(key: &[u8; 16], author: &str, message: &str) -> Result<String> {
    log::info!("[decrypt] decompressing message");

    let (bytes_len, message) = message.split_at(usize::MAX.to_string().len());
    let bytes_len = bytes_len.parse::<usize>().unwrap_or_default();

    let now = Instant::now();
    let message = base64::decode(message).map_err(|e| anyhow!("decoding Base64 failed: {}", e))?;
    let elapsed = now.elapsed();

    log::info!("[decrypt] Base64 decoding done ({:.2?})", elapsed);
    log::info!("[decrypt] decrypting AES encrypted data");
    let now = Instant::now();
    let key = GenericArray::from(key.clone());
    let cipher = Aes128::new(&key);

    let mut bytes = Vec::new();

    let sections = crate::utils::slice::split_into_sections(&message, 16);
    for section in sections {
        let mut sized = [0u8; 16];
        vec_to_sized!(section, sized);

        let mut block = GenericArray::from(sized);
        cipher.decrypt_block(&mut block);
        bytes.extend(block.iter());
    }

    let elapsed = now.elapsed();
    log::info!("[decrypt] AES decryption done ({:.2?})", elapsed);

    bytes.resize(bytes_len, 0);

    log::info!("[decrypt] decompressing zlib compressed data");

    let mut decoded = Vec::new();
    let mut decoder = ZlibDecoder::new(decoded);

    decoder
        .write_all(&bytes[..])
        .map_err(|e| anyhow!("zlib decompression error: {}", e))?;

    decoded = decoder.finish()?;

    // convert back to string
    let raw_text = String::from_utf8_lossy(&decoded);

    // get rid of the prefix of the author thing
    if !raw_text.starts_with(author) {
        return Err(anyhow!("invalid header"));
    }

    Ok(raw_text.strip_prefix(author).unwrap().to_string())
}

pub async fn encrypt_message(key: &[u8; 16], author: &str, message: &str) -> Result<String> {
    log::info!("[encrypt] compressing message");

    let mut bytes = Vec::new();
    let mut encoder = ZlibEncoder::new(bytes, Compression::fast());

    let now = Instant::now();
    encoder
        .write_all(format!("{}{}", author, message).as_bytes())
        .map_err(|e| anyhow!("zlib error: {}", e))?;

    bytes = encoder.finish()?;

    let elapsed = now.elapsed();
    log::info!("[encrypt] zlib compression done ({:.2?})", elapsed);

    let mut encrypted: Vec<u8> = Vec::new();

    log::info!("[encrypt] encrypting zlib compressed data to AES");
    let now = Instant::now();
    let key = GenericArray::from(key.clone());
    let cipher = Aes128::new(&key);

    let bytes_len = defined_output(bytes.len());
    let sections = crate::utils::slice::split_into_sections(&bytes, 16);

    for section in sections {
        let mut sized = [0u8; 16];
        vec_to_sized!(section, sized);

        let mut block = GenericArray::from(sized);
        cipher.encrypt_block(&mut block);
        encrypted.extend(block.iter());
    }

    let elapsed = now.elapsed();
    log::info!("[encrypt] AES encryption done ({:.2?})", elapsed);

    log::info!("[encrypt] converting AES data to Base64");
    let now = Instant::now();
    let value = base64::encode(encrypted);
    let elapsed = now.elapsed();

    log::info!("[encrypt] Base64 conversion done ({:.2?})", elapsed);
    Ok(format!("{}{}", bytes_len, value))
}
