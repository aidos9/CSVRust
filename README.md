# CSVRust
CSV Parsing Library written in rust

## Usage guide
For an example project see the examples directory. To run an example use:
```
cargo run --example [EXAMPLE_NAME]
```

Parsing a string of CSV is as simple as:
```
csv_parser::CSVDocument::parse_string("header01,header02");
```

## Documentation
The CSVDocument and CSVRow structs are the two main structs of the library. They represent a row of CSV and the whole file.
#### CSVDocument
Represents the whole file.

##### Fields:

```header: CSVRow``` : The first row of the CSV file. The columns of the file can be referenced from the cells in the header row.

```contents: Vec<CSVRow>``` : A vector containing each row excluding the header row of the CSV file.

##### Public methods:
```fn new() -> CSVDocument``` : Returns an empty CSVDocument

```fn retrieve_cell(&mut self, row_index: usize, cell_index: usize) -> String``` : Returns a string representation of the cell from the specified row and index.

```fn add_row(&mut self, row: CSVRow)``` : Appends a row to the contents field.

```fn remove_row(&mut self, index: usize) -> Result<(), &'static str>``` : Removes a row from the contents field at the specified index.

```fn row_count(&self) -> usize``` : Returns the number of rows in the contents field.

```fn to_string(&self) -> String``` : Serializes the document into a string of CSV.

```fn parse_string(input: &String) -> Result<CSVDocument, &'static str>``` : Parses a CSV string, supports quoted fields containing new lines, commas and quotes escaped with another quote e.g. "" represents one quote in the cell. Returns either a new document or an error message.

#### CSVRow
Represents a single row of the CSV file.

##### Fields:

```cells: Vec<String>``` : A vector containing each cell in a row of the CSV file.

##### Public methods:
```fn parse_line_new(line: &String) -> Result<CSVRow, &'static str>``` : Parses a single line of the CSV file and returns either  a new row or an error message.

pub fn parse_line(&mut self, line: &String) -> Result<(), &'static str>``` : Parses a single line of the CSV file into a pre-existing row object and either returns nothing or an error message.

```fn to_string(&self) -> String``` : Parses the row into a string of CSV, supports quoted fields containing new lines, commas and quotes escaped with another quote e.g. "" represents one quote in the cell. Returns either a new document or an error message.

```fn len(&self) -> usize``` : Returns the number of cells in the row.

```fn new() -> CSVRow``` : Constructs an empty row and returns it.
