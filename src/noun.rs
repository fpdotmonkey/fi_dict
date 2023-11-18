use crate::inflection;
use crate::kaikki;
use crate::table::Table;

pub fn extract(words: &Vec<kaikki::Sana>) -> Vec<Noun> {
    words
        .iter()
        .filter_map(|word| Noun::new(word).ok())
        .collect()
}

pub fn create_table(table_name: &'static str, nouns: Vec<Noun>) -> String {
    Noun::create_table_with_data(table_name, nouns)
}

#[derive(Debug, Default)]
pub struct Noun {
    inner: Data,

    inflection: Option<inflection::Conjugation>,
    gradation: Option<inflection::Gradation>,
}

impl Noun {
    fn new(word_data: &kaikki::Sana) -> Result<Self, &'static str> {
        let data = Data::from_sana(word_data)?;
        Ok(Self {
            inner: data,
            ..Default::default()
        })
    }
}

impl Table for Noun {
    fn columns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("nominative", "TEXT"),
            ("nominative_plural", "TEXT"),
            ("genitive", "TEXT"),
            ("genitive_plural", "TEXT"),
            ("partitive", "TEXT"),
            ("partitive_plural", "TEXT"),
            ("inessive", "TEXT"),
            ("inessive_plural", "TEXT"),
            ("elative", "TEXT"),
            ("elative_plural", "TEXT"),
            ("illative", "TEXT"),
            ("illative_plural", "TEXT"),
            ("adessive", "TEXT"),
            ("adessive_plural", "TEXT"),
            ("ablative", "TEXT"),
            ("ablative_plural", "TEXT"),
            ("allative", "TEXT"),
            ("allative_plural", "TEXT"),
            ("essive", "TEXT"),
            ("essive_plural", "TEXT"),
            ("translative", "TEXT"),
            ("translative_plural", "TEXT"),
            ("inflection", "INT"),
            ("gradation", "INT"),
        ]
    }
    fn row(&self) -> Vec<std::string::String> {
        vec![
            self.inner.nominative().clone(),
            self.inner.nominative_plural().clone(),
            self.inner.genitive().clone(),
            self.inner.genitive_plural().clone(),
            self.inner.partitive().clone(),
            self.inner.partitive_plural().clone(),
            self.inner.inessive().clone(),
            self.inner.inessive_plural().clone(),
            self.inner.elative().clone(),
            self.inner.elative_plural().clone(),
            self.inner.illative().clone(),
            self.inner.illative_plural().clone(),
            self.inner.adessive().clone(),
            self.inner.adessive_plural().clone(),
            self.inner.ablative().clone(),
            self.inner.ablative_plural().clone(),
            self.inner.allative().clone(),
            self.inner.allative_plural().clone(),
            self.inner.essive().clone(),
            self.inner.essive_plural().clone(),
            self.inner.translative().clone(),
            self.inner.translative_plural().clone(),
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

    fn foreign_keys() -> Option<Vec<(&'static str, &'static str, &'static str)>> {
        Some(vec![
            ("inflection", "inflections", "kotus_type"),
            ("gradation", "gradations", "kotus_type"),
        ])
    }
}

#[derive(Default, Debug)]
struct Data {
    nominative: String, // mikä
    nominative_plural: String,
    genitive: String, // minkä
    genitive_plural: String,
    partitive: String, // mitä
    partitive_plural: String,
    inessive: String, // missä
    inessive_plural: String,
    elative: String, // mistä
    elative_plural: String,
    illative: String, // mihin
    illative_plural: String,
    adessive: String, // millä
    adessive_plural: String,
    ablative: String, // miltä
    ablative_plural: String,
    allative: String, // mille
    allative_plural: String,
    essive: String, // minä
    essive_plural: String,
    translative: String, // miksi
    translative_plural: String,
}

impl Data {
    fn from_sana(word_data: &kaikki::Sana) -> Result<Self, &'static str> {
        if !word_data.is_nominal() {
            return Err("word isn't the dictionary word");
        }
        let Some(_) = word_data.expansion() else {
            return Err("no basic form");
        };

        let Some(forms): Option<&Vec<kaikki::Form>> = word_data.forms() else {
            return Err("no forms");
        };

        let mut data: Data = Default::default();

        for form in forms.iter() {
            if form.tags().contains("comitative") {
                // need to exit before the start of the posessive tables
                break;
            }
            let tags = form.tags();
            let mut tags = tags.iter().collect::<Vec<_>>();
            tags.sort();
            // println!("{} {:?}", form.name(), tags);
            match tags[..] {
                [&"nominative", &"plural"] => {
                    if data.nominative_plural.is_empty() {
                        data.nominative_plural = form.name().into()
                    }
                }
                [&"nominative", &"singular"] => {
                    if data.nominative.is_empty() {
                        data.nominative = form.name().into()
                    }
                }
                [&"genitive", &"plural"] => {
                    if data.genitive_plural.is_empty() {
                        data.genitive_plural = form.name().into()
                    }
                }
                [&"genitive", &"singular"] => {
                    if data.genitive.is_empty() {
                        data.genitive = form.name().into()
                    }
                }
                [&"partitive", &"plural"] => {
                    if data.partitive_plural.is_empty() {
                        data.partitive_plural = form.name().into()
                    }
                }
                [&"partitive", &"singular"] => {
                    if data.partitive.is_empty() {
                        data.partitive = form.name().into()
                    }
                }
                [&"inessive", &"plural"] => {
                    if data.inessive_plural.is_empty() {
                        data.inessive_plural = form.name().into()
                    }
                }
                [&"inessive", &"singular"] => {
                    if data.inessive.is_empty() {
                        data.inessive = form.name().into()
                    }
                }
                [&"elative", &"plural"] => {
                    if data.elative_plural.is_empty() {
                        data.elative_plural = form.name().into()
                    }
                }
                [&"elative", &"singular"] => {
                    if data.elative.is_empty() {
                        data.elative = form.name().into()
                    }
                }
                [&"illative", &"plural"] => {
                    if data.illative_plural.is_empty() {
                        data.illative_plural = form.name().into()
                    }
                }
                [&"illative", &"singular"] => {
                    if data.illative.is_empty() {
                        data.illative = form.name().into()
                    }
                }
                [&"adessive", &"plural"] => {
                    if data.adessive_plural.is_empty() {
                        data.adessive_plural = form.name().into()
                    }
                }
                [&"adessive", &"singular"] => {
                    if data.adessive.is_empty() {
                        data.adessive = form.name().into()
                    }
                }
                [&"ablative", &"plural"] => {
                    if data.ablative_plural.is_empty() {
                        data.ablative_plural = form.name().into()
                    }
                }
                [&"ablative", &"singular"] => {
                    if data.ablative.is_empty() {
                        data.ablative = form.name().into()
                    }
                }
                [&"allative", &"plural"] => {
                    if data.allative_plural.is_empty() {
                        data.allative_plural = form.name().into()
                    }
                }
                [&"allative", &"singular"] => {
                    if data.allative.is_empty() {
                        data.allative = form.name().into()
                    }
                }
                [&"essive", &"plural"] => {
                    if data.essive_plural.is_empty() {
                        data.essive_plural = form.name().into()
                    }
                }
                [&"essive", &"singular"] => {
                    if data.essive.is_empty() {
                        data.essive = form.name().into()
                    }
                }
                [&"plural", &"translative"] => {
                    if data.translative_plural.is_empty() {
                        data.translative_plural = form.name().into()
                    }
                }
                [&"singular", &"translative"] => {
                    if data.translative.is_empty() {
                        data.translative = form.name().into()
                    }
                }
                _ => {}
            }
        }

        Ok(data)
    }

    fn nominative(&self) -> &String {
        &self.nominative
    }
    fn nominative_plural(&self) -> &String {
        &self.nominative_plural
    }
    fn genitive(&self) -> &String {
        &self.genitive
    }
    fn genitive_plural(&self) -> &String {
        &self.genitive_plural
    }
    fn partitive(&self) -> &String {
        &self.partitive
    }
    fn partitive_plural(&self) -> &String {
        &self.partitive_plural
    }
    fn inessive(&self) -> &String {
        &self.inessive
    }
    fn inessive_plural(&self) -> &String {
        &self.inessive_plural
    }
    fn elative(&self) -> &String {
        &self.elative
    }
    fn elative_plural(&self) -> &String {
        &self.elative_plural
    }
    fn illative(&self) -> &String {
        &self.illative
    }
    fn illative_plural(&self) -> &String {
        &self.illative_plural
    }
    fn adessive(&self) -> &String {
        &self.adessive
    }
    fn adessive_plural(&self) -> &String {
        &self.adessive_plural
    }
    fn ablative(&self) -> &String {
        &self.ablative
    }
    fn ablative_plural(&self) -> &String {
        &self.ablative_plural
    }
    fn allative(&self) -> &String {
        &self.allative
    }
    fn allative_plural(&self) -> &String {
        &self.allative_plural
    }
    fn essive(&self) -> &String {
        &self.essive
    }
    fn essive_plural(&self) -> &String {
        &self.essive_plural
    }
    fn translative(&self) -> &String {
        &self.translative
    }
    fn translative_plural(&self) -> &String {
        &self.translative_plural
    }
}
