use std::{fs::File, io::BufReader, collections::HashMap};
use csv::{Reader, ReaderBuilder};
use log::{debug, error};
use crate::tui::csv_data::CSVData;

pub struct CSVProcessor {
    delimiter: u8
}

impl CSVProcessor {

    pub fn new() -> CSVProcessor {
        CSVProcessor {
            delimiter: b','   
        }
    }

    pub fn read_csv(&mut self, file_path: &str) -> CSVData {
        self.delimiter = self.detect_delimiter(&file_path);
        debug!("Detected delimiter: `{}`", self.delimiter);

        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Error opening csv file {}: {}", file_path, e);
                return CSVData::new();
            }
        };
        let reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .from_reader(reader);
        return self.to_csv_data_type(&mut rdr);
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

    fn to_csv_data_type(&self, reader: &mut Reader<BufReader<File>>) -> CSVData {
        let mut header_vec: Vec<String> = Vec::new();
        let mut contents_vec: Vec<Vec<String>> = Vec::new();
        let mut csv_data = CSVData::new();

        if let Some(headers) = reader.headers().ok(){
            for header in headers.iter() {
                header_vec.push(header.to_string());
            }  
        }

        for record in reader.records(){
            let mut row: Vec<String> = Vec::new();
            match record{
                Ok(record) => {
                    for field in record.iter() {
                        row.push(field.to_string());
                    }
                }
                Err(e) => {
                    error!("Error reading record: {}", e);
                }
            }
            contents_vec.push(row);
        }
        csv_data.header = header_vec;
        csv_data.content = contents_vec;
        return csv_data;
    }
}

