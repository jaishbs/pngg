use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
pub struct Png{
    chunks: Vec<Chunk>
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut chunk_vec:Vec<Chunk> = Vec::new();
        let mut curr_chunk: usize = 8;
        while curr_chunk < value.len(){
            // println!("{:?}\n{:?}", value[curr_chunk..curr_chunk+4].to_vec(), value[curr_chunk..].to_vec());
            let curr_len = u32::from_be_bytes(value[curr_chunk..curr_chunk+4].try_into().expect("👺"));
            chunk_vec.push(value[curr_chunk..curr_chunk+12+(curr_len as usize)].try_into()?);
            curr_chunk += curr_len as usize + 12;
        }
        Ok(Png::from_chunks(chunk_vec))
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}

#[allow(unused)]
impl Png {
    const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png{
        Self{ chunks }
    }
    pub fn append_chunk(&mut self, chunk: Chunk){
        self.chunks.push(chunk);
    }
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk>{
        for i in 0..self.chunks().len(){
            if  self.chunks()[i].chunk_type().to_string().as_str() == chunk_type {
                return Ok(self.chunks.remove(i));
            }
        }
        //temp - remove once Error figured out
        return Ok(Chunk::new(ChunkType::from_str("RuSt").unwrap(), vec![1,2,3,4]));
    }
    pub fn header(&self) -> &[u8; 8]{
        &Png::STANDARD_HEADER
    }
    pub fn chunks(&self) -> &[Chunk]{
        &self.chunks
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk>{
        self.chunks()
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }
    pub fn as_bytes(&self) -> Vec<u8>{
        let mut result= Png::STANDARD_HEADER[..].to_vec();
        for i in self.chunks().iter(){
            &mut result.append(&mut i.as_bytes());
        }
        result
    }
}