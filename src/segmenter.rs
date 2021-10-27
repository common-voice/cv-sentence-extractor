use inline_python::{python, Context};

pub fn split_sentences_with_python(language: &str, text: &str) -> Vec<String> {
    match language {
        "en" => split_sentences_with_python_en(text),
        "de" => split_sentences_with_python_de(text),
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

        split_sentences = nltk.sent_tokenize('text, language="german")
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
    #[should_panic]
    fn test_segmenter_invalid_language() {
        let language = "INVALID_LANGUAGE";
        let text = "I am a sentence. Me too!";

        assert_eq!(split_sentences_with_python(language, text).len(), 2);
    }
}
