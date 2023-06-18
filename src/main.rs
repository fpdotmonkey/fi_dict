#[doc(inline)]
pub use std;

mod verb;

use std::io::BufRead;
use verb::Verb;

fn main() {
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

    let mut query: Vec<String> = vec![Verb::create_table("verbs")];

    for verb in verbs {
        query.push(verb.db_row())
    }

    let query: String = query.join("\n");
    let connection = sqlite::open("verbit.sqlite").unwrap();
    connection.execute(query).unwrap();

    let query = "SELECT * FROM verbs";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        println!(
            "infinitive = {}",
            statement.read::<String, _>("infinitive").unwrap()
        );
        println!(
            "second_singular_negative_past = {}",
            statement
                .read::<String, _>("second_singular_negative_past")
                .unwrap()
        );
        println!();
    }
}
