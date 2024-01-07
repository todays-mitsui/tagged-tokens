mod expression;
mod tag;
mod token;

use expression::app;
use tag::get_range;
use token::tokenize;

fn main() {
    let expr = app(
        app(app(app("A", "B"), "C"), app(app("D", "E"), "F")),
        app("G", "H"),
    );
    println!("expr: {:#?}", expr);

    let (tokens, tags) = tokenize(expr);
    println!("tokens: {:?}", tokens);
    println!("tags: {:?}", tags);

    let range = get_range(tags, vec![3, 1].into()).unwrap();
    println!("range: {:?}", range);
    println!("tokens[range]: {:?}", &tokens[range])
}
