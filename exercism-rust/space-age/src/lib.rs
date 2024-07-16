#[derive(Debug)]
pub struct Duration(f64);

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / EARTH_YEAR_SECONDS)
    }
}

pub trait Planet {
    const EARTH_RATE: f64;
    fn years_during(d: &Duration) -> f64{
        d.0 / Self::EARTH_RATE
    }
}

// Try [marco_rules](https://doc.rust-lang.org/stable/book/ch19-06-macros.html)
macro_rules!  planet {
    ($n:ident, $p:expr) => {
        pub struct $n;
        impl Planet for $n {
            const EARTH_RATE: f64 = $p;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

// macro_rules! boilerplate {
//     ($($t:ident => $e:expr),+) => {
//         $(
//             pub struct $t {}
//             impl Planet for $t {
//                 const EARTH_YEAR_RATIO: f64 = $e;
//             }
//         )*
//     }
// }
// boilerplate!(
//     Mercury => 0.2408467,
//     Venus => 0.61519726,
//     Earth => 1.0,
//     Mars => 1.8808158,
//     Jupiter => 11.862615,
//     Saturn => 29.447498,
//     Uranus => 84.016846,
//     Neptune => 164.7913);

// pub struct Mercury;
// pub struct Venus;
// pub struct Earth;
// pub struct Mars;
// pub struct Jupiter;
// pub struct Saturn;
// pub struct Uranus;
// pub struct Neptune;
//
// impl Planet for Mercury {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (0.2408467 * EARTH_YEAR_SECONDS)
//     }
// }
//
// impl Planet for Venus {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (0.61519726 * EARTH_YEAR_SECONDS)
//     }
// }
//
// impl Planet for Earth {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / EARTH_YEAR_SECONDS
//     }
// }
//
// impl Planet for Mars {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (1.8808158 * EARTH_YEAR_SECONDS)
//     }
// }
//
// impl Planet for Jupiter {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (11.862615 * EARTH_YEAR_SECONDS)
//     }
// }
//
// impl Planet for Saturn {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (29.447498 * EARTH_YEAR_SECONDS)
//     }
// }
//
// impl Planet for Uranus {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (84.016846 * EARTH_YEAR_SECONDS)
//     }
// }
//
// impl Planet for Neptune {
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds as f64 / (164.79132 * EARTH_YEAR_SECONDS)
//     }
// }