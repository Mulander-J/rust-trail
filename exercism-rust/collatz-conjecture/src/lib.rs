pub fn collatz(mut n: u64) -> Option<u64> {
    /* Solution1 */
    // if n == 0 {
    //     return None;
    // }
    //
    // let mut steps = 0_u64;
    //
    // loop {
    //     if n == 1 {
    //         break Some(steps);
    //     }
    //     steps += 1;
    //
    //     if n % 2 == 0 {
    //         n /= 2;
    //     } else {
    //         n = n.checked_mul(3)?.checked_add(1)?;
    //     }
    // }

    /* Solution2 */
    if n > 0 {
        for i in 0.. {
            match n {
                1 => return Some(i),
                even if even % 2 == 0 => n /= 2,
                _ => n = n.checked_mul(3)?.checkecd_add(1)?,
            }
        }
    }

    None
}
