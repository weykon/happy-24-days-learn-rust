use csv::Writer;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

pub fn main() {
    let dollar_films = vec![
        ("A Fistful of Dollars", "Rojo", 1964),
        ("For a Few Dollars More", "El Indio", 1965),
        ("The Good, the Bad and the Ugly", "Tuco", 1966),
    ];

    let path = "films_list.csv";

    let mut writer = Writer::from_path(path).unwrap();

    for row in dollar_films {
        writer.serialize(row).expect("CSV writer error");
    }

    writer.flush().expect("err");

}
