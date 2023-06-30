#[doc(inline)]
pub use std;

mod verb;

use std::io::BufRead;
use verb::Verb;

fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    const FI_DICT: &str = "kaikki.org-dictionary-Finnish.json";

    let file = std::fs::File::open(FI_DICT).unwrap();
    let reader = std::io::BufReader::new(file);

    let verbs: Vec<Verb> = reader
        .lines()
        .filter_map(|word_data| {
            verb::Verb::new(serde_json::from_str(&word_data.unwrap()).unwrap()).ok()
        })
        // .take(160)
        .collect();

    println!("JSON parsing: {}s", now.elapsed().as_secs());
    let now: std::time::Instant = std::time::Instant::now();

    match std::fs::remove_file("verbit.sqlite") {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("{}", error),
    }

    let query: String = vec![
        Verb::create_table("verbs"),
        verbs
            .iter()
            .map(|verb| verb.db_row())
            .collect::<Vec<String>>()
            .join(", "),
        ";".to_string(),
    ]
    .join("");

    let connection = sqlite::open("verbit.sqlite").unwrap();
    connection.execute(query).unwrap();

    println!("SQLite generation: {}ms", now.elapsed().as_millis());
}
