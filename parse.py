from dataclasses import dataclass
from enum import auto, Enum
import json
from typing import Any, List, Dict, Optional


FI_DICTIONARY = "kaikki.org-dictionary-Finnish.json"


def main():
    words = []
    with open(FI_DICTIONARY, "r") as file:
        counter = 0
        for line in file:
            if counter > 388:
                break
            data = json.loads(line)
            words.append(data)
            counter += 1
    for word in words:
        print(string_from_word(word))


class Case(Enum):
    NOMINATIVE = auto()
    ACCUSATIVE_NOMINITIVE = auto()
    ACCUSATIVE_GENITIVE = auto()
    GENITIVE = auto()
    PARTITIVE = auto()
    INESSIVE = auto()
    ELATIVE = auto()
    ILLATIVE = auto()
    ADESSIVE = auto()
    ABLATIVE = auto()
    ALLATIVE = auto()
    ESSIVE = auto()
    TRANSLATIVE = auto()
    INSTRUCTIVE = auto()
    ABESSIVE = auto()
    COMITATIVE = auto()

    @classmethod
    def from_name(cls, name) -> Optional["Case"]:
        return cls.__members__.get(name.upper())


@dataclass(frozen=True)
class SingularPlural:
    singular: Optional[str]
    plural: Optional[str]


class TenseAspect(Enum):
    PRESENT = auto()
    IMPERFECT = auto()
    PERFECT = auto()
    PLUPERFECT = auto()

    @classmethod
    def of(cls, tag_list: List[str]) -> "TenseAspect":
        match (tag_list):
            case present if "present" in tag_list:
                return cls.PRESENT
            case imperfect if "imperfect" in tag_list:
                return cls.IMPERFECT
            case perfect if "perfect" in tag_list:
                return cls.PERFECT
            case pluperfect if "pluperfect" in tag_list:
                return cls.PLUPERFECT


class Mood(Enum):
    INDICATIVE = auto()
    CONDITIONAL = auto()
    IMPERITIVE = auto()
    POTENTIAL = auto()
    INFINITIVE = auto()

    @classmethod
    def of(cls, tag_list: List[str]) -> TenseAspect:
        match (tag_list):
            case indicative if "indicative" in tag_list:
                return cls.INDICATIVE
            case conditional if "conditional" in tag_list:
                return cls.CONDITIONAL
            case imperitive if "imperitive" in tag_list:
                return cls.IMPERITIVE
            case potential if "potential" in tag_list:
                return cls.POTENTIAL
            case infinitive if "infinitive" in tag_list:
                return cls.INFINITIVE


class Infinitive(Enum):
    FIRST = auto()
    LONG_FIRST = auto()
    SECOND = auto()
    THIRD = auto()
    FOURTH = auto()
    FIFTH = auto()


class Verb:
    mood: Mood
    tense_aspect: Optional[TenseAspect]
    infinitive: Optional[Infinitive]

    def __init__(self, form):
        pass


class NounInflection:
    cases: dict[Case, SingularPlural]
    first_person_singular_posessor: dict[Case, SingularPlural]
    second_person_singular_posessor: dict[Case, SingularPlural]
    first_person_plural_posessor: dict[Case, SingularPlural]
    second_person_plural_posessor: dict[Case, SingularPlural]
    third_person_posessor: dict[Case, SingularPlural]


class Noun:
    inflections: List[NounInflection]


@dataclass(frozen=True)
class Form:
    name: Optional[str]
    tags: List[str]
    source: Optional[str]

    def __str__(self) -> str:
        return (
            (self.name if self.name is not None else "-")
            + " ("
            + " ".join(self.tags)
            + ")"
        )


class Forms:
    forms: List[Form]

    def __init__(self, form_data: Optional[dict]) -> None:
        self.forms = []
        if form_data is None:
            return

        ignore_tags = ("table-tags", "inflection-template", "class")

        # possessive forms aren't tagged, so I have to do it myself
        # TODO: do a different state machine if the word is a verb
        declension_additional_tags = (
            (),  # non-possessive forms
            (),  # transition to possessive forms
            ("first-person", "singular", "possessor"),  # possessive forms
            ("second-person", "singular", "possessor"),
            ("first-person", "plural", "possessor"),
            ("second-person", "plural", "possessor"),
            ("third-person", "possessor"),
        )
        form_state = -1  # there's a one-time transition at the beginning

        for form in form_data:
            tags = form.get("tags")
            if tags is not None and "inflection-template" in tags:
                # it needs to loop through the states since some words
                # like "menu" have multiple sets of declensions
                form_state = (form_state + 1) % len(declension_additional_tags)
            if form_state >= 0 and (
                tags is None
                or not any(ignore_tag in tags for ignore_tag in ignore_tags)
            ):
                for additional_tag in declension_additional_tags[form_state]:
                    if additional_tag not in tags:
                        tags.append(additional_tag)
                form_name = form.get("form")
                if form_name == "-":
                    form_name = None
                form_data = Form(
                    form_name, form.get("tags", []), form.get("source")
                )
                self.forms.append(form_data)

    def __str__(self) -> str:
        if len(self.forms) == 0:
            return ""
        return "\n".join(str(form) for form in self.forms) + "\n"


@dataclass(frozen=True)
class Example:
    text: str
    english: Optional[str]

    def __str__(self) -> str:
        return self.text + (
            " (" + self.english + ")" if self.english is not None else ""
        )


class Sense:
    glosses: Optional[List[str]]
    examples: Optional[List[Example]]

    def __init__(
        self, glosses: Optional[List[str]], examples: Optional[List[Example]]
    ) -> None:
        self.glosses = glosses
        try:
            self.examples = (
                [
                    Example(example["text"], example.get("english"))
                    for example in examples
                ]
                if examples is not None
                else None
            )
        except:
            print(examples)
            raise

    def __str__(self) -> str:
        gloss_text = (
            "\n".join(str(gloss) for gloss in self.glosses)
            if self.glosses is not None and len(self.glosses) > 0
            else ""
        )
        example_text = (
            "\n".join(str(example) for example in self.examples)
            if self.examples is not None and len(self.examples) > 0
            else ""
        )
        return (
            gloss_text
            + ("\n" if len(gloss_text) > 0 and len(example_text) > 0 else "")
            + example_text
        )


class Senses:
    senses: List[Sense]

    def __init__(self, sense_data: Optional[dict]) -> None:
        self.senses = []
        if sense_data is None:
            return
        for sense in sense_data:
            self.senses.append(
                Sense(sense.get("glosses"), sense.get("examples"))
            )

    def __str__(self) -> str:
        if len(self.senses) == 0:
            return ""
        return "\n".join(str(sense) for sense in self.senses) + "\n"


def string_from_word(word: dict):
    name = word.get("word")
    try:
        part_of_speach = word.get("pos")
        word_forms = Forms(word.get("forms"))
        senses = Senses(word.get("senses"))

        etymology_text = word.get("etymology_text", "")

        expansions = []
        if word.get("etymology_templates") is not None:
            for etymology_template in word["etymology_templates"]:
                expansion = etymology_template.get("expansion")
                if expansion is not None:
                    expansions.append(expansion)
        expansion_text = (
            expansions[0] if len(expansions) == 1 else ", ".join(expansions)
        ) + "\n"
    except:
        print(name)
        raise

    return (
        f"{part_of_speach} {name}\n"
        + str(senses)
        + etymology_text
        + "\n"
        + str(word_forms)
    )


## I want:
## - the word
## - the meaning
## - the etymology
## - the inflections


if __name__ == "__main__":
    main()
