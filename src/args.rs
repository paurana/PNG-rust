use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]

pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(clap::Args)]
pub struct Encode {
    pub path: PathBuf,
    //#[clap(parse(try_from_str=ChunkType::from_str))]
    pub chunktype: String,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct Decode {
    pub path: PathBuf,
    pub chunktype: String,
}

#[derive(clap::Args)]
pub struct Remove {
    pub path: PathBuf,
    pub chunktype: String,
}

#[derive(clap::Args)]
pub struct Print {
    pub path: PathBuf,
}
