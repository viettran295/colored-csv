use std::{fmt::format, vec};
use colored::*;

pub struct TableFormatter {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    compensate: i32,
}

impl TableFormatter {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { 
            headers, 
            rows,
            compensate: 7,
        }
    }

    fn calculate_column_widths(&self) -> Vec<usize> {
        let mut widths = self.headers.iter().map(|h| h.len()).collect::<Vec<usize>>();
        
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        return widths;   
    }

    fn format_row(rows: &[String], widths: &[usize]) -> String {
        let mut output: Vec<String> = Vec::new();
        for (row, width) in rows.iter().zip(widths) {
            output.push(format!("{:<width$}", row, width = width));
        }
        return output.join(" ┃ ")
    }

    fn create_border(widths: &[usize], left: char, middle: char, right: char) -> String {
        let mut border = String::from(left);
        
        for (i, &width) in widths.iter().enumerate() {
            border.push_str(&"━".repeat(width - 7));
            if i < widths.len() - 1 {
                border.push(middle);
            }
        }
        
        border.push(right);
        border
    }

    fn create_top_border(widths: &[usize]) -> String {
        Self::create_border(widths, '┏', '┳', '┓')
    }

    fn create_bottom_border(widths: &[usize]) -> String {
        Self::create_border(widths, '┗', '┻', '┛')
    }

    fn create_separator(widths: &[usize]) -> String {
        Self::create_border(widths, '┣', '╋', '┫')
    }

    pub fn to_string(&self) -> String {
        let widths = self.calculate_column_widths();
        
        // Format headers
        let header_row = Self::format_row(&self.headers, &widths);
        
        // Create borders and separator
        let top_border = Self::create_top_border(&widths);
        let separator = Self::create_separator(&widths);
        let bottom_border = Self::create_bottom_border(&widths);
        
        // Format data rows
        let data_rows: Vec<String> = self.rows.iter()
            .map(|row| Self::format_row(row, &widths))
            .collect();
        
        // Combine all parts
        let mut output = vec![
            top_border,
            format!("┃ {} ┃", header_row),
            separator,
        ];
        
        // Add data rows with borders
        for row in data_rows {
            output.push(format!("┃ {} ┃", row));
        }
        
        output.push(bottom_border);
        output.join("\n")
    }
}
