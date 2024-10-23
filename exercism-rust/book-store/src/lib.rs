pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts: Vec<u32> = books.iter().fold(vec![0; 5], |mut acc, &bk| {
        acc[(bk as usize) - 1] += 1;
        acc
    });
    counts.sort_unstable();

    let n5 = counts[0];
    let [n4, n3, n2, n1]: [u32; 4] = counts
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    8 * (n1 * 100 + n2 * 190 + n3 * 270 + n4 * 320 + n5 * 375 - 5 * n3.min(n5))
}