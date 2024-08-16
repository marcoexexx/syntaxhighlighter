use highlight::{theme::ApplyTheme, themes::gruvbox::gruvbox_theme, Highlighter};
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
    let theme = gruvbox_theme();
    let js_highlighter = Highlighter::new(JS_CODE_EXAMPLE, Box::new(JavaScriptTokenizer));

    let js_tokens = js_highlighter.highlight();

    let mut highlighted = String::from(JS_CODE_EXAMPLE);

    for token in js_tokens {
        token.apply_theme_and_render(&theme, &mut highlighted);
    }

    println!("{}", highlighted)
}
