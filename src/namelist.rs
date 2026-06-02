use std::collections::HashMap;

// The Namelist struct is used to store the mapping of labels to their corresponding addresses in the assembled code.
#[derive(Debug)]
pub struct Namelist {
    pub labels: HashMap<String, u16>,
}

impl Namelist {
    pub fn new() -> Self {
        Namelist {
            labels: HashMap::new(),
        }
    }

    pub fn insert(&mut self, label: String, address: u16) {
        self.labels.insert(label, address);
    }

    pub fn get(&self, label: &str) -> Option<&u16> {
        self.labels.get(label)
    }
}
