pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut ret = Vec::with_capacity(rna.len() / 3);

    for chunk in rna.as_bytes().chunks(3) {
        let codon = std::str::from_utf8(chunk).ok()?;
        if let Some(protein) = match codon {
            "AUG" => Some("Methionine"),
            "UUU" | "UUC" => Some("Phenylalanine"),
            "UUA" | "UUG" => Some("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
            "UAU" | "UAC" => Some("Tyrosine"),
            "UGU" | "UGC" => Some("Cysteine"),
            "UGG" => Some("Tryptophan"),
            "UAA" | "UAG" | "UGA" => break,
            _ => { return None }
        } {
            ret.push(protein);
        }
    }

    Some(ret)
}
