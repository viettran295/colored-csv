use std::{fs::File, io::BufReader, collections::HashMap};
use csv::{Reader, ReaderBuilder};
use colored::*;
use log::{debug, error};

pub struct CSVProcessor {
    delimiter: u8
}

impl CSVProcessor {

    pub fn new() -> CSVProcessor {
        CSVProcessor {
            delimiter: b','   
        }
    }

    pub fn read_csv(&mut self, file_path: &str) -> String {
        self.delimiter = self.detect_delimiter(&file_path);
        debug!("Detected delimiter: `{}`", self.delimiter);

        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Error opening csv file {}: {}", file_path, e);
                return String::new();
            }
        };
        let reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .from_reader(reader);
        return self.colorized_contents(&mut rdr, self.delimiter);
    }

    fn detect_delimiter(&self, file_path: &str) -> u8 {
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening csv file {}: {}", file_path, e);
                return b',';
            }
        };
        let reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader);
        
        let delimiters = vec![b',', b';', b'\t', b'|', b'/', b'\\'];
        let mut delimiter_counts: HashMap<u8, usize> = HashMap::new();
        
        for (i, record) in rdr.records().enumerate() {
            if i >= 5 { // Only check first 5 records
                break;
            }
            if let Ok(record) = record {
                let line = record.iter().collect::<Vec<_>>().join("");
                for &delimiter in &delimiters {
                    let count = line.as_bytes().iter().filter(|&&b| b == delimiter).count();
                    *delimiter_counts.entry(delimiter).or_insert(0) += count;
                }
            }
        }
        delimiter_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(delimiter, _)| delimiter)
            .unwrap_or(b',') // Default to comma if no clear delimiter found
    }

    fn colorized_contents(&self, reader: &mut Reader<BufReader<File>>, delimiter: u8) -> String {
        let mut contents: String = String::new();
        let mut color_idx: usize = 0;
        let colors: Vec<Color> = vec![
            Color::BrightGreen,
            Color::BrightWhite,
            Color::BrightMagenta,
            Color::BrightYellow,
            Color::BrightWhite,
            Color::BrightCyan,
        ];

        if let Some(headers) = reader.headers().ok(){
            for header in headers.iter() {
                let color = colors[color_idx];
                let colored_txt = header.color(color);
                contents.push_str(&colored_txt.to_string());
                contents.push(delimiter as char);
                color_idx = (color_idx + 1) % colors.len();
            }  
            contents.pop();
            contents.push('\n');
        }

        for record in reader.records(){
            color_idx = 0;
            match record{
                Ok(record) => {
                    for field in record.iter() {
                        let color = colors[color_idx];
                        let colored_txt = field.color(color);
                        contents.push_str(&colored_txt.to_string());
                        contents.push(delimiter as char);
                        color_idx = (color_idx + 1) % colors.len();
                    }
                    contents.pop();
                    contents.push('\n');
                }
                Err(e) => {
                    error!("Error reading record: {}", e);
                }
            }
        }
        return  contents;
    }
}

