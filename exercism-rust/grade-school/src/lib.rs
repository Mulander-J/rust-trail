use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.grades.values().flatten().any(|x| x == student) {
            // self.grades.entry(grade)
            //     .and_modify(|students| students.push(student.to_string()))
            //     .or_insert(vec![student.to_string()]);
            let entry = self.grades.entry(grade).or_default();
            entry.push(student.to_string());
            entry.sort();
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut s = self.grades.keys().cloned().collect::<Vec<_>>();
        s.sort();
        s
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades.get(&grade).cloned().unwrap_or(vec![])
    }
}
