mod csvtojson;

use csvtojson::convert_csv_to_json;

fn main() {
    convert_csv_to_json().map_err(|err| println!("{:?}", err)).ok();
}
