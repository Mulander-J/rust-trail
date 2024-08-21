use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    /* Solution1 */
    // match nucleotide {
    //     'A' | 'C' | 'G' | 'T' => {
    //         if let Some(c) = dna.chars().find(|x| !['A', 'C', 'G', 'T'].contains(x)) {
    //             Err(c)
    //         } else {
    //             Ok(dna.split(nucleotide).count().saturating_sub(1))
    //         }
    //     }
    //     _ => Err(nucleotide),
    // }

    /* Solution2 */
    // if !matches!(nucleotide, 'A' | 'C' | 'G' | 'T') {
    //     return Err(nucleotide);
    // }
    //
    // if let Some(c) = dna.chars().find(|&c| !matches!(c, 'A' | 'C' | 'G' | 'T')) {
    //     Err(c)
    // } else {
    //     Ok(dna.chars().filter(|&c| c == nucleotide).count())
    // }

    /* Solution3 */
    // nucleotide_counts(dna).and_then(|hm| hm.get(&nucleotide).cloned().ok_or(nucleotide))
    // nucleotide_counts(dna)?.remove(&nucleotide).ok_or(nucleotide)
    nucleotide_counts(dna)?.get(&nucleotide).cloned().ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // let mut map = HashMap::from([
    //     ('A', 0),
    //     ('C', 0),
    //     ('G', 0),
    //     ('T', 0)
    // ]);

    let mut map: HashMap<char, usize> = "ACGT".chars().map(|c| (c, 0)).collect();

    for c in dna.chars() {
        let count = map.get_mut(&c).ok_or(c)?;
        *count += 1;
    }
    Ok(map)
}
