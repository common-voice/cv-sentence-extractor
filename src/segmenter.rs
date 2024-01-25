use inline_python::{python, Context};

pub fn split_sentences_with_python(language: &str, text: &str) -> Vec<String> {
    match language {
        "en" => split_sentences_with_python_en(text),
        "de" => split_sentences_with_python_de(text),
        "bn" => split_sentences_with_python_bn(text),
        "tr" => split_sentences_with_python_tr(text),
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

pub fn split_sentences_with_python_bn(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        from nltk.tokenize.punkt import PunktSentenceTokenizer, PunktLanguageVars
        try:
            class BengaliLangVars(PunktLanguageVars):
                sent_end_chars = ('?', '!', '।')
        
        except LookupError:
            nltk.download("punkt")
            class BengaliLangVars(PunktLanguageVars):
                sent_end_chars = ('?', '!', '।')

        tokenizer = PunktSentenceTokenizer(lang_vars = BengaliLangVars())

        split_sentences = tokenizer.tokenize('text)
    });

    ctx.get("split_sentences")
}

pub fn split_sentences_with_python_tr(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        from zemberek import TurkishSentenceExtractor

        extractor = TurkishSentenceExtractor()
        split_sentences = extractor.from_paragraph('text)
    });

    ctx.get("split_sentences")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_segmenter_de() {
        let language = "de";
        let text = "I am a sentence. Me too!";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }

    #[test]
    fn test_segmenter_bn() {
        let language = "bn";
        let text = "আমি প্রথম বাক্য। আমি আর একটি বাক্য।";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }

    #[test]
    fn test_segmenter_tr() {
        let language = "tr";
        let text = "Ben bir cümleyim. Ben de!";

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
