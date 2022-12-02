extern crate csv;
use serde::{Serialize};
use std::error::Error;

#[derive(Serialize, Debug)]
struct RecordJson {
    id: i32,
    name: String,
    age: i32
}

    pub fn convert_csv_to_json()-> Result<(), Box<dyn Error>> {

        let mut csv_vector: Vec<RecordJson> = Vec::new();
        let rdr = csv::ReaderBuilder::new().from_path("sample.csv");

        for result in rdr?.records() {
            let record = result?;
            let csv_json = RecordJson {
                id: record[0].trim().parse::<i32>().unwrap(),
                name: record[1].trim().to_string(),
                age: record[2].trim().parse::<i32>().unwrap(),
            };

            csv_vector.push(csv_json);
        }

        println!("{:?}", csv_vector);
        Ok(())
    }
