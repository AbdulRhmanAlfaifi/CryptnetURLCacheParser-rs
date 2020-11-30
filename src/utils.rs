use chrono::{DateTime, Utc, prelude::*};
use byteorder::ReadBytesExt;
use std::{char::decode_utf16, io, fs::File, io::BufReader, io::Read};
use data_encoding::HEXUPPER;
use ring::digest::{Context, SHA256};

/// Reads a FILETIME as `i64` and return ISO8601 formated string.
pub fn filetime_to_iso8601(timestamp:i64) -> String
{
    let epochtime = (timestamp - (11644473600000 * 10000)) / 10000000;
    let naive_datetime = NaiveDateTime::from_timestamp(epochtime, 0);
    let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    let iso8106 = datetime_again.format("%Y-%m-%dT%H:%M:%SZ");
    format!("{}",iso8106)
}

// modified version of the function here https://github.com/omerbenamram/mft/blob/master/src/utils.rs
/// Reads UTF-16LE string from a file and return it as `String`.
pub fn read_utf16_string(stream: &mut std::fs::File, len: usize) -> io::Result<String> 
{
    let mut buffer =  Vec::with_capacity(len);
    for _ in 0..len {
        let next_char = stream.read_u16::<byteorder::LittleEndian>()?;
        buffer.push(next_char);
    }
    decode_utf16(buffer.into_iter().take_while(|&byte| byte != 0x00))
        .map(|r| r.map_err(|_e| io::Error::from(io::ErrorKind::InvalidData)))
        .collect()
}

/// Calculates the `SHA256` hash for a file and return it as `String` in hex format.
pub fn sha256(filepath:&str) -> io::Result<String>
{
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(HEXUPPER.encode(context.finish().as_ref()))
}