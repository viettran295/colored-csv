#[derive(Debug, Clone)]
pub struct CSVData {
    pub header: Vec<String>,
    pub content: Vec<Vec<String>>
}

impl CSVData {
    pub fn new() -> Self {
        Self {
            header: Vec::new(),
            content: Vec::new()
        }
    }
}