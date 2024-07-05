// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(vec: &[f64;3]) -> f64 {
    let mut res:f64 = 0.0;
    for v in vec {
        res += v * v;
    };
    res.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(_vec: &mut [f64;3]) {
    let mag = magnitude(_vec);
    for v in _vec {
        *v /= mag;
    }
}

#[test]
fn test_magnitude() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!(
        "Magnitude of {v:?} after normalization: {}",
        magnitude(&v)
    );
}