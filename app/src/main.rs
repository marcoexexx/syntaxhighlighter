use languages::js::JavaScriptTokenizer;
use syntaxhighlighter::highlighter::Highlighter;

fn main() {
    let js_code = "function main() { let x = 42; };";

    let js_highlighter = Highlighter::new(Box::new(JavaScriptTokenizer));

    let js_tokens = js_highlighter.highlight(js_code);

    println!("{:?}", js_tokens);
}
