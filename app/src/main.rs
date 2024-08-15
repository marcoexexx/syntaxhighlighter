use languages::js::JavaScriptTokenizer;
use syntaxhighlighter::highlighter::Highlighter;

fn main() {
    let js_code = r#"
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

    let js_highlighter = Highlighter::new(Box::new(JavaScriptTokenizer));

    let js_tokens = js_highlighter.highlight(js_code);

    println!("{:#?}", js_tokens);
}
