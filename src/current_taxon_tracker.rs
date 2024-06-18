use crate::taxon_byterange::TaxonByterange;

pub struct CurrentTaxonTracker {
    family: String,
    genus: String,
    species: String,
    family_position: u64,
    genus_position: u64,
    species_position: u64
}

impl CurrentTaxonTracker {
    pub fn new() -> CurrentTaxonTracker {
        CurrentTaxonTracker {
            family: String::new(),
            genus: String::new(),
            species: String::new(),
            family_position: 0,
            genus_position: 0,
            species_position: 0
        }
    }
    pub fn update(&mut self, family: &str, genus: &str, species: &str, current_position: u64, taxon_byteranges: &mut Vec<TaxonByterange>, hit_type: &str){ 
        if self.family != family {
            if current_position != 0 {
                taxon_byteranges.push(
                    TaxonByterange::new(
                        self.family.clone(),
                        self.family_position,
                        current_position, 
                        hit_type
                    )
                );
            }
            self.family = family.to_string();
            self.family_position = current_position;
        }
        if self.genus != genus {
            if current_position != 0 {
                taxon_byteranges.push(
                    TaxonByterange::new(
                        self.genus.clone(),
                        self.genus_position,
                        current_position, 
                        hit_type
                    )
                );
            }
            self.genus = genus.to_string();
            self.genus_position = current_position;
        }
        if self.species != species {
            if current_position != 0 {
                taxon_byteranges.push(
                    TaxonByterange::new(
                        self.species.clone(),
                        self.species_position,
                        current_position,
                        hit_type
                    )
                );
            }
            self.species = species.to_string();
            self.species_position = current_position;
        }
    }
    pub fn finalize(&mut self, taxon_byteranges: &mut Vec<TaxonByterange>, current_position: u64, hit_type: &str){
        taxon_byteranges.push(
            TaxonByterange::new(
                self.family.clone(),
                self.family_position,
                current_position,
                hit_type
            )
        );
        taxon_byteranges.push(
            TaxonByterange::new(
                self.genus.clone(),
                self.genus_position,
                current_position, 
                hit_type
            )
        );
        taxon_byteranges.push(
            TaxonByterange::new(
                self.species.clone(),
                self.species_position,
                current_position, 
                hit_type
            )
        )
    }
}