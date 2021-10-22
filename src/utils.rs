use data_encoding::HEXUPPER;
use ring::digest::{Context, SHA256};
use std::{fs::File, io, io::BufReader, io::Read};

/// Calculates the `SHA256` hash for a file and return it as `String` in hex format.
pub fn sha256(filepath: &str) -> io::Result<String> {
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
