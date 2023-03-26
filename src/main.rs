use clap::Parser;
use dysentropy::{deobfuscate_iter, obfuscate_iter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    input: std::path::PathBuf,

    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    output: std::path::PathBuf,

    #[arg(short, long, default_value_t = true)]
    encode: bool,

    #[arg(short, long)]
    decode: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    if args.encode {
        let input_bytes = std::fs::read(&args.input)?;
        let obfuscated: Vec<_> = obfuscate_iter(&input_bytes[..]).collect();

        std::fs::write(&args.output, &obfuscated)?;
        println!("Wrote obfuscated bytes to {:?}", &args.output);
    } else if args.decode {
        let input_bytes = std::fs::read(&args.input)?;
        let obfuscated: Vec<_> = deobfuscate_iter(&input_bytes[..]).collect();

        std::fs::write(&args.output, &obfuscated)?;
        println!("Wrote deobfuscated bytes to {:?}", &args.output);
    }

    Ok(())
}
