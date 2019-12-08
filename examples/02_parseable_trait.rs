// Sample code for using the library.
extern crate csv_parser;

// We want to fill out this struct.
struct Person {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    email: String,
}

impl csv_parser::CSVParseable for Person {
    // By implementing Parseable we are able to use the CSVDocument method for constructing an Object
    fn construct_from_row(row: &csv_parser::CSVRow) -> Result<Person, &'static str>
    {
        if row.cells.len() != 4 {
            return Err("Expected 4 cells in the row.");
        }

        return Ok(Person {
            first_name: row.cells[0].clone(),
            last_name: row.cells[1].clone(),
            date_of_birth: row.cells[2].clone(),
            email: row.cells[3].clone(),
        });
    }
}

fn main() {
    let csv_string = String::from("first name,last name,DOB,email\njames,bob,00/00/2101,james@bob.com\ncatherine,crack,00/01/2102,catherine@crack.com\n");
    let doc: csv_parser::CSVDocument;

    // Parse the CSV string and check for any errors
    match csv_parser::CSVDocument::parse_string(&csv_string) {
        Ok(d) => doc = d,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    // Creates a vector of people using the method we previously defined in the CSVParseable trait.
    let people: Vec<Person> = match doc.construct_vector() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    for person in people {
        println!(
            "{} {}, {}, {}",
            person.first_name, person.last_name, person.date_of_birth, person.email
        );
    }
}
