use serde_json::json;

#[derive(Debug)]
pub struct Verb {
    data: Data,
}

impl Verb {
    pub fn create_table(table_name: &str) -> String {
        let mut headings: Vec<String> = vec!["infinitive TEXT".to_string()];
        const PERSONS: [&str; 6] = [
            "first_singular",
            "second_singular",
            "third_singular",
            "first_plural",
            "second_plural",
            "third_plural",
        ];
        const TENSES: [&str; 2] = ["present", "past"];
        for tense in TENSES.iter() {
            for positivity in 0..2 {
                for person in PERSONS.iter() {
                    // TODO: no trailing comma
                    headings.push(format!(
                        "{}{}_{} TEXT",
                        person,
                        if positivity == 0 { "" } else { "_negative" },
                        tense
                    ));
                }
            }
        }

        vec![
            format!("CREATE TABLE {} (", table_name),
            headings.join(", "),
            " );".to_string(),
        ]
        .join("")
    }

    pub fn new(word_data: serde_json::Value) -> Result<Self, &'static str> {
        Ok(Self {
            data: Data::from_json(word_data)?,
        })
    }

    pub fn db_row(&self) -> String {
        format!(
            "INSERT INTO verbs VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
            self.infinitive(),
            self.form().person(Person::FirstSingular).tense(Tense::Present),
            self.form().person(Person::SecondSingular).tense(Tense::Present),
            self.form().person(Person::ThirdSingular).tense(Tense::Present),
            self.form().person(Person::FirstPlural).tense(Tense::Present),
            self.form().person(Person::SecondPlural).tense(Tense::Present),
            self.form().person(Person::ThirdPlural).tense(Tense::Present),
            self.form().negative().person(Person::FirstSingular).tense(Tense::Present),
            self.form().negative().person(Person::SecondSingular).tense(Tense::Present),
            self.form().negative().person(Person::ThirdSingular).tense(Tense::Present),
            self.form().negative().person(Person::FirstPlural).tense(Tense::Present),
            self.form().negative().person(Person::SecondPlural).tense(Tense::Present),
            self.form().negative().person(Person::ThirdPlural).tense(Tense::Present),
            self.form().person(Person::FirstSingular).tense(Tense::Past),
            self.form().person(Person::SecondSingular).tense(Tense::Past),
            self.form().person(Person::ThirdSingular).tense(Tense::Past),
            self.form().person(Person::FirstPlural).tense(Tense::Past),
            self.form().person(Person::SecondPlural).tense(Tense::Past),
            self.form().person(Person::ThirdPlural).tense(Tense::Past),
            self.form().negative().person(Person::FirstSingular).tense(Tense::Past),
            self.form().negative().person(Person::SecondSingular).tense(Tense::Past),
            self.form().negative().person(Person::ThirdSingular).tense(Tense::Past),
            self.form().negative().person(Person::FirstPlural).tense(Tense::Past),
            self.form().negative().person(Person::SecondPlural).tense(Tense::Past),
            self.form().negative().person(Person::ThirdPlural).tense(Tense::Past),
        )
    }

    pub fn form(&self) -> NormalVerb {
        NormalVerb {
            word_data: &self.data,
            // This defines default values
            positive: true,
            person: Person::FirstSingular,
            tense: Tense::Present,
        }
    }

    pub fn infinitive(&self) -> Infinitive {
        Infinitive {
            word_data: &self.data,
        }
    }
}

#[derive(Clone)]
pub struct NormalVerb<'a> {
    word_data: &'a Data,
    positive: bool,
    person: Person,
    tense: Tense,
}

impl NormalVerb<'_> {
    pub fn negative(&self) -> Self {
        let mut new = self.clone();
        new.positive = false;
        new
    }

    pub fn person(&self, person: Person) -> Self {
        let mut new = self.clone();
        new.person = person;
        new
    }

    pub fn tense(&self, tense: Tense) -> Self {
        let mut new = self.clone();
        new.tense = tense;
        new
    }
}

impl std::fmt::Display for NormalVerb<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}",
            self.word_data.inflection(Kind::Normal {
                positive: self.positive,
                person: &self.person,
                tense: &self.tense,
            })
        )
    }
}

pub struct Infinitive<'a> {
    word_data: &'a Data,
}

impl std::fmt::Display for Infinitive<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.word_data.inflection(Kind::Infinitive))
    }
}

#[derive(Clone)]
pub enum Person {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
}

#[derive(Clone)]
pub enum Tense {
    Present,
    Past,
}

enum Kind<'a> {
    Infinitive,
    Normal {
        positive: bool,
        person: &'a Person,
        tense: &'a Tense,
    },
}

#[derive(Debug)]
struct Data {
    present: Inflections,
    past: Inflections,
    infinitive: String,
}

impl Data {
    fn from_json(word_data: serde_json::Value) -> Result<Data, &'static str> {
        if word_data["pos"] != "verb" || word_data["head_templates"][0]["args"]["2"] != "verb" {
            return Err("word isn't the dictionary verb");
        }
        let Some(infinitive) = &word_data["head_templates"][0]["expansion"].as_str() else {
            return Err("no infinitive form");
        };
        let infinitive: String = infinitive.to_string();
        let mut present: Inflections = Inflections {
            positive: Default::default(),
            negative: Default::default(),
        };
        let mut past: Inflections = Inflections {
            positive: Default::default(),
            negative: Default::default(),
        };

        const PERSON_TAGS: [&str; 3] = ["first-person", "second-person", "third-person"];
        const PLURAL_TAGS: [&str; 2] = ["singular", "plural"];
        const NEGATIVE_TAG: &str = "negative";
        const TENSE_TAGS: [&str; 2] = ["present", "past"];
        const MOOD_TAG: &str = "indicative";

        let Some(forms): Option<&Vec<_>> = word_data["forms"].as_array() else {
            return Err("no forms");
        };

        for tense in TENSE_TAGS.iter() {
            let mut positive: [String; 6] = Default::default();
            let mut negative: [String; 6] = Default::default();
            for positivity in 0..2 {
                let mut words: [String; 6] = Default::default();
                let mut i = 0;
                for plural in PLURAL_TAGS.iter() {
                    for person in PERSON_TAGS.iter() {
                        let tags =
                            std::collections::HashSet::from([MOOD_TAG, tense, plural, person]);
                        words[i] = forms
                            .iter()
                            .find(|&form| {
                                let empty_vec = vec![];
                                let form_tags = std::collections::HashSet::from_iter(
                                    form["tags"]
                                        .as_array()
                                        .unwrap_or(&empty_vec)
                                        .iter()
                                        .filter_map(|value| value.as_str()),
                                );
                                form_tags.is_superset(&tags)
                                    && (if positivity == 0 {
                                        !form_tags.contains(NEGATIVE_TAG)
                                    } else {
                                        form_tags.contains(NEGATIVE_TAG)
                                    })
                            })
                            .unwrap_or(&json!(null))["form"]
                            .as_str()
                            .unwrap_or("-")
                            .to_string();
                        i += 1;
                    }
                }
                if positivity == 0 {
                    positive = words;
                } else {
                    negative = words;
                }
            }
            if tense == &"present" {
                present = Inflections { positive, negative };
            } else {
                past = Inflections { positive, negative }
            }
        }

        Ok(Data {
            present,
            past,
            infinitive,
        })
    }

    fn inflection(&self, kind: Kind) -> String {
        match kind {
            Kind::Infinitive => self.infinitive.clone(),
            Kind::Normal {
                positive,
                person,
                tense,
            } => match tense {
                Tense::Present => self.present.inflection(positive, person),
                Tense::Past => self.past.inflection(positive, person),
            },
        }
    }
}

#[derive(Debug)]
struct Inflections {
    positive: [String; 6],
    negative: [String; 6],
}

impl Inflections {
    fn inflection(&self, positive: bool, person: &Person) -> String {
        if positive {
            match person {
                Person::FirstSingular => self.positive[0].clone(),
                Person::SecondSingular => self.positive[1].clone(),
                Person::ThirdSingular => self.positive[2].clone(),
                Person::FirstPlural => self.positive[3].clone(),
                Person::SecondPlural => self.positive[4].clone(),
                Person::ThirdPlural => self.positive[5].clone(),
            }
        } else {
            match person {
                Person::FirstSingular => self.negative[0].clone(),
                Person::SecondSingular => self.negative[1].clone(),
                Person::ThirdSingular => self.negative[2].clone(),
                Person::FirstPlural => self.negative[3].clone(),
                Person::SecondPlural => self.negative[4].clone(),
                Person::ThirdPlural => self.negative[5].clone(),
            }
        }
    }
}
