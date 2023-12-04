
#[cfg(test)]
mod tests;

#[cfg(not(feature="derive"))]
use parity_scale_codec_derive::{Encode, Decode};

// use parity_scale_codec::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Student {
    name: Vec<u8>,
    score: u8,
    id: u16,
}

