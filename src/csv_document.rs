#[path = "csv_row.rs"]
mod csv_row;

pub struct CSVDocument {
    pub header: csv_row::CSVRow,
    pub contents: Vec<csv_row::CSVRow>,
}

impl CSVDocument {
    pub fn new() -> CSVDocument {
        return CSVDocument {
            header: csv_row::CSVRow::new(),
            contents: vec![],
        };
    }

    pub fn to_string(&self) -> String
    {
        let mut str = self.header.to_string();

        for i in 0..self.contents.len()
        {
            str += &self.contents[i].to_string();
        }

        return str;
    }

    pub fn parse_string(input: &String) -> Result<CSVDocument, &'static str> {
        let mut header_row = csv_row::CSVRow::new();
        let mut header_row_set = false;
        let mut contents_rows = vec![];

        let characters: Vec<char> = input.chars().collect();
        let mut i = 0;
        let mut in_quotes = false;
        let mut line = String::new();

        while i < characters.len() {
            if characters[i] == '"' && !in_quotes {
                in_quotes = true;
            } else if characters[i] == '"' && in_quotes {
                if i + 1 >= characters.len() {
                    in_quotes = false;
                } else {
                    let p = characters[i + 1];

                    if p != '\0' && p != '"' {
                        in_quotes = false;
                    } else if p == '"' {
                        i += 1;
                        line.push('"');
                    }
                }
            }

            if characters[i] == '\n' && !in_quotes {
                if !header_row_set {
                    let res = header_row.parse_line(&line);
                    match res {
                        Err(e) => return Err(e),
                        Ok(_) => (),
                    };

                    line = String::new();
                    header_row_set = true;
                } else {
                    let res = csv_row::CSVRow::parse_line_new(&line);

                    let current_row;

                    match res {
                        Err(e) => return Err(e),
                        Ok(row) => current_row = row,
                    }

                    if current_row.len() != header_row.len() {
                        return Err("The amount of cells in the row was invalid.");
                    } else {
                        contents_rows.push(current_row);
                    }

                    line = String::new();
                }
            } else {
                line.push(characters[i]);
            }

            i += 1;
        }

        if line != String::new() {
            if !header_row_set {
                let res = header_row.parse_line(&line);
                match res {
                    Err(e) => return Err(e),
                    Ok(_) => (),
                };
            } else {
                let res = csv_row::CSVRow::parse_line_new(&line);

                match res {
                    Err(e) => return Err(e),
                    Ok(row) => contents_rows.push(row),
                }
            }
        }
        return Ok(CSVDocument {
            header: header_row,
            contents: contents_rows,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_string_1() {
        let string: String = "cats,dogs,cars,humans".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
    }

    #[test]
    fn test_parse_string_2() {
        let string: String = "cats,dogs,cars,humans\n1,500,0,500\n".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
        assert_eq!(doc.contents.len(), 1);
        assert_eq!(doc.contents[0].cells, vec!["1", "500", "0", "500"]);
    }

    #[test]
    fn test_parse_string_3() {
        let string: String = "cats,dogs,cars,humans\n1,500,0,500\n2,20,0,20\n".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
        assert_eq!(doc.contents.len(), 2);
        assert_eq!(doc.contents[0].cells, vec!["1", "500", "0", "500"]);
        assert_eq!(doc.contents[1].cells, vec!["2", "20", "0", "20"]);
    }

    #[test]
    fn test_parse_string_embedded_quotes() {
        let string: String = "cats,dogs,cars,humans\nnone,\"\"\"james\"\"\",none,none".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
        assert_eq!(doc.contents.len(), 1);
        assert_eq!(
            doc.contents[0].cells,
            vec!["none", "\"james\"", "none", "none"]
        );
    }

    #[test]
    fn test_parse_string_embedded_quotes_2() {
        let string: String = "cats,dogs,cars,humans\nnone,\"\"\"james\"\"\",none,none\n\"james,\"\"cars!\"\"\",none,none,none".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
        assert_eq!(doc.contents.len(), 2);
        assert_eq!(
            doc.contents[0].cells,
            vec!["none", "\"james\"", "none", "none"]
        );
        assert_eq!(
            doc.contents[1].cells,
            vec!["james,\"cars!\"", "none", "none", "none"]
        );
    }

    #[test]
    fn test_parse_string_embedded_nl() {
        let string: String =
            "cats,dogs,cars,humans\nnone,\"james,\njohn\n catherine!\",none,none".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
        assert_eq!(doc.contents.len(), 1);
        assert_eq!(
            doc.contents[0].cells,
            vec!["none", "james,\njohn\n catherine!", "none", "none"]
        );
    }

    #[test]
    fn test_parse_string_embedded_nl_2() {
        let string: String = "cats,dogs,cars,humans\nnone,\"james,\njohn\n catherine!\",none,none\nnone,none,none,none\n".to_string();
        let doc: CSVDocument = CSVDocument::parse_string(&string).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(doc.header.cells, vec!["cats", "dogs", "cars", "humans"]);
        assert_eq!(doc.contents.len(), 2);
        assert_eq!(
            doc.contents[0].cells,
            vec!["none", "james,\njohn\n catherine!", "none", "none"]
        );
        assert_eq!(doc.contents[1].cells, vec!["none", "none", "none", "none"]);
    }
}
