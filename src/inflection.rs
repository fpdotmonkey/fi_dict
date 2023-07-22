use crate::table::Table;

pub fn create_table(table_name: &'static str) -> String {
    let inflections: Vec<ConjugationData> = vec![
        ConjugationData {
            conjugation: Conjugation::Sanoa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-[ouöy]a",
            hint_present: "-[ouöy]-",
            hint_past: "-[ouöy]i-",
        },
        ConjugationData {
            conjugation: Conjugation::Muistaa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-aa",
            hint_present: "-a-",
            hint_past: "-i-",
        },
        ConjugationData {
            conjugation: Conjugation::Huutaa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-taa",
            hint_present: "-ta- (KPT)",
            hint_past: "-si-",
        },
        ConjugationData {
            conjugation: Conjugation::Soutaa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-taa",
            hint_present: "-ta- (KPT)",
            hint_past: "-ti- (KPT)",
        },
        ConjugationData {
            conjugation: Conjugation::Kaivaa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-aa",
            hint_present: "-a-",
            hint_past: "-oi-",
        },
        ConjugationData {
            conjugation: Conjugation::Saartaa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-taa",
            hint_present: "-a-",
            hint_past: "-si-",
        },
        ConjugationData {
            conjugation: Conjugation::Laskea,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-ea",
            hint_present: "-e-",
            hint_past: "-i-",
        },
        ConjugationData {
            conjugation: Conjugation::Tuntea,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-ntea",
            hint_present: "-nte- (KPT)",
            hint_past: "-nsi-",
        },
        ConjugationData {
            conjugation: Conjugation::Lahtea,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-htea",
            hint_present: "-hde-",
            hint_past: "-hdi-",
        },
        ConjugationData {
            conjugation: Conjugation::Sallia,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-ia",
            hint_present: "-i-",
            hint_past: "-i-",
        },
        ConjugationData {
            conjugation: Conjugation::Voida,
            verb_type: "2",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-{vowel}ida",
            hint_present: "-{vowel}i-",
            hint_past: "-{vowel}i-",
        },
        ConjugationData {
            conjugation: Conjugation::Saada,
            verb_type: "2",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-{long vowel}da",
            hint_present: "-{long vowel}-",
            hint_past: "-{short vowel}i-",
        },
        ConjugationData {
            conjugation: Conjugation::Juoda,
            verb_type: "2",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-ieda, -uoda, -yödä",
            hint_present: "-ie-, -uo-, -yö-",
            hint_past: "-ei-, -oi-, -öi-",
        },
        ConjugationData {
            conjugation: Conjugation::Kayda,
            verb_type: "2",
            gradation_style: GradationStyle::None,
            hint_infinitive: "käydä",
            hint_present: "käy-",
            hint_past: "kävi-",
        },
        ConjugationData {
            conjugation: Conjugation::Rohkaista,
            verb_type: "3",
            gradation_style: GradationStyle::WeakStrong, // but most verbs don't gradate
            hint_infinitive: "-sta",
            hint_present: "-se-",
            hint_past: "-si-",
        },
        ConjugationData {
            conjugation: Conjugation::Tulla,
            verb_type: "3",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-lla, -nna, -rra",
            hint_present: "-le-, -ne-, -re-",
            hint_past: "-li-, -ni-, -ri-",
        },
        ConjugationData {
            conjugation: Conjugation::Tupakoida,
            verb_type: "2",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-oida",
            hint_present: "-oi-",
            hint_past: "-oi-",
        },
        ConjugationData {
            conjugation: Conjugation::Valita,
            verb_type: "5",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-ita",
            hint_present: "-itse-",
            hint_past: "-itsi-",
        },
        ConjugationData {
            conjugation: Conjugation::Juosta,
            verb_type: "3",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-sta",
            hint_present: "-kse-",
            hint_past: "-ksi-",
        },
        ConjugationData {
            conjugation: Conjugation::Nahda,
            verb_type: "2",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "nähdä, tehdä",
            hint_present: "näe-, tee- (KPT)",
            hint_past: "näi-, tei- (KPT)",
        },
        ConjugationData {
            conjugation: Conjugation::Vanheta,
            verb_type: "6",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-eta",
            hint_present: "-ene-",
            hint_past: "-eni-",
        },
        ConjugationData {
            conjugation: Conjugation::Salata,
            verb_type: "4",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-[aouäöy]ta",
            hint_present: "-[aouäöy]a-",
            hint_past: "-[aouäöy]si-",
        },
        ConjugationData {
            conjugation: Conjugation::Katketa,
            verb_type: "4",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-[eouöy]ta",
            hint_present: "-[eouöy]a-",
            hint_past: "-[eouöy]si-",
        },
        ConjugationData {
            conjugation: Conjugation::Selvita,
            verb_type: "4",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-[eiouöy]ta",
            hint_present: "-[eiouöy]a-",
            hint_past: "-[eiouöy]si-",
        },
        ConjugationData {
            conjugation: Conjugation::Taitaa,
            verb_type: "1",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "taitaa, teitää",
            hint_present: "-ta- (KPT)",
            hint_past: "-si-",
        },
        ConjugationData {
            conjugation: Conjugation::Kumajaa,
            verb_type: "uncommon",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-ajaa",
            hint_present: "-aja-",
            hint_past: "-asi-, -aji-",
        },
        ConjugationData {
            conjugation: Conjugation::Kaikaa,
            verb_type: "uncommon",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-kaa, -saa",
            hint_present: "-kaa-, -saa-",
            hint_past: "past tense is not used",
        },
        // Non-kotus inflections
        ConjugationData {
            conjugation: Conjugation::Olla,
            verb_type: "3",
            gradation_style: GradationStyle::None,
            hint_infinitive: "olla",
            hint_present: "ole-, on",
            hint_past: "oli-",
        },
        ConjugationData {
            conjugation: Conjugation::Taytya,
            verb_type: "täytyä",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-tyä",
            hint_present: "{genetive} -tyy",
            hint_past: "{genetive} -tyi",
        },
        ConjugationData {
            conjugation: Conjugation::Seista,
            verb_type: "seistä",
            gradation_style: GradationStyle::None,
            hint_infinitive: "seistä",
            hint_present: "seiso-",
            hint_past: "seisoi-",
        },
        ConjugationData {
            conjugation: Conjugation::Koraa,
            verb_type: "köraa",
            gradation_style: GradationStyle::None,
            hint_infinitive: "köraa",
            hint_present: "köraa-",
            hint_past: "körasi-",
        },
        ConjugationData {
            conjugation: Conjugation::Tarvii,
            verb_type: "tarvii",
            gradation_style: GradationStyle::None,
            hint_infinitive: "tarvii",
            hint_present: "tarvii-",
            hint_past: "(None)",
        },
        ConjugationData {
            conjugation: Conjugation::Tarttee,
            verb_type: "tarttee",
            gradation_style: GradationStyle::None,
            hint_infinitive: "tarttee",
            hint_present: "tartte-",
            hint_past: "tartti-",
        },
        ConjugationData {
            conjugation: Conjugation::Henkaa,
            verb_type: "henkää",
            gradation_style: GradationStyle::None,
            hint_infinitive: "henkää",
            hint_present: "henkä-",
            hint_past: "henkäsi-",
        },
    ];

    ConjugationData::create_table_with_data(table_name, inflections)
}

pub fn gradation_table(table_name: &'static str) -> String {
    let gradations: Vec<GradationData> = vec![
        GradationData {
            gradation: Gradation::None,
            strong: "",
            weak: "",
        },
        GradationData {
            gradation: Gradation::KkK,
            strong: "kk",
            weak: "k",
        },
        GradationData {
            gradation: Gradation::PpP,
            strong: "pp",
            weak: "p",
        },
        GradationData {
            gradation: Gradation::TtT,
            strong: "tt",
            weak: "t",
        },
        GradationData {
            gradation: Gradation::K0,
            strong: "k",
            weak: "",
        },
        GradationData {
            gradation: Gradation::PV,
            strong: "p",
            weak: "v",
        },
        GradationData {
            gradation: Gradation::TD,
            strong: "t",
            weak: "d",
        },
        GradationData {
            gradation: Gradation::NkNg,
            strong: "nk",
            weak: "ng",
        },
        GradationData {
            gradation: Gradation::MpMm,
            strong: "mp",
            weak: "mm",
        },
        GradationData {
            gradation: Gradation::LtLl,
            strong: "lt",
            weak: "ll",
        },
        GradationData {
            gradation: Gradation::NtNn,
            strong: "nt",
            weak: "nn",
        },
        GradationData {
            gradation: Gradation::RtRr,
            strong: "rt",
            weak: "rr",
        },
        GradationData {
            gradation: Gradation::KJ,
            strong: "k",
            weak: "j",
        },
        GradationData {
            gradation: Gradation::KV,
            strong: "k",
            weak: "v",
        },
        // Non-kotus gradations
        GradationData {
            gradation: Gradation::GgG,
            strong: "gg",
            weak: "g",
        },
        GradationData {
            gradation: Gradation::BbB,
            strong: "bb",
            weak: "b",
        },
        GradationData {
            gradation: Gradation::T0,
            strong: "t",
            weak: "",
        },
    ];

    GradationData::create_table_with_data(table_name, gradations)
}

#[derive(Debug, Clone)]
pub enum Conjugation {
    // kotus inflections
    Sanoa = 52,
    Muistaa = 53,
    Huutaa = 54,
    Soutaa = 55,
    Kaivaa = 56,
    Saartaa = 57,
    Laskea = 58,
    Tuntea = 59,
    Lahtea = 60,
    Sallia = 61,
    Voida = 62,
    Saada = 63,
    Juoda = 64,
    Kayda = 65,
    Rohkaista = 66,
    Tulla = 67,
    Tupakoida = 68,
    Valita = 69,
    Juosta = 70,
    Nahda = 71,
    Vanheta = 72,
    Salata = 73,
    Katketa = 74,
    Selvita = 75,
    Taitaa = 76,
    Kumajaa = 77,
    Kaikaa = 78,
    // non-kotus inflections
    Olla = 100,
    Taytya = 101,
    Seista = 102,
    Koraa = 103,
    Tarvii = 104,
    Tarttee = 105,
    Henkaa = 106,
}

impl Conjugation {
    pub fn from_template(template: &String, infinitive: &String) -> Option<Self> {
        match template.as_str() {
            "fi-conj-sanoa" => Some(Conjugation::Sanoa),
            "fi-conj-muistaa" => Some(Conjugation::Muistaa),
            "fi-conj-huutaa" => Some(Conjugation::Huutaa),
            "fi-conj-soutaa" => Some(Conjugation::Soutaa),
            "fi-conj-kaivaa" => Some(Conjugation::Kaivaa),
            "fi-conj-saartaa" => Some(Conjugation::Saartaa),
            "fi-conj-laskea" => Some(Conjugation::Laskea),
            "fi-conj-tuntea" => Some(Conjugation::Tuntea),
            "fi-conj-lähteä" => Some(Conjugation::Lahtea),
            "fi-conj-sallia" => Some(Conjugation::Sallia),
            "fi-conj-voida" => Some(Conjugation::Voida),
            "fi-conj-saada" => Some(Conjugation::Saada),
            "fi-conj-juoda" => Some(Conjugation::Juoda),
            "fi-conj-käydä" => Some(Conjugation::Kayda),
            "fi-conj-rohkaista" => Some(Conjugation::Rohkaista),
            "fi-conj-tulla" => Some(Conjugation::Tulla),
            "fi-conj-tupakoida" => Some(Conjugation::Tupakoida),
            "fi-conj-valita" => Some(Conjugation::Valita),
            "fi-conj-juosta" => Some(Conjugation::Juosta),
            "fi-conj-nähdä" => Some(Conjugation::Nahda),
            "fi-conj-vanheta" => Some(Conjugation::Vanheta),
            "fi-conj-salata" => Some(Conjugation::Salata),
            "fi-conj-katketa" => Some(Conjugation::Katketa),
            "fi-conj-selvitä" => Some(Conjugation::Selvita),
            "fi-conj-taitaa" => Some(Conjugation::Taitaa),
            "fi-conj-kumajaa" | "fi-conj-höpäjätä" => Some(Conjugation::Kumajaa),
            "fi-conj-kaikaa" => Some(Conjugation::Kaikaa),
            "fi-conj-olla" => Some(Conjugation::Olla),
            "fi-conj-seistä" => Some(Conjugation::Seista),
            // these templates note phrasal verbs where you need to
            // inflect non-verb words specially, which I skip for verbs
            "fi-infl-vp-accusative" => None,
            "fi-infl-vp-partitive" => None,
            "fi-conj-vp-impr" => None,
            // irregular verbs
            "fi-conj-table" => match infinitive.as_str() {
                "täytyä" | "häätyä" => Some(Conjugation::Taytya),
                "köraa" => Some(Conjugation::Koraa),
                "henkää" => Some(Conjugation::Henkaa),
                "humajaa" => Some(Conjugation::Kumajaa),
                _ => {
                    eprintln!("unhandled table word '{infinitive}'");
                    None
                }
            },
            "fi-conj-irreg" => match infinitive.as_str() {
                "tarvii" => Some(Conjugation::Tarvii),
                "tarttee" => Some(Conjugation::Tarttee),
                _ => {
                    eprintln!("unhandled irregular word '{infinitive}'");
                    None
                }
            },
            _ => {
                eprintln!("unknown inflection template '{template}' for '{infinitive}'");
                None
            }
        }
    }

    pub fn kotus_type(&self) -> u32 {
        self.clone() as u32
    }
}

#[derive(Debug)]
pub struct ConjugationData {
    conjugation: Conjugation,
    verb_type: &'static str,
    gradation_style: GradationStyle,
    hint_infinitive: &'static str,
    hint_present: &'static str,
    hint_past: &'static str,
}

impl Table for ConjugationData {
    fn columns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("conjugation", "INTEGER PRIMARY KEY"),
            ("verb_type", "TEXT"),
            ("gradation_style", "TEXT"),
            ("hint_infinitive", "TEXT"),
            ("hint_present", "TEXT"),
            ("hint_past", "TEXT"),
        ]
    }
    fn row(&self) -> Vec<std::string::String> {
        vec![
            self.conjugation.kotus_type().to_string(),
            self.verb_type.to_string(),
            self.gradation_style.to_string(),
            self.hint_infinitive.to_string(),
            self.hint_present.to_string(),
            self.hint_past.to_string(),
        ]
    }
}

#[derive(Debug, Clone)]
pub enum Gradation {
    None = 0,
    KkK = 1,
    PpP = 2,
    TtT = 3,
    K0 = 4,
    PV = 5,
    TD = 6,
    NkNg = 7,
    MpMm = 8,
    LtLl = 9,
    NtNn = 10,
    RtRr = 11,
    KJ = 12,
    KV = 13,
    // Non-kotus gradations
    GgG = 14,
    BbB = 15,
    T0 = 16,
}

impl Gradation {
    pub fn from_template(template: &String) -> Option<Self> {
        match template.as_str() {
            "no gradation" => Some(Gradation::None),
            "kk-k gradation" => Some(Gradation::KkK),
            "pp-p gradation" => Some(Gradation::PpP),
            "tt-t gradation" => Some(Gradation::TtT),
            "k-∅ gradation" => Some(Gradation::K0),
            "p-v gradation" => Some(Gradation::PV),
            "t-d gradation" => Some(Gradation::TD),
            "nk-ng gradation" => Some(Gradation::NkNg),
            "mp-mm gradation" => Some(Gradation::MpMm),
            "lt-ll gradation" => Some(Gradation::LtLl),
            "nt-nn gradation" => Some(Gradation::NtNn),
            "rt-rr gradation" => Some(Gradation::RtRr),
            "k-j gradation" => Some(Gradation::KJ),
            "k-v gradation" => Some(Gradation::KV),
            // non-kotus gradations
            "gg-g gradation" => Some(Gradation::GgG),
            "bb-b gradation" => Some(Gradation::BbB),
            "t-∅ gradation" => Some(Gradation::T0),
            // the data isn't specifying the gradation, so likely it's None
            non_gradation if !non_gradation.contains("gradation") => Some(Gradation::None),
            _ => {
                eprintln!("unknown gradation '{template}'");
                None
            }
        }
    }

    pub fn kotus_type(&self) -> u32 {
        self.clone() as u32
    }
}

#[derive(Debug)]
pub struct GradationData {
    gradation: Gradation,
    strong: &'static str,
    weak: &'static str,
}

impl Table for GradationData {
    fn columns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("gradation", "INTEGER PRIMARY KEY"),
            ("strong", "TEXT"),
            ("weak", "TEXT"),
        ]
    }
    fn row(&self) -> Vec<std::string::String> {
        vec![
            self.gradation.kotus_type().to_string(),
            self.strong.to_string(),
            self.weak.to_string(),
        ]
    }
}

#[derive(Debug)]
enum GradationStyle {
    None,
    StrongWeak,
    WeakStrong,
}

impl core::fmt::Display for GradationStyle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}",
            match *self {
                GradationStyle::None => "None",
                GradationStyle::StrongWeak => "StrongWeak",
                GradationStyle::WeakStrong => "WeakStrong",
            }
        )
    }
}
