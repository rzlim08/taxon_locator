mod fasta_record;
mod current_taxon_tracker;
mod taxon_byterange;

use serde_json;
use bio::io::fasta;
use std::fs::File;
use regex::Regex;
use std::io::{Write, BufReader, BufWriter};
use std::path::Path;
use clap::Parser;

use crate::fasta_record::FastaRecord;
use crate::current_taxon_tracker::CurrentTaxonTracker; 
use crate::taxon_byterange::TaxonByterange;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    input: String,
    #[clap(short, long, value_parser)]
    output_prefix: String,
    /// Sets the output JSON file for taxon byteranges
    #[clap(short, long, value_parser)]
    json_prefix: String,
}

fn read_fasta_records(hit_type: &str, reader: fasta::Reader<BufReader<File>>) -> Vec<FastaRecord> {
    let mut records: Vec<FastaRecord> = Vec::new();
    let pattern = format!(r"family_{}:(-?\d+):.*?genus_{}:(-?\d+):.*?species_{}:(-?\d+)", hit_type, hit_type, hit_type);
    let re = Regex::new(&pattern).unwrap();

    for record in reader.records() {
        let record = record.unwrap();
        if let Some(caps) = re.captures(record.id()) {
            let family = caps.get(1).unwrap().as_str();
            let genus = caps.get(2).unwrap().as_str();
            let species = caps.get(3).unwrap().as_str();
            let id = record.id().to_string();
            let seq = record.seq().to_vec();
            let len = seq.len() as u32;
            let fasta_record = FastaRecord::new(id, family.to_string(), genus.to_string(), species.to_string(), len, seq);
            records.push(fasta_record);
        }
    }
    records
}

fn parse_fasta_for_hit_type(file: File, hit_type: &str, output: &str, mut taxon_byteranges: Vec<TaxonByterange> ) -> Result<Vec<TaxonByterange>, std::io::Error> {

    let reader = fasta::Reader::new(file);
    let mut records = read_fasta_records(&hit_type, reader);
    println!("Number of records: {}", records.len());
    records.sort();

    let path = Path::new(output);
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    let mut current_taxon_tracker = CurrentTaxonTracker::new();
    let mut end_position: u64 = 0;

    for record in records{
        current_taxon_tracker.update(&record.family, &record.genus, &record.species, end_position, &mut taxon_byteranges, &hit_type);
        writeln!(writer, ">{}\n{}", record.id, String::from_utf8_lossy(&record.seq))?;
        end_position += record.id.len() as u64 + record.seq.len() as u64 + 3;
    }
    current_taxon_tracker.finalize(&mut taxon_byteranges, end_position, &hit_type);
    // Flush the buffer to ensure all data is written to the file
    writer.flush()?;
    Ok(taxon_byteranges)
}


fn main() -> Result<(), std::io::Error> {
    
    let cli = Cli::parse();
    let input = &cli.input;
    let output_prefix = &cli.output_prefix;
    let taxon_byterange_prefix = &cli.json_prefix;   

    let hit_types = vec!["nt", "nr"];
    let mut taxon_byteranges: Vec<TaxonByterange> = Vec::new();


    for hit_type in hit_types {
        taxon_byteranges = parse_fasta_for_hit_type(
            File::open(input).unwrap(),
            &hit_type,
            &format!("{}_{}.fasta", output_prefix, hit_type),
            taxon_byteranges
        )?;
    }

    let taxon_byteranges_json = serde_json::to_string_pretty(&taxon_byteranges)?;
    let mut file = File::create(format!("{}.json", taxon_byterange_prefix))?;
    file.write_all(taxon_byteranges_json.as_bytes())?;


    // Return Ok if everything was successful
    Ok(())
}
