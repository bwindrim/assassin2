use std::collections::HashMap;

#[derive(Debug)]
pub struct Namelist {
    labels: HashMap<String, i32>,
}

impl Namelist {
    pub fn new() -> Self {
        Namelist {
            labels: HashMap::new(),
        }
    }
    pub fn insert(&mut self, label: String, value: i32) {
        self.labels.insert(label, value);
    }

    pub fn get(&self, label: &str) -> Option<&i32> {
        self.labels.get(label)
    }
}