use inline_python::{python, Context};

pub fn split_sentences_with_python(language: &str, text: &str) -> Vec<String> {
    match language {
        "en" => split_sentences_with_python_en(text),
        "de" => split_sentences_with_python_de(text),
        "pl" => split_sentences_with_python_pl(text),
        _ => {
            panic!("{} is not supported for Python segmenter, please implement it or remove the segmenter rule", language);
        },
    }
}

// Note that this is for reference only, for now English uses the default rust-punkt
// segmenter. This can be copy/pasted to implement new Python based segmenters.
// If you want to test the English implementation, add `segmenter = "python"` to the
// English rules file. See the README for more information on the Python segmenter
// implementation.
pub fn split_sentences_with_python_en(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        import nltk

        try:
            nltk.data.load("tokenizers/punkt")
        except LookupError:
            nltk.download("punkt")

        split_sentences = nltk.sent_tokenize('text)
    });

    ctx.get("split_sentences")
}

pub fn split_sentences_with_python_de(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        import nltk

        try:
            nltk.data.load("tokenizers/punkt/german.pickle")
        except LookupError:
            nltk.download("punkt")

        split_sentences = nltk.sent_tokenize('text)
    });

    ctx.get("split_sentences")
}

pub fn split_sentences_with_python_pl(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        """
        Core of the script taken from https://gist.github.com/ksopyla/f05fe2f48bbc9de895368b8a7863b5c3
        All credits goes to its respective owner.
        """
        import nltk

        try:
            nltk.data.find("tokenizers/punkt")
        except LookupError:
            nltk.download("punkt")

        extra_abbreviations = [
            "ps", "inc", "corp", "ltd", "Co", "pkt", "Dz.Ap", "Jr", "jr", "sp.k", "sp", "poj", "pseud",
            "krypt", "ws", "itd", "np", "sanskryt", "nr", "gł", "Takht", "tzw", "tzn", "t.zw", "ewan", "tyt", "fig",
            "oryg", "t.j", "vs", "l.mn", "l.poj", "ul", "al", "Al", "el", "tel", "bud", "pok", "wł",
            "wew",  # wewnętrzny
            "sam",  # samochód
            "sa",  # spółka sa.
            "wit",  # witaminy
            "mat",  # materiały
            "kat",  # kategorii
            "wg",  # według
            "btw",  #
            "itp",  #
            "wz",  # w związku
            "gosp",  #
            "dział",  #
            "hurt",  #
            "mech",  #
            "wyj",  # wyj
            "pt",  # pod tytułem
            "zew",  # zewnętrzny
            # "Sp",
        ]

        position_abbrev = [
            "Ks", "Abp", "abp", "bp", "dr", "kard", "mgr", "prof", "zwycz", "hab", "arch",
            "arch.kraj", "B.Sc", "Ph.D", "lek", "med", "n.med", "bł", "św", "hr", "dziek",
        ]

        roman_abbrev = (
            []
        )

        quantity_abbrev = [
            "mln", "tys", "obr./min", "km/godz", "godz", "egz", "ha", "j.m", "cal", "obj",
            "alk", "wag", "op", "wk", "mm",
            "MB",  # mega bajty
            "Mb",  # mega bity
            "jedn",  # jednostkowe
            "obr",  # obroty
            "szt",  # sztuk
        ]

        actions_abbrev = [
            "tłum", "tlum", "zob", "wym", "w/wym", "pot", "ww", "ogł", "wyd", "min",
            "m.i", "m.in", "in", "im", "muz", "tj", "dot", "wsp", "właść", "właśc", "przedr",
            "czyt", "proj", "dosł", "hist", "daw", "zwł", "zaw", "późn", "spr", "jw",
            "odp",  # odpowiedź
            "symb",  # symbol
            "klaw",  # klawiaturowe
        ]

        place_abbrev = [
            "śl",
            "płd",
            "geogr",
            "zs",
            "pom",  # pomorskie
            "kuj-pom",  # kujawsko pomorskie
        ]

        lang_abbrev = [
            "jęz", "fr", "franc", "ukr", "ang", "gr", "hebr", "czes", "pol", "niem",
            "arab", "egip", "hiszp", "jap", "chin", "kor", "tyb", "wiet", "sum", "chor",
            "słow", "węg", "ros", "boś", "szw",
        ]

        administration = [
            "dz.urz",  # dziennik urzędowy
            "póź.zm",
            "rej",  # rejestr, rejestracyjny dowód
            "sygn",  # sygnatura
            "Dz.U",  # dziennik ustaw
            "woj",  # województow
            "ozn",  #
            "ust",  # ustawa
            "ref",  # ref
            "dz",
            "akt",  # akta
        ]

        time = [
            "tyg",  # tygodniu
        ]

        military_abbrev = [
            "kpt", "kpr", "obs", "pil", "mjr", "płk", "dypl", "pp", "gw", "dyw",
            "ppłk", "mar", "marsz", "rez", "ppor", "DPanc", "BPanc", "DKaw", "p.uł",
            "sierż", "post", "asp", "podinsp", "nadkom"
            "bryg",  # brygady
            "szt",  # sztabowy
            "kom",  # komendant, tel. komórka
        ]

        extra_abbreviations = (
            extra_abbreviations
            + position_abbrev
            + quantity_abbrev
            + place_abbrev
            + actions_abbrev
            + lang_abbrev
            + administration
            + time
            + military_abbrev
        )

        sentence_tokenizer = nltk.data.load("tokenizers/punkt/polish.pickle")
        sentence_tokenizer._params.abbrev_types.update(extra_abbreviations)

        split_sentences = sentence_tokenizer.tokenize('text)
    });

    ctx.get("split_sentences")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_segmenter_en() {
        let language = "en";
        let text = "I am a sentence. Me too!";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }

    #[test]
    fn test_segmenter_de() {
        let language = "de";
        let text = "I am a sentence. Me too!";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }

    #[test]
    fn test_segmenter_pl() {
        let language = "pl";
        let text = "Jestem zdaniem. Ja również!";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_segmenter_invalid_language() {
        let language = "INVALID_LANGUAGE";
        let text = "I am a sentence. Me too!";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }
}
