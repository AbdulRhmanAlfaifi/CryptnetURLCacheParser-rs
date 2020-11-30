use cryptnet_url_cache_parser::CryptnetURLCacheParser;
use clap::{App, Arg};
use std::io::{self, Write};
use std::fs::File;
use glob::glob;

enum OutputFormat {
    JSONL,
    CSV
}

impl OutputFormat {
    pub fn from_str(s: &str) -> OutputFormat {
        match s {
            "jsonl" => OutputFormat::JSONL,
            "csv" => OutputFormat::CSV,
            _ => OutputFormat::CSV
        }
    }
}

fn parse_cli_args() -> clap::ArgMatches<'static>
{
    App::new("CryptnetURLCacheParser")
        .version(env!("CARGO_PKG_VERSION"))
        .author("AbdulRhman Alfaifi - @A__ALFAIFI")
        .about("CryptnetURLCache metadata files parser")
        .arg(Arg::with_name("PATH")
                .short("-p")
                .long("--path")
                .takes_value(true)
                .multiple(true)
                .help("Path(s) to CryptnetURLCache Metadata Files to be Parsed - accepts glob (Defaults to all windows paths)"))
        .arg(
            Arg::with_name("output")
                .short("-o")
                .long("--output")
                .default_value("stdout")
                .takes_value(true)
                .help("The file path to write the output to"))
        .arg(
            Arg::with_name("output-format")
            .long("--output-format")
            .takes_value(true)
            .possible_values(&["csv", "jsonl"])
            .default_value("csv")
            .help("Output format."))
        .arg(
            Arg::with_name("use-content")
                .long("--use-content")
                .takes_value(false)
                .help("Try finding the cached file and calculate the SHA256 hash for it"))
        .arg(
            Arg::with_name("no-headers")
                .long("--no-headers")
                .takes_value(false)
                .help("Don't print headers when using CSV as the output format"))
        .get_matches()
}


fn output_data_csv(parsed_data: &CryptnetURLCacheParser) -> io::Result<String>
{
    if parsed_data.sha256.is_none(){
        return Ok(format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",parsed_data.timestamp,parsed_data.url,parsed_data.file_size,parsed_data.metadata_hash,parsed_data.full_path))
    }
    else{
        return Ok(format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",parsed_data.timestamp,parsed_data.url,parsed_data.file_size,parsed_data.metadata_hash,parsed_data.full_path,parsed_data.sha256.as_ref().unwrap()))
    }
}

fn main()
{
    let args = parse_cli_args();
    let output_format = OutputFormat::from_str(args.value_of("output-format").unwrap());
    let output_to = args.value_of("output").unwrap();
    let use_content = match args.occurrences_of("use-content"){
        0 => Some(false),
        _ => Some(true)
    };
    let mut output: Box<dyn Write> = match output_to {
        "stdout" => Box::new(io::stdout()),
        _ => Box::new(File::create(output_to).unwrap())
    };

    if args.occurrences_of("no-headers") == 0{
        match output_format {
            OutputFormat::CSV => {
                if use_content == Some(true){
                    output.write("\"Timestamp\",\"URL\",\"FileSize (Bytes)\",\"MetadataHash\",\"FullPath\",\"SHA256\"\r\n".as_bytes()).expect("Error Writing Data !");
                }
                else{
                    output.write("\"Timestamp\",\"URL\",\"FileSize (Bytes)\",\"MetadataHash\",\"FullPath\"\r\n".as_bytes()).expect("Error Writing Data !");
                }
                ()
            },
            _ => ()
        };
    }
    // Default windows paths for CryptnetURLCache metadata/content files
    let mut certutil_cache_paths = vec![
        "C:\\Windows\\System32\\config\\systemprofile\\AppData\\LocalLow\\Microsoft\\CryptnetUrlCache\\Metadata\\*",
        "C:\\Windows\\SysWOW64\\config\\systemprofile\\AppData\\LocalLow\\Microsoft\\CryptnetUrlCache\\Metadata\\*",
        "C:\\Users\\*\\AppData\\LocalLow\\Microsoft\\CryptnetUrlCache\\MetaData\\*"
    ];
    if args.occurrences_of("PATH") > 0 {
        // override certutil_cache_paths if the argument -p/--path is specified.
        certutil_cache_paths = args.values_of("PATH").unwrap().collect();
    }
    for dir in certutil_cache_paths {
        for entry in glob(dir).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let fullpath = path.as_path().to_str().unwrap();
                    let parsed = CryptnetURLCacheParser::parse_file(fullpath,use_content).unwrap();
                    match output_format 
                    {
                        OutputFormat::JSONL => {
                            let json_data = serde_json::to_string(&parsed).unwrap();
                            output.write(json_data.as_bytes()).expect("Error Writing Data !");
                            output.write(b"\r\n").expect("Error Writing Data !");
                        },
                        OutputFormat::CSV => {
                            let data = output_data_csv(&parsed).unwrap();
                            output.write(data.as_bytes()).expect("Error Writing Data !");
                        }
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
}