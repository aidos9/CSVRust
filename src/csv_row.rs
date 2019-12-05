pub struct CSVRow {
    pub cells: Vec<String>,
}

impl CSVRow {
    pub fn new() -> CSVRow {
        return CSVRow {cells: vec![]};
    }

    pub fn len(&self) -> usize
    {
        return self.cells.len();
    }

    pub fn parse_line(& mut self, line: &String) -> Result<(), &'static str>
    {
        let mut in_quotes = false;
        let mut current_cell = String::new();
        let mut cells: Vec<String> = Vec::new();

        let characters: Vec<char> = line.chars().collect();

        let mut i = 0;

        while i < characters.len() {

            if characters[i] == '"' && !in_quotes {
                in_quotes = true;
                current_cell = String::new(); // Ignore anything that was there before the quotes.
            }else if characters[i] == '"' && in_quotes {
                if i+1 >= characters.len() {
                    in_quotes = false;
                    cells.push(current_cell);
                    current_cell = String::new();
                }else{
                    let p = characters[i+1];

                    if p != '\0' && p != '"'
                    {
                        in_quotes = false;
                    }else if p == '"' {
                        i += 1;
                    }
                }
            }else {
                if characters[i] == ',' && !in_quotes {
                    cells.push(current_cell);
                    current_cell = String::new();
                }else{
                    current_cell.push(characters[i]);
                }
            }


            i+= 1;
        }

        if in_quotes {
            return Err("Unterminated quotes in cell.");
        }

        if current_cell.len() != 0 {
            cells.push(current_cell);
        }

        self.cells = cells;

        return Ok(());
    }

    pub fn parse_line_new(line: &String) -> Result<CSVRow, &'static str> {
        let mut in_quotes = false;
        let mut current_cell = String::new();
        let mut cells: Vec<String> = Vec::new();

        let characters: Vec<char> = line.chars().collect();

        let mut i = 0;

        while i < characters.len() {

            if characters[i] == '"' && !in_quotes {
                in_quotes = true;
                current_cell = String::new(); // Ignore anything that was there before the quotes.
            }else if characters[i] == '"' && in_quotes {
                if i+1 >= characters.len() {
                    in_quotes = false;
                    cells.push(current_cell);
                    current_cell = String::new();
                }else{
                    let p = characters[i+1];

                    if p != '\0' && p != '"'
                    {
                        in_quotes = false;
                    }else if p == '"' {
                        i += 1;
                        current_cell.push(characters[i]);
                    }
                }
            }else {
                if characters[i] == ',' && !in_quotes {
                    cells.push(current_cell);
                    current_cell = String::new();
                }else{
                    current_cell.push(characters[i]);
                }
            }

            i+= 1;
        }

        if in_quotes {
            return Err("Unterminated quotes in cell.");
        }

        if current_cell.len() != 0 {
            cells.push(current_cell);
        }

        return Ok(CSVRow {cells});
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Unterminated quotes in cell.")]
    fn test_parse_line_unterminated_string()
    {
        let line: String = "header,1,,bob,\"cats, dogs, cars".to_string();
        CSVRow::parse_line_new(&line).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    #[test]
    fn test_parse_line()
    {
        let line: String = "header,1,,bob,\"cats, \"\"dogs\"\", cars\",\"multi-\nlinestring!\"".to_string();
        let row: CSVRow = CSVRow::parse_line_new(&line).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(row.cells, vec!["header","1","","bob","cats, \"dogs\", cars", "multi-\nlinestring!"]);
    }

    #[test]
    fn test_parse_line_embedded_quotes()
    {
        let line: String = "none,\"\"\"james\"\"\",none,none".to_string();
        let row: CSVRow = CSVRow::parse_line_new(&line).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(row.cells, vec!["none","\"james\"","none","none"]);
    }

    #[test]
    fn test_parse_line_embedded_quotes_2()
    {
        let line: String = "\"james,\"\"cars!\"\"\",none,none,none".to_string();
        let row: CSVRow = CSVRow::parse_line_new(&line).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(row.cells, vec!["james,\"cars!\"","none","none","none"]);
    }
}
