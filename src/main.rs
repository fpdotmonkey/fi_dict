#[doc(inline)]
pub use std;

mod inflection;
mod kaikki;
mod verb;

use std::io::BufRead;

fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    const FI_DICT: &str = "kaikki.org-dictionary-Finnish.json";

    let file = std::fs::File::open(FI_DICT).unwrap();
    let reader = std::io::BufReader::new(file);

    println!("parsing from JSON");
    let words: Vec<kaikki::Sana> = reader
        .lines()
        // .take(160)
        .filter_map(|word_data| kaikki::Sana::from_json(&word_data.unwrap()))
        .collect();

    let verbs: Vec<verb::Verb> = words
        .iter()
        .filter_map(|word| verb::Verb::new(word).ok())
        .collect();

    println!(
        "parsing: {}s, {:.0}ms/word",
        now.elapsed().as_secs(),
        1000.0 * now.elapsed().as_secs_f64() / verbs.len() as f64
    );
    let now: std::time::Instant = std::time::Instant::now();

    match std::fs::remove_file("verbit.sqlite") {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("{}", error),
    }

    let verb_query: String = vec![
        verb::Verb::create_table("verbs"),
        verbs
            .iter()
            .map(|verb| verb.db_row())
            .collect::<Vec<String>>()
            .join(", "),
        ";".to_string(),
    ]
    .join("");

    let connection = sqlite::open("verbit.sqlite").unwrap();
    connection.execute(verb_query).unwrap();
    connection
        .execute(inflection::create_table("inflections"))
        .unwrap();
    connection
        .execute(inflection::gradation_table("gradations"))
        .unwrap();

    println!("SQLite generation: {}ms", now.elapsed().as_millis());
}
