use crate::inflection;
use crate::kaikki;
use crate::table::Table;

pub fn extract(words: &Vec<kaikki::Sana>) -> Vec<Verb> {
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

#[derive(Debug, Default)]
struct Data {
    first_sing_pres_ind: String,
    second_sing_pres_ind: String,
    third_sing_pres_ind: String,
    first_plur_pres_ind: String,
    second_plur_pres_ind: String,
    third_plur_pres_ind: String,
    first_sing_past_ind: String,
    second_sing_past_ind: String,
    third_sing_past_ind: String,
    first_plur_past_ind: String,
    second_plur_past_ind: String,
    third_plur_past_ind: String,
    neg_first_sing_pres_ind: String,
    neg_second_sing_pres_ind: String,
    neg_third_sing_pres_ind: String,
    neg_first_plur_pres_ind: String,
    neg_second_plur_pres_ind: String,
    neg_third_plur_pres_ind: String,
    neg_first_sing_past_ind: String,
    neg_second_sing_past_ind: String,
    neg_third_sing_past_ind: String,
    neg_first_plur_past_ind: String,
    neg_second_plur_past_ind: String,
    neg_third_plur_past_ind: String,

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

        let Some(forms): Option<&Vec<kaikki::Form>> = word_data.forms() else {
            return Err("no forms");
        };

        let mut data: Data = Default::default();

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

        data.infinitive = infinitive.into();
        data.conjugation = conjugation;
        data.gradation = gradation;

        for form in forms.iter() {
            let tags = form.tags();
            let mut tags = tags.iter().collect::<Vec<_>>();
            tags.sort();
            match tags[..] {
                [&"first-person", &"indicative", &"present", &"singular"] => {
                    data.first_sing_pres_ind = form.name().into();
                }
                [&"indicative", &"present", &"second-person", &"singular"] => {
                    data.second_sing_pres_ind = form.name().into();
                }
                [&"indicative", &"present", &"singular", &"third-person"] => {
                    data.third_sing_pres_ind = form.name().into();
                }
                [&"first-person", &"indicative", &"plural", &"present"] => {
                    data.first_plur_pres_ind = form.name().into();
                }
                [&"indicative", &"plural", &"present", &"second-person"] => {
                    data.second_plur_pres_ind = form.name().into();
                }
                [&"indicative", &"plural", &"present", &"third-person"] => {
                    data.third_plur_pres_ind = form.name().into();
                }
                [&"first-person", &"indicative", &"past", &"singular"] => {
                    data.first_sing_past_ind = form.name().into();
                }
                [&"indicative", &"past", &"second-person", &"singular"] => {
                    data.second_sing_past_ind = form.name().into();
                }
                [&"indicative", &"past", &"singular", &"third-person"] => {
                    data.third_sing_past_ind = form.name().into();
                }
                [&"first-person", &"indicative", &"past", &"plural"] => {
                    data.first_plur_past_ind = form.name().into();
                }
                [&"indicative", &"past", &"plural", &"second-person"] => {
                    data.second_plur_past_ind = form.name().into();
                }
                [&"indicative", &"past", &"plural", &"third-person"] => {
                    data.third_plur_past_ind = form.name().into();
                }

                [&"first-person", &"indicative", &"negative", &"present", &"singular"] => {
                    data.neg_first_sing_pres_ind = form.name().into();
                }
                [&"indicative", &"negative", &"present", &"second-person", &"singular"] => {
                    data.neg_second_sing_pres_ind = form.name().into();
                }
                [&"indicative", &"negative", &"present", &"singular", &"third-person"] => {
                    data.neg_third_sing_pres_ind = form.name().into();
                }
                [&"first-person", &"indicative", &"negative", &"plural", &"present"] => {
                    data.neg_first_plur_pres_ind = form.name().into();
                }
                [&"indicative", &"negative", &"plural", &"present", &"second-person"] => {
                    data.neg_second_plur_pres_ind = form.name().into();
                }
                [&"indicative", &"negative", &"plural", &"present", &"third-person"] => {
                    data.neg_third_plur_pres_ind = form.name().into();
                }
                [&"first-person", &"indicative", &"negative", &"past", &"singular"] => {
                    data.neg_first_sing_past_ind = form.name().into();
                }
                [&"indicative", &"negative", &"past", &"second-person", &"singular"] => {
                    data.neg_second_sing_past_ind = form.name().into();
                }
                [&"indicative", &"negative", &"past", &"singular", &"third-person"] => {
                    data.neg_third_sing_past_ind = form.name().into();
                }
                [&"first-person", &"indicative", &"negative", &"past", &"plural"] => {
                    data.neg_first_plur_past_ind = form.name().into();
                }
                [&"indicative", &"negative", &"past", &"plural", &"second-person"] => {
                    data.neg_second_plur_past_ind = form.name().into();
                }
                [&"indicative", &"negative", &"past", &"plural", &"third-person"] => {
                    data.neg_third_plur_past_ind = form.name().into();
                }
                _ => {}
            }
        }

        Ok(data)
    }

    fn inflection(&self, kind: Kind) -> String {
        match kind {
            Kind::Infinitive => self.infinitive.clone(),
            Kind::Normal {
                positive,
                person,
                tense,
            } => match tense {
                Tense::Present => {
                    if positive {
                        match person {
                            Person::FirstSingular => self.first_sing_pres_ind.clone(),
                            Person::SecondSingular => self.second_sing_pres_ind.clone(),
                            Person::ThirdSingular => self.third_sing_pres_ind.clone(),
                            Person::FirstPlural => self.first_plur_pres_ind.clone(),
                            Person::SecondPlural => self.second_plur_pres_ind.clone(),
                            Person::ThirdPlural => self.third_plur_pres_ind.clone(),
                        }
                    } else {
                        match person {
                            Person::FirstSingular => self.neg_first_sing_pres_ind.clone(),
                            Person::SecondSingular => self.neg_second_sing_pres_ind.clone(),
                            Person::ThirdSingular => self.neg_third_sing_pres_ind.clone(),
                            Person::FirstPlural => self.neg_first_plur_pres_ind.clone(),
                            Person::SecondPlural => self.neg_second_plur_pres_ind.clone(),
                            Person::ThirdPlural => self.neg_third_plur_pres_ind.clone(),
                        }
                    }
                }
                Tense::Past => {
                    if positive {
                        match person {
                            Person::FirstSingular => self.first_sing_past_ind.clone(),
                            Person::SecondSingular => self.second_sing_past_ind.clone(),
                            Person::ThirdSingular => self.third_sing_past_ind.clone(),
                            Person::FirstPlural => self.first_plur_past_ind.clone(),
                            Person::SecondPlural => self.second_plur_past_ind.clone(),
                            Person::ThirdPlural => self.third_plur_past_ind.clone(),
                        }
                    } else {
                        match person {
                            Person::FirstSingular => self.neg_first_sing_past_ind.clone(),
                            Person::SecondSingular => self.neg_second_sing_past_ind.clone(),
                            Person::ThirdSingular => self.neg_third_sing_past_ind.clone(),
                            Person::FirstPlural => self.neg_first_plur_past_ind.clone(),
                            Person::SecondPlural => self.neg_second_plur_past_ind.clone(),
                            Person::ThirdPlural => self.neg_third_plur_past_ind.clone(),
                        }
                    }
                }
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
