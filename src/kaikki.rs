use nanoserde::{DeJson, DeJsonState, SerJson};

#[derive(Debug, SerJson, DeJson)]
pub struct Sana {
    pos: String,
    head_templates: Option<Vec<HeadTemplate>>,
    inflection_templates: Option<Vec<InflectionTemplate>>,
    forms: Option<Vec<Form>>,
    etymology_text: Option<String>,
    etymology_templates: Option<Vec<EtymologyTemplate>>,
    sounds: Option<Vec<std::collections::HashMap<String, String>>>,
    word: String,
    lang: String,
    lang_code: Option<String>,
    derived: Option<Vec<Derive>>,
    senses: Vec<Sense>,
    synonyms: Option<Vec<Synonym>>,
}

impl Sana {
    pub fn from_json(json: String) -> Option<Self> {
        let mut json_state = DeJsonState::default();
        let mut chars = json.chars();
        json_state.next(&mut chars);
        json_state.next_tok(&mut chars).unwrap();
        Self::de_json(&mut json_state, &mut chars).ok()
    }

    pub fn is_a_verb(&self) -> bool {
        if let Some(forms) = &self.forms {
            if !forms.is_empty() {
                if let Some(secondary_pos) = &self.head_templates {
                    if let Some(secondary_pos) = secondary_pos.get(0) {
                        if let Some(secondary_pos) = secondary_pos.args.get("2") {
                            return self.pos == "verb" && secondary_pos == "verb";
                        }
                    }
                }
            }
        }
        false
    }

    pub fn expansion(&self) -> Option<&String> {
        if let Some(head_template) = &self.head_templates {
            if let Some(head_template) = head_template.get(0) {
                return Some(&head_template.expansion);
            }
        }
        None
    }

    pub fn forms(&self) -> Option<&Vec<Form>> {
        self.forms.as_ref()
    }

    pub fn inflection_templates(&self) -> Option<&Vec<InflectionTemplate>> {
        self.inflection_templates.as_ref()
    }
}

#[derive(Debug, SerJson, DeJson)]
struct HeadTemplate {
    name: String,
    args: std::collections::HashMap<String, String>,
    expansion: String,
}

#[derive(Debug, SerJson, DeJson)]
pub struct InflectionTemplate {
    name: String,
    args: std::collections::HashMap<String, String>,
}

impl InflectionTemplate {
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, SerJson, DeJson)]
pub struct Form {
    form: String,
    source: String,
    tags: Vec<String>,
}

impl Form {
    pub fn name(&self) -> &String {
        &self.form
    }

    pub fn source(&self) -> &String {
        &self.source
    }

    pub fn tags(&self) -> std::collections::HashSet<&str> {
        std::collections::HashSet::from_iter(self.tags.iter().map(|string| string.as_str()))
    }
}

#[derive(Debug, SerJson, DeJson)]
struct EtymologyTemplate {
    name: String,
    args: std::collections::HashMap<String, String>,
    expansion: String,
}

#[derive(Debug, SerJson, DeJson)]
struct Derive {
    word: String,
    _dis1: String,
}

#[derive(Debug, SerJson, DeJson)]
struct Sense {
    links: Vec<Vec<String>>,
    topics: Option<Vec<String>>,
    raw_glosses: Option<Vec<String>>,
    glosses: Option<Vec<String>>,
    form_of: Option<Vec<FormOf>>,
    alt_of: Option<Vec<AltOf>>,
    tags: Option<Vec<String>>,
    to: Option<String>,
    categories: Vec<Category>,
}

#[derive(Debug, SerJson, DeJson)]
struct FormOf {
    word: String,
}

#[derive(Debug, SerJson, DeJson)]
struct AltOf {
    word: String,
    extra: String,
}

#[derive(Debug, SerJson, DeJson)]
struct Category {
    name: String,
    kind: String,
    parents: Vec<String>,
    source: String,
    _dis: Option<String>,
}

#[derive(Debug, SerJson, DeJson)]
struct Synonym {
    word: String,
    sense: Option<String>,
}
