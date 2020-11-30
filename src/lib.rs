mod utils;
use std::{
    io,
    io::prelude::*,
    fs::File,
    io::SeekFrom,
    path::PathBuf
};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Debug)]
/// Represents a CryptnetURLCache metadata file.
pub struct CryptnetURLCacheParser
{
    #[serde(skip_serializing)]
    url_size: u32,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(skip_serializing)]
    hash_size: u32,
    #[serde(rename = "FileSize")]
    pub file_size: u32,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "MetadataHash")]
    pub metadata_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SHA256")]
    pub sha256: Option<String>,
    #[serde(rename = "FullPath")]
    pub full_path: String
}

impl Default for CryptnetURLCacheParser {
    fn default () -> CryptnetURLCacheParser {
        CryptnetURLCacheParser {
            url_size: 0,
            timestamp: String::new(),
            hash_size: 0,
            file_size: 0,
            url: String::new(),
            metadata_hash: String::new(),
            sha256: None,
            full_path: String::new()
        }
    }
}

impl CryptnetURLCacheParser
{
    /// Read a file and parse all values then returns `CryptnetURLCacheParser` struct if successfull.
    /// To read more about CryptnetURLCache metadata file strcuture you can refer to my blog `https://u0041.co/blog/post/3`
    pub fn parse_file(filepath:&str, use_content: Option<bool>) -> Result<CryptnetURLCacheParser,io::Error>
    {
        let use_content = match use_content {
            Some(true) => true,
            Some(false) => false,
            _ => false
        };
        let fullpath = PathBuf::from(filepath).canonicalize().unwrap();
        let parent = fullpath.as_path().parent().unwrap().parent().unwrap();
        let contentpath = parent.join("Content").join(fullpath.as_path().file_name().unwrap());
        let f = File::open(fullpath.clone());
        let mut f = match f {
            Ok(file) => file,
            Err(_) => panic!("File not Found !")
        };

        f.seek(SeekFrom::Start(12))?;
        let mut x = CryptnetURLCacheParser::default();
        x.full_path = fullpath.as_path().to_str().unwrap().to_string().replace("\\\\?\\","");
        x.url_size = f.read_u32::<LittleEndian>()?;
        x.timestamp = utils::filetime_to_iso8601(f.read_i64::<LittleEndian>()?);
        f.seek(SeekFrom::Current(76))?;
        x.hash_size = f.read_u32::<LittleEndian>()?;
        f.seek(SeekFrom::Current(8))?;
        x.file_size = f.read_u32::<LittleEndian>()?;
        x.url = match utils::read_utf16_string(&mut f,(x.url_size / 2) as usize) {
            Ok(url) => url,
            Err(_) => String::from("No URL Found !")
        };
        x.metadata_hash = match utils::read_utf16_string(&mut f,(x.hash_size / 2) as usize) {
            Ok(metadata_hash) => metadata_hash.replace("\"",""),
            Err(_) => String::from("No Hash Found !")
        };
        if use_content{
            let hash = utils::sha256(contentpath.to_str().unwrap());
            x.sha256 = match hash {
                Ok(hash) => Some(hash),
                Err(_) => Some(String::from(""))
            };
        }
        Ok(x)
    }
}