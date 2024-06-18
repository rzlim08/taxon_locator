use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonByterange {
    #[serde(rename = "taxid")]
    taxon_id: String,
    #[serde(rename = "first_byte")]
    start: u64,
    #[serde(rename = "last_byte")]
    end: u64,
    hit_type: String
}

impl TaxonByterange { 
    pub fn new(taxon_id: String, start: u64, end: u64, hit_type: &str) -> TaxonByterange {
        TaxonByterange {
            taxon_id: taxon_id,
            start: start,
            end: end,
            hit_type: hit_type.to_uppercase().to_string()
        }
    }
}