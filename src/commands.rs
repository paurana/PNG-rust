use std::fs;
use std::str::FromStr;

use crate::{args::*, chunk::Chunk, chunk_type::ChunkType, png::Png};
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Encode(args) => match encode(args) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Commands::Decode(args) => match decode(args) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Commands::Remove(args) => match remove(args) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Commands::Print(args) => match print(args) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
    }
}

fn encode(encode: &Encode) -> Result<String> {
    let vec = fs::read(&encode.path)?;
    let mut png = Png::try_from(&vec[..])?;

    let chunktype = ChunkType::from_str(encode.chunktype.as_str())?;
    let chunk = Chunk::new(chunktype, encode.message.as_bytes().to_vec());

    png.append_chunk(chunk);

    fs::write(&encode.path, png.as_bytes())?;

    if let Some(output) = &encode.output {
        fs::write(output, "Encoded")?;
    }

    Ok("Encoded".to_string())
}

fn decode(decode: &Decode) -> Result<String> {
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

    Ok("Decoded".to_string())
}

fn remove(remove: &Remove) -> Result<String> {
    let vec = fs::read(&remove.path)?;
    let mut png = Png::try_from(&vec[..])?;

    match png.remove_chunk(&remove.chunktype) {
        Ok(chunk) => {
            println!("{} {}", &remove.chunktype, chunk.data_as_string().unwrap());
            fs::write(&remove.path, png.as_bytes())?;
            Ok("Removed".to_string())
        }
        Err(err) => {
            println!("chunk not deleted");
            Err(err)
        }
    }
}

fn print(print: &Print) -> Result<String> {
    use std::process::Command;
    match Command::new("feh").arg(&print.path).output() {
        Ok(_) => Ok("Printed".to_string()),
        Err(_) => {
            let vec = fs::read(&print.path)?;
            println!("{:?}", vec);
            Ok("Printed".to_string())
        }
    }
}
