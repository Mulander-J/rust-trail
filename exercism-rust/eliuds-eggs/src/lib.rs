pub fn egg_count(display_value: u32) -> usize {
    /* Solution2 */
    // display_value.count_ones()

    /* Solution1 */
    let mut val = display_value.clone() as usize;
    let mut count = 0;
    loop {
        if val <= 0 {
            break;
        }
        count += val % 2; // count += val & 1 // 使用位与操作符检查最低位（0 或 1），并将其累加到 count 中
        val /= 2; // val >>= 1 // 使用右移操作符将 val 除以 2，相当于右移一位
    }

    count
}
