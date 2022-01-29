use serde_json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs, io::Write};

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct People(Vec<Person>);

pub fn read_json_example() -> Result<()> {
    let filenames = vec![
        "./data/johndoe.json",
        "./data/suzenflit.json",
    ];
    let outfilename = "./data/output.json";

    let mut people = People(vec![]);
    for filename in filenames{
        let data = fs::read_to_string(filename).unwrap();
        let p: Person = serde_json::from_str(data.as_str())?;
        people.0.push(p.clone());
    }

    println!("{:#?}", serde_json::to_string(&people));
    let mut outfile = fs::File::create(outfilename).unwrap();
    let contents = serde_json::to_string_pretty(&people).unwrap();
    outfile.write(contents.as_bytes()).unwrap();

    Ok(())
}
