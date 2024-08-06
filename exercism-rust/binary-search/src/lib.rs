// Ord https://doc.rust-lang.org/core/cmp/trait.Ord.html
// AsRef https://doc.rust-lang.org/core/convert/trait.AsRef.html
pub fn find<T: Ord, V: AsRef<[T]>>(array: V, key: T) -> Option<usize> {
    // https://doc.rust-lang.org/core/convert/trait.AsRef.html#tymethod.as_ref
    let arr = array.as_ref();
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == key {
            return Some(mid);
        } else if arr[mid] < key {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}
