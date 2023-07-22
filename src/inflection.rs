pub fn create_table(table_name: &'static str) -> String {
    let inflections: Vec<Inflection> = vec![
        Inflection {
            verb_type: "1",
            kotus_type: "52",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-[ouöy]a",
            hint_present: "-[ouöy]-",
            hint_past: "-[ouöy]i-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "53",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-aa",
            hint_present: "-a-",
            hint_past: "-i-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "54",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-taa",
            hint_present: "-ta- (KPT)",
            hint_past: "-si-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "55",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-taa",
            hint_present: "-ta- (KPT)",
            hint_past: "-ti- (KPT)",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "56",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-aa",
            hint_present: "-a-",
            hint_past: "-oi-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "57",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-taa",
            hint_present: "-a-",
            hint_past: "-si-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "58",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-ea",
            hint_present: "-e-",
            hint_past: "-i-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "59",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-ntea",
            hint_present: "-nte- (KPT)",
            hint_past: "-nsi-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "60",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-htea",
            hint_present: "-hde-",
            hint_past: "-hdi-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "61",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "-ia",
            hint_present: "-i-",
            hint_past: "-i-",
        },
        Inflection {
            verb_type: "2",
            kotus_type: "62",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-{vowel}ida",
            hint_present: "-{vowel}i-",
            hint_past: "-{vowel}i-",
        },
        Inflection {
            verb_type: "2",
            kotus_type: "63",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-{long vowel}da",
            hint_present: "-{long vowel}-",
            hint_past: "-{short vowel}i-",
        },
        Inflection {
            verb_type: "2",
            kotus_type: "64",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-ieda, -uoda, -yödä",
            hint_present: "-ie-, -uo-, -yö-",
            hint_past: "-ei-, -oi-, -öi-",
        },
        Inflection {
            verb_type: "2",
            kotus_type: "65",
            gradation_style: GradationStyle::None,
            hint_infinitive: "käydä",
            hint_present: "käy-",
            hint_past: "kävi-",
        },
        Inflection {
            verb_type: "3",
            kotus_type: "66",
            gradation_style: GradationStyle::WeakStrong, // but most verbs don't gradate
            hint_infinitive: "-sta",
            hint_present: "-se-",
            hint_past: "-si-",
        },
        Inflection {
            verb_type: "3",
            kotus_type: "67",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-lla, -nna, -rra",
            hint_present: "-le-, -ne-, -re-",
            hint_past: "-li-, -ni-, -ri-",
        },
        Inflection {
            verb_type: "2",
            kotus_type: "68",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-oida",
            hint_present: "-oi-",
            hint_past: "-oi-",
        },
        Inflection {
            verb_type: "5",
            kotus_type: "69",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-ita",
            hint_present: "-itse-",
            hint_past: "-itsi-",
        },
        Inflection {
            verb_type: "3",
            kotus_type: "70",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-sta",
            hint_present: "-kse-",
            hint_past: "-ksi-",
        },
        Inflection {
            verb_type: "2",
            kotus_type: "71",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "nähdä, tehdä",
            hint_present: "näe-, tee- (KPT)",
            hint_past: "näi-, tei- (KPT)",
        },
        Inflection {
            verb_type: "6",
            kotus_type: "72",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-eta",
            hint_present: "-ene-",
            hint_past: "-eni-",
        },
        Inflection {
            verb_type: "4",
            kotus_type: "73",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-[aouäöy]ta",
            hint_present: "-[aouäöy]a-",
            hint_past: "-[aouäöy]si-",
        },
        Inflection {
            verb_type: "4",
            kotus_type: "74",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-[eouöy]ta",
            hint_present: "-[eouöy]a-",
            hint_past: "-[eouöy]si-",
        },
        Inflection {
            verb_type: "4",
            kotus_type: "75",
            gradation_style: GradationStyle::WeakStrong,
            hint_infinitive: "-[eiouöy]ta",
            hint_present: "-[eiouöy]a-",
            hint_past: "-[eiouöy]si-",
        },
        Inflection {
            verb_type: "1",
            kotus_type: "76",
            gradation_style: GradationStyle::StrongWeak,
            hint_infinitive: "taitaa, teitää",
            hint_present: "-ta- (KPT)",
            hint_past: "-si-",
        },
        Inflection {
            verb_type: "uncommon",
            kotus_type: "77",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-ajaa",
            hint_present: "-aja-",
            hint_past: "-asi-, -aji-",
        },
        Inflection {
            verb_type: "uncommon",
            kotus_type: "78",
            gradation_style: GradationStyle::None,
            hint_infinitive: "-kaa, -saa",
            hint_present: "-kaa-, -saa-",
            hint_past: "past tense is not used",
        },
    ];

    let columns: Vec<&'static str> = vec![
        "verb_type",
        "kotus_type",
        "gradation_style",
        "hint_infinitive",
        "hint_present",
        "hint_past",
    ];
    vec![
        format!("CREATE TABLE {table_name} ("),
        columns.join(", "),
        ");\n".to_string(),
        format!("INSERT INTO {table_name} ("),
        columns.join(", "),
        ") VALUES".to_string(),
        inflections
            .iter()
            .map(|inflection| inflection.to_string())
            .collect::<Vec<String>>()
            .join(",\n"),
    ]
    .join(" ")
}

pub fn gradation_table(table_name: &'static str) -> String {
    let gradations: Vec<Gradation> = vec![
        Gradation {
            kotus_type: "A",
            strong: "kk",
            weak: "k",
        },
        Gradation {
            kotus_type: "B",
            strong: "pp",
            weak: "p",
        },
        Gradation {
            kotus_type: "C",
            strong: "tt",
            weak: "t",
        },
        Gradation {
            kotus_type: "D",
            strong: "k",
            weak: "",
        },
        Gradation {
            kotus_type: "E",
            strong: "p",
            weak: "v",
        },
        Gradation {
            kotus_type: "F",
            strong: "t",
            weak: "d",
        },
        Gradation {
            kotus_type: "G",
            strong: "nk",
            weak: "ng",
        },
        Gradation {
            kotus_type: "H",
            strong: "mp",
            weak: "mm",
        },
        Gradation {
            kotus_type: "I",
            strong: "lt",
            weak: "ll",
        },
        Gradation {
            kotus_type: "J",
            strong: "nt",
            weak: "nn",
        },
        Gradation {
            kotus_type: "K",
            strong: "rt",
            weak: "rr",
        },
        Gradation {
            kotus_type: "L",
            strong: "k",
            weak: "j",
        },
        Gradation {
            kotus_type: "M",
            strong: "k",
            weak: "v",
        },
    ];

    let columns: Vec<&'static str> = vec!["kotus_type", "strong", "weak"];
    vec![
        format!("CREATE TABLE {table_name} ("),
        columns.join(", "),
        ");\n".to_string(),
        format!("INSERT INTO {table_name} ("),
        columns.join(", "),
        ") VALUES".to_string(),
        gradations
            .iter()
            .map(|gradation| gradation.to_string())
            .collect::<Vec<String>>()
            .join(",\n"),
    ]
    .join(" ")
}

#[derive(Debug)]
pub struct Inflection {
    verb_type: &'static str,
    kotus_type: &'static str,
    gradation_style: GradationStyle,
    hint_infinitive: &'static str,
    hint_present: &'static str,
    hint_past: &'static str,
}

impl Inflection {
    pub fn from_template(template: String) -> Self {
        todo!()
    }
}

impl core::fmt::Display for Inflection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "('{}')",
            vec![
                self.verb_type,
                self.kotus_type,
                &self.gradation_style.to_string(),
                self.hint_infinitive,
                self.hint_present,
                self.hint_past,
            ]
            .join("', '")
        )
    }
}

#[derive(Debug)]
pub struct Gradation {
    kotus_type: &'static str,
    strong: &'static str,
    weak: &'static str,
}

impl Gradation {
    pub fn from_template(template: String) -> Self {
        todo!()
    }
}

impl core::fmt::Display for Gradation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "('{}')",
            vec![self.kotus_type, self.strong, self.weak,].join("', '")
        )
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
