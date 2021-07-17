use inline_python::{python, Context};

pub fn split_sentences_with_python(language: &str, text: &str) -> Vec<String> {
    match language {
        "en" => split_sentences_with_python_en(text),
        _ => {
            panic!("{} is not supported for Python tokenizer, please implement it or remove the tokenizer rule", language);
        },
    }
}

// Note that this is for reference only, for now English uses the default rust-punkt
// tokenizer. This can be copy/pasted to implement new Python based tokenizers.
// If you want to test the English implementation, add `tokenizer = "python"` to the
// English rules file. See the README for more information on the Python tokenizer
// implementation.
pub fn split_sentences_with_python_en(text: &str) -> Vec<String> {
    let ctx = Context::new();

    ctx.run(python! {
        import nltk

        try:
            nltk.data.find("tokenizers/punkt")
        except LookupError:
            nltk.download("punkt")

        split_sentences = nltk.sent_tokenize('text)
    });
    
    ctx.get("split_sentences")
}
