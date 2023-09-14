use crate::inflection;
use crate::kaikki;
use crate::table::Table;

pub fn extract(words: Vec<kaikki::Sana>) -> Vec<Verb> {
    words
        .iter()
        .filter_map(|word| Verb::new(word).ok())
        .collect()
}

pub fn create_table(table_name: &'static str, verbs: Vec<Verb>) -> String {
    Verb::create_table_with_data(table_name, verbs)
}

#[derive(Debug)]
pub struct Verb {
    infinitive: String,
    first_singular_present: String,
    second_singular_present: String,
    third_singular_present: String,
    first_plural_present: String,
    second_plural_present: String,
    third_plural_present: String,
    first_singular_negative_present: String,
    second_singular_negative_present: String,
    third_singular_negative_present: String,
    first_plural_negative_present: String,
    second_plural_negative_present: String,
    third_plural_negative_present: String,
    first_singular_past: String,
    second_singular_past: String,
    third_singular_past: String,
    first_plural_past: String,
    second_plural_past: String,
    third_plural_past: String,
    first_singular_negative_past: String,
    second_singular_negative_past: String,
    third_singular_negative_past: String,
    first_plural_negative_past: String,
    second_plural_negative_past: String,
    third_plural_negative_past: String,

    inflection: Option<inflection::Conjugation>,
    gradation: Option<inflection::Gradation>,
}

impl Verb {
    fn new(word_data: &kaikki::Sana) -> Result<Self, &'static str> {
        let data = Data::from_sana(word_data)?;
        Ok(Self {
            infinitive: data.infinitive().to_string(),
            first_singular_present: data
                .form()
                .person(Person::FirstSingular)
                .tense(Tense::Present)
                .to_string(),
            second_singular_present: data
                .form()
                .person(Person::SecondSingular)
                .tense(Tense::Present)
                .to_string(),
            third_singular_present: data
                .form()
                .person(Person::ThirdSingular)
                .tense(Tense::Present)
                .to_string(),
            first_plural_present: data
                .form()
                .person(Person::FirstPlural)
                .tense(Tense::Present)
                .to_string(),
            second_plural_present: data
                .form()
                .person(Person::SecondPlural)
                .tense(Tense::Present)
                .to_string(),
            third_plural_present: data
                .form()
                .person(Person::ThirdPlural)
                .tense(Tense::Present)
                .to_string(),
            first_singular_negative_present: data
                .form()
                .negative()
                .person(Person::FirstSingular)
                .tense(Tense::Present)
                .to_string(),
            second_singular_negative_present: data
                .form()
                .negative()
                .person(Person::SecondSingular)
                .tense(Tense::Present)
                .to_string(),
            third_singular_negative_present: data
                .form()
                .negative()
                .person(Person::ThirdSingular)
                .tense(Tense::Present)
                .to_string(),
            first_plural_negative_present: data
                .form()
                .negative()
                .person(Person::FirstPlural)
                .tense(Tense::Present)
                .to_string(),
            second_plural_negative_present: data
                .form()
                .negative()
                .person(Person::SecondPlural)
                .tense(Tense::Present)
                .to_string(),
            third_plural_negative_present: data
                .form()
                .negative()
                .person(Person::ThirdPlural)
                .tense(Tense::Present)
                .to_string(),
            first_singular_past: data
                .form()
                .person(Person::FirstSingular)
                .tense(Tense::Past)
                .to_string(),
            second_singular_past: data
                .form()
                .person(Person::SecondSingular)
                .tense(Tense::Past)
                .to_string(),
            third_singular_past: data
                .form()
                .person(Person::ThirdSingular)
                .tense(Tense::Past)
                .to_string(),
            first_plural_past: data
                .form()
                .person(Person::FirstPlural)
                .tense(Tense::Past)
                .to_string(),
            second_plural_past: data
                .form()
                .person(Person::SecondPlural)
                .tense(Tense::Past)
                .to_string(),
            third_plural_past: data
                .form()
                .person(Person::ThirdPlural)
                .tense(Tense::Past)
                .to_string(),
            first_singular_negative_past: data
                .form()
                .negative()
                .person(Person::FirstSingular)
                .tense(Tense::Past)
                .to_string(),
            second_singular_negative_past: data
                .form()
                .negative()
                .person(Person::SecondSingular)
                .tense(Tense::Past)
                .to_string(),
            third_singular_negative_past: data
                .form()
                .negative()
                .person(Person::ThirdSingular)
                .tense(Tense::Past)
                .to_string(),
            first_plural_negative_past: data
                .form()
                .negative()
                .person(Person::FirstPlural)
                .tense(Tense::Past)
                .to_string(),
            second_plural_negative_past: data
                .form()
                .negative()
                .person(Person::SecondPlural)
                .tense(Tense::Past)
                .to_string(),
            third_plural_negative_past: data
                .form()
                .negative()
                .person(Person::ThirdPlural)
                .tense(Tense::Past)
                .to_string(),

            inflection: data.conjugation,
            gradation: data.gradation,
        })
    }
}

impl Table for Verb {
    fn columns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("infinitive", "TEXT"),
            ("first_singular_present", "TEXT"),
            ("second_singular_present", "TEXT"),
            ("third_singular_present", "TEXT"),
            ("first_plural_present", "TEXT"),
            ("second_plural_present", "TEXT"),
            ("third_plural_present", "TEXT"),
            ("first_singular_negative_present", "TEXT"),
            ("second_singular_negative_present", "TEXT"),
            ("third_singular_negative_present", "TEXT"),
            ("first_plural_negative_present", "TEXT"),
            ("second_plural_negative_present", "TEXT"),
            ("third_plural_negative_present", "TEXT"),
            ("first_singular_past", "TEXT"),
            ("second_singular_past", "TEXT"),
            ("third_singular_past", "TEXT"),
            ("first_plural_past", "TEXT"),
            ("second_plural_past", "TEXT"),
            ("third_plural_past", "TEXT"),
            ("first_singular_negative_past", "TEXT"),
            ("second_singular_negative_past", "TEXT"),
            ("third_singular_negative_past", "TEXT"),
            ("first_plural_negative_past", "TEXT"),
            ("second_plural_negative_past", "TEXT"),
            ("third_plural_negative_past", "TEXT"),
            ("inflection", "INT"),
            ("gradation", "INT"),
        ]
    }
    fn row(&self) -> Vec<std::string::String> {
        vec![
            self.infinitive.to_string(),
            self.first_singular_present.to_string(),
            self.second_singular_present.to_string(),
            self.third_singular_present.to_string(),
            self.first_plural_present.to_string(),
            self.second_plural_present.to_string(),
            self.third_plural_present.to_string(),
            self.first_singular_negative_present.to_string(),
            self.second_singular_negative_present.to_string(),
            self.third_singular_negative_present.to_string(),
            self.first_plural_negative_present.to_string(),
            self.second_plural_negative_present.to_string(),
            self.third_plural_negative_present.to_string(),
            self.first_singular_past.to_string(),
            self.second_singular_past.to_string(),
            self.third_singular_past.to_string(),
            self.first_plural_past.to_string(),
            self.second_plural_past.to_string(),
            self.third_plural_past.to_string(),
            self.first_singular_negative_past.to_string(),
            self.second_singular_negative_past.to_string(),
            self.third_singular_negative_past.to_string(),
            self.first_plural_negative_past.to_string(),
            self.second_plural_negative_past.to_string(),
            self.third_plural_negative_past.to_string(),
            match &self.inflection {
                Some(inflection) => inflection.kotus_type().to_string(),
                None => "NULL".to_string(),
            },
            match &self.gradation {
                Some(gradation) => gradation.kotus_type().to_string(),
                None => "NULL".to_string(),
            },
        ]
    }

    fn foreign_keys() -> std::option::Option<Vec<(&'static str, &'static str, &'static str)>> {
        Some(vec![
            ("inflection", "inflections", "kotus_type"),
            ("gradation", "gradations", "kotus_type"),
        ])
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
    conjugation: Option<inflection::Conjugation>,
    gradation: Option<inflection::Gradation>,
}

impl Data {
    fn from_sana(word_data: &kaikki::Sana) -> Result<Data, &'static str> {
        if !word_data.is_a_verb() {
            return Err("word isn't the dictionary verb");
        }
        let Some(infinitive) = word_data.expansion() else {
            return Err("no infinitive form");
        };
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

        let Some(forms): Option<&Vec<kaikki::Form>> = word_data.forms() else {
            return Err("no forms");
        };

        let conjugation: Option<inflection::Conjugation> = match word_data.inflection_templates() {
            Some(templates) => {
                if templates.is_empty() {
                    None
                } else {
                    inflection::Conjugation::from_template(templates[0].name(), infinitive)
                }
            }
            None => None,
        };
        let mut gradation: Option<inflection::Gradation> = None;
        if let Some(conjugation_class) = forms.iter().find(|form| {
            form.source() == Some(&"conjugation".to_string()) && form.tags().contains("class")
        }) {
            gradation = inflection::Gradation::from_template(conjugation_class.name(), infinitive);
        }

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
                                form.tags().is_superset(&tags)
                                    && (if positivity == 0 {
                                        !form.tags().contains(NEGATIVE_TAG)
                                    } else {
                                        form.tags().contains(NEGATIVE_TAG)
                                    })
                            })
                            .map(|form| form.name().to_string())
                            .unwrap_or("-".to_string());
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
            infinitive: infinitive.to_string(),
            gradation,
            conjugation,
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

    pub fn form(&self) -> NormalVerb {
        NormalVerb {
            word_data: self,
            // This defines default values
            positive: true,
            person: Person::FirstSingular,
            tense: Tense::Present,
        }
    }

    pub fn infinitive(&self) -> Infinitive {
        Infinitive { word_data: self }
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
