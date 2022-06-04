use std::fs;
use std::str::FromStr;

use crate::{args::*, chunk::Chunk, chunk_type::ChunkType, png::Png};
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print(args),
    }
}

fn encode(encode: &Encode) -> Result<()> {
    let vec = fs::read(&encode.path)?;
    let mut png = Png::try_from(&vec[..])?;

    let chunktype = ChunkType::from_str(encode.chunktype.as_str())?;
    let chunk = Chunk::new(chunktype, encode.message.as_bytes().to_vec());

    png.append_chunk(chunk);

    fs::write(&encode.path, png.as_bytes())?;

    if let Some(output) = &encode.output {
        fs::write(output, "Encoded")?;
    }

    Ok(())
}

fn decode(decode: &Decode) -> Result<()> {
    let vec = fs::read(&decode.path)?;
    let png = Png::try_from(&vec[..])?;

    match png.chunk_by_type(&decode.chunktype) {
        Some(chunk) => {
            println!("{}", chunk.data_as_string().unwrap());
        }
        None => {
            println!("chunk not found");
        }
    }

    Ok(())
}

fn remove(remove: &Remove) -> Result<()> {
    let vec = fs::read(&remove.path)?;
    let mut png = Png::try_from(&vec[..])?;

    match png.remove_chunk(&remove.chunktype) {
        Ok(chunk) => {
            println!("{} {}", &remove.chunktype, chunk.data_as_string().unwrap());
            fs::write(&remove.path, png.as_bytes())?;
            Ok(())
        }
        Err(err) => {
            println!("chunk not deleted");
            Err(err)
        }
    }
}

fn print(print: &Print) -> Result<()> {
    use std::process::Command;
    match Command::new("feh").arg(&print.path).output() {
        Ok(_) => Ok(()),
        Err(_) => {
            let vec = fs::read(&print.path)?;
            println!("{:?}", vec);
            Ok(())
        }
    }
}
