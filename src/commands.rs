use std::{
    convert::TryInto,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

use crate::{Result, args::{DecodeArgs, EncodeArgs, RemoveArgs}, chunk::Chunk, chunk_type::ChunkType, png::Png};

fn take_png<P: AsRef<Path>>(input: P) -> Result<Png> {
    let mut file = OpenOptions::new().write(true).read(true).open(input)?;
    let mut buffer = Vec::with_capacity(1_000_000);

    file.read_to_end(&mut buffer)?;
    Ok(buffer.as_slice().try_into()?)
}

pub fn encode<S: AsRef<Path>>(input: S, args: EncodeArgs) -> Result<()> {
    let mut png = take_png(&input)?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes(),
    ));

    let mut file = std::fs::File::create(input)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn decode<S: AsRef<Path>>(input: S, args: DecodeArgs) -> Result<()> {
    let png = take_png(&input)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!(
            "Hidden message in the chunk {}: '{}'",
            chunk.chunk_type().to_string(),
            chunk.data_as_string()?
        );
    } else {
        // return Err(Error::Custom("Unable to decode chunk"));
    }
    Ok(())
}

pub fn remove<S: AsRef<Path>>(input: S, args: RemoveArgs) -> Result<()> {
    let mut png = take_png(&input)?;
    png.remove_chunk(&args.chunk_type)?;

    let mut file = std::fs::File::create(input)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn print(input: &Path) -> Result<()> {
    let png = take_png(&input)?;
    println!("File: {}, Size: {}", input.display(), png.as_bytes().len());
    for (i, chunk) in png.chunks().iter().enumerate() {
        print!("\n({})", i + 1);
        print!("{}", chunk);
    }
    Ok(())
}