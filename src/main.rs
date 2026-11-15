mod lexer;
mod parser;
mod tokens;

fn main() {
    let lex_test: String =
        "?x [num] = 36;\n21+9/5;?add(x[num], y[num]) [num] { return x + y; }".into();
    let lexed = lexer::lex_code(lex_test);
    println!("{:#?}", lexed);
}
