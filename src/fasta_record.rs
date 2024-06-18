#[derive(Debug, Eq, PartialEq, PartialOrd, Ord,)]
pub struct FastaRecord {
    pub family: String, 
    pub genus: String,
    pub species: String,
    len: u32,
    pub id: String,
    pub seq: Vec<u8>
}

impl FastaRecord { 
    pub fn new(id: String, family: String, genus: String, species: String, len: u32, seq: Vec<u8>) -> FastaRecord {
        FastaRecord {
            id: id,
            family: family,
            genus: genus,
            species: species,
            len: len,
            seq: seq
        }
    }
}