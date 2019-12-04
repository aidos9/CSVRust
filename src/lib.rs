pub mod parser {

    pub struct CSVRow {
        cells: Vec<String>,
    }

    pub struct CSVDocument {
        header: CSVRow,
        contents: Vec<CSVRow>,
    }

    impl CSVDocument {
        /*pub fn parse_string() -> XMLDocument
        {

        }*/
    }

    impl CSVRow {
        pub fn parse_line(line: &String) -> CSVRow {
            let mut in_quotes = false;
            let mut current_cell = String::new();
            let mut cells: Vec<String> = Vec::new();

            for c in line.chars() {
                if c != ',' {
                    current_cell.push(c);
                }else {
                    cells.push(current_cell);
                    current_cell = String::new();
                }
            }

            return CSVRow {cells};
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line()
    {
        let line: String = "header,1,,bob".to_string();
        let row = parser::CSVRow::parse_line(&line);
    }
}
