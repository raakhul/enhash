use structopt::StructOpt;

mod encodings;
mod hashings;

#[derive(Debug, StructOpt)]
#[structopt(name = "enhash")]
struct CliArgs {

    #[structopt(short = "e", long = "encode", help = "Available Encoding type base64, url, html, binary, decimal, hex", global = true, default_value = "")]
    encode: String,
    
    #[structopt(short = "d", long = "decode", help = "Available Decoding type base64, url, html, binary, decimal, hex", global = true, default_value = "")]
    decode: String,

    #[structopt(short = "s", long = "hash", help = "Available Hashing type md5, sha224, sha256, sha384, sha512, sha3_224, sha3_256, sha3_384, sha3_512, gost94", global = true, default_value = "")]
    hash: String,

    input: String,
}


fn main() {
    let args = CliArgs::from_args();
 
    
    let input_text=args.input.trim().to_string();

    if args.encode != "" 
    {
        let encoded_output=encodings::string_encode(args.encode.to_lowercase(), input_text);
        println!("{}: {}",encoded_output.0,encoded_output.1)

    }
    else if args.decode != "" 
    {
        let decoded_output=encodings::numeric_decode(args.decode.to_lowercase(), input_text);
        println!("{}: {}",decoded_output.0,decoded_output.1)

    }
    else if args.hash != "" 
    {
        hashings::string_hashing(args.hash.to_lowercase(), input_text);
    }
    else
    {
        println!("Invalid Option");
    }
}
