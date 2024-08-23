use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

fn get_valid_nucleotides(target: &str, valid_nucleotides: &str) -> Result<String, usize> {
    if let Some(i) = target.find(|c: char| !valid_nucleotides.contains(c)) {
        Err(i)
    } else {
        Ok(target.to_string())
    }
}

impl Dna {
    const NUCLEOTIDES: &'static str = "GCTA";

    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = get_valid_nucleotides(dna, Self::NUCLEOTIDES)?;
        Ok(Self(nucleotides))
    }

    pub fn into_rna(self) -> Rna {
        let d_rna: HashMap<char, char> = Dna::NUCLEOTIDES
            .chars()
            .zip(Rna::NUCLEOTIDES.chars())
            .collect();

        Rna::new(
            &self
                .0
                .chars()
                .map(|c| d_rna.get(&c).unwrap())
                .collect::<String>(),
        )
            .unwrap()
    }
}

impl Rna {
    const NUCLEOTIDES: &'static str = "CGAU";

    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = get_valid_nucleotides(rna, Self::NUCLEOTIDES)?;
        Ok(Self(nucleotides))
    }
}