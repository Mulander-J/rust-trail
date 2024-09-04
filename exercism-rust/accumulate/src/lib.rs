/// What should the type of _function be?
// pub fn map<T, K, F: FnMut(T) -> K>(input: Vec<T>, mut _function: F) -> Vec<K> {
pub fn map<T, K>(input: Vec<T>, mut _function: impl FnMut(T) -> K) -> Vec<K> {
    let mut ret = Vec::with_capacity(input.len());
    for x in input {
        ret.push(_function(x));
    }
    ret
}
