# CryptnetURLCacheParser-rs

CryptnetURLCacheParser-rs is a rust implementation for the python parser I made here https://github.com/AbdulRhmanAlfaifi/CryptnetURLCacheParser.

## How to use

The following is the command line tool help message:

```
CryptnetURLCacheParser 0.1.0
AbdulRhman Alfaifi - @A__ALFAIFI
CryptnetURLCache metadata files parser

USAGE:
    cryptnet_url_cache_parser.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
        --no-headers     Don't print headers when using CSV as the output format
        --use-content    Try finding the cached file and calculate the SHA256 hash for it
    -V, --version        Prints version information

OPTIONS:
    -p, --path <PATH>...                   Path(s) to CryptnetURLCache Metadata Files to be Parsed - accepts glob
                                           (Defaults to all windows paths)
    -o, --output <output>                  The file path to write the output to [default: stdout]
        --output-format <output-format>    Output format. [default: csv]  [possible values: csv, jsonl]
```

## File Structure & Artifact Specifics

I wrote a blog post that describe the file structure for the metadata files. You can find it here https://u0041.co/blog/post/3. 

