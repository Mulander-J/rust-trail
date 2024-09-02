use std::{collections::HashSet, sync::Mutex};

use lazy_static::lazy_static;
use rand::{Rng, thread_rng};

lazy_static! {
    static ref ALL_ORDINALS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self { name: new_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = new_name();
    }
}

fn new_name() -> String {
    let mut ordinals = ALL_ORDINALS.lock().unwrap();
    loop {
        let ordinal = format!(
            "{}{}{:<03}",
            thread_rng().gen_range(b'A'..=b'Z') as char,
            thread_rng().gen_range(b'A'..=b'Z') as char,
            thread_rng().gen_range(0..=999)
        );
        if !ordinals.contains(&ordinal) {
            ordinals.insert(ordinal.clone());
            return ordinal;
        }
    }
}
