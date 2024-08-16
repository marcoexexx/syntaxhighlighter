use highlight::Highlighter;
use languages::js::JavaScriptTokenizer;

const JS_CODE_EXAMPLE: &str = r#"
class Cat {
  isTeen() {
    if (this.age < 10) return "teen"
  }
}

// this is single comment.

/** 
 * this is multi comment
 * next line multi comment
 */

function main() {
    let x = 42;
    let DATA = "hello";
    if (x > 10) {
        console.log('Hello, world!');
    }
}
"#;

fn main() {
    let js_highlighter = Highlighter::new(Box::new(JavaScriptTokenizer));

    let js_tokens = js_highlighter.highlight(JS_CODE_EXAMPLE);

    println!("{:#?}", js_tokens);
}
