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

    pub fn add_row(& mut self, row: csv_row::CSVRow)
    {
        self.contents.push(row);
    }

    pub fn remove_row(& mut self, index: usize) -> Result<(), &'static str>
    {
        if index >= self.contents.len() {
            return Err("Index is too large.");
        }

        self.contents.remove(index);

        return Ok(());
    }

    pub fn row_count (&self) -> usize
    {
        return self.contents.len();
    }

    pub fn to_string(&self) -> String
    {
        let mut str = self.header.to_string();
        str.push('\n');

        for i in 0..self.contents.len()
        {
            str += &self.contents[i].to_string();
            str.push('\n');
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

    #[test]
    fn test_to_string_single_line() {
        let doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![]};
        assert_eq!(doc.to_string(), "name,dob,location\n");
    }

    #[test]
    fn test_to_string_multi_line() {
        let doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("NYC")]}]};
        assert_eq!(doc.to_string(), "name,dob,location\njames,14/03/2000,NYC\n");
    }

    #[test]
    fn test_to_string_multi_line_2() {
        let doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]}]};
        assert_eq!(doc.to_string(), "name,dob,location\njames,14/03/2000,\"\"\"Jersey\"\"\"\n");
    }

    #[test]
    fn test_row_count() {
        let doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]}]};
        assert_eq!(doc.row_count(), 1);
    }

    #[test]
    fn test_add_row() {
        let mut doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]}]};
        assert_eq!(doc.row_count(), 1);
        let row = csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]};
        doc.add_row(row);
        assert_eq!(doc.row_count(), 2);
    }

    #[test]
    #[should_panic(expected = "Index is too large.")]
    fn test_remove_row_fail() {
        let mut doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]}]};
        assert_eq!(doc.row_count(), 1);
        let row = csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]};
        doc.add_row(row);
        assert_eq!(doc.row_count(), 2);
        match doc.remove_row(3)
        {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    }

    #[test]
    fn test_remove_row() {
        let mut doc: CSVDocument = CSVDocument {header: csv_row::CSVRow {cells: vec![String::from("name"),String::from("dob"),String::from("location")]}, contents: vec![csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]}]};
        assert_eq!(doc.row_count(), 1);
        let row = csv_row::CSVRow {cells: vec![String::from("james"),String::from("14/03/2000"),String::from("\"Jersey\"")]};
        doc.add_row(row);
        assert_eq!(doc.row_count(), 2);
        match doc.remove_row(0)
        {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
        assert_eq!(doc.row_count(), 1);
    }
}
