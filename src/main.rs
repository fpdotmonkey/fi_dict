#[doc(inline)]
pub use std;

mod inflection;
mod kaikki;
mod table;
mod verb;

use crate::table::Table;

use std::io::BufRead;

fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    const FI_DICT: &str = "kaikki.org-dictionary-Finnish.json";

    let file = std::fs::File::open(FI_DICT).unwrap();
    let reader = std::io::BufReader::new(file);

    println!("parsing JSON");
    let words: Vec<kaikki::Sana> = reader
        .lines()
        // .take(160)
        .filter_map(|word_data| kaikki::Sana::from_json(word_data.unwrap()))
        .collect();
    println!(
        "JSON parsing: {}s, {:.3}ms/word",
        now.elapsed().as_secs(),
        1000.0 * now.elapsed().as_secs_f64() / words.len() as f64
    );

    println!(
        "{:?}",
        words
            .iter()
            .find(|word| word.expansion() == Some(&"puhua".to_string()))
    );

    println!("parsing verb data");
    let now: std::time::Instant = std::time::Instant::now();

    let verbs = verb::extract(words);
    println!(
        "Verb parsing: {}s, {:.3}ms/word",
        now.elapsed().as_secs(),
        1000.0 * now.elapsed().as_secs_f64() / verbs.len() as f64
    );

    match std::fs::remove_file("verbit.sqlite") {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("{}", error),
    }
    let now: std::time::Instant = std::time::Instant::now();

    let connection = sqlite::open("verbit.sqlite").unwrap();
    match connection.execute(inflection::create_table("inflections")) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", inflection::create_table("inflections"));
            panic!("{}", error)
        }
    };
    match connection.execute(inflection::gradation_table("gradations")) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", inflection::gradation_table("gradations"));
            panic!("{}", error)
        }
    };
    match connection.execute(verb::create_table("verbs", verbs)) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", verb::Verb::create_table("verbs"));
            panic!("{}", error)
        }
    };

    println!("Table generation: {}ms", now.elapsed().as_millis());
}
