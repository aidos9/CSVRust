pub mod parser {

    pub struct XMLRow {
        cells: Vec<String>,
    }

    pub struct XMLDocument {
        header: XMLRow,
        contents: Vec<XMLRow>,
    }

    impl XMLDocument {
        /*pub fn parse_string() -> XMLDocument
        {

        }*/
    }

    impl XMLRow {
        pub fn parse_line(line: &String) -> XMLRow {
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

            return XMLRow {cells};
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
        let row = parser::XMLRow::parse_line(&line);
    }
}
