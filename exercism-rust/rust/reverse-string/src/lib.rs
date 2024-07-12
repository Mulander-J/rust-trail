use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // v1
    // input.chars().rev().collect()

    // v2
    // UnicodeSegmentation::graphemes(input, true).rev().collect()
    input.graphemes(true).rev().collect()
}
