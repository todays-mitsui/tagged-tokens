mod expression;
mod tag;
mod token;

use expression::Expr;
use tag::Tag;
use token::Token;

fn main() {
    println!("main");
}

pub fn tokens(expr: Expr) -> (Vec<Token>, Vec<Tag>) {
    _tokens(expr, Tag::empty())
}

pub fn _tokens(expr: Expr, tag: Tag) -> (Vec<Token>, Vec<Tag>) {
    let mut left_expr = expr;
    let mut right_exprs = Vec::new();

    while let Expr::App { left, right } = left_expr {
        left_expr = *left;
        right_exprs.push(right);
    }

    if let Expr::Var(name) = left_expr {
        let mut backquote_tokens: Vec<Token> = vec![Token::Backquote; right_exprs.len()];
        let mut backquote_tags: Vec<Tag> = (0..right_exprs.len())
            .map(|i| tag.push(i + 1))
            .rev()
            .collect();

        let left_token: Token = name.into();
        let left_tag = tag.push(0);

        let (mut right_tokens, mut right_tags): (Vec<Token>, Vec<Tag>) = right_exprs
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, expr)| {
                let tag = tag.push(i + 1);
                _tokens(*expr, tag)
            })
            .reduce(|(mut all_tokens, mut all_tags), (mut tokens, mut tags)| {
                all_tokens.append(&mut tokens);
                all_tags.append(&mut tags);
                (all_tokens, all_tags)
            })
            .unwrap_or((Vec::new(), Vec::new()));

        let mut tokens: Vec<Token> = Vec::new();
        tokens.append(&mut backquote_tokens);
        tokens.push(left_token);
        tokens.append(&mut right_tokens);

        let mut tags: Vec<Tag> = Vec::new();
        tags.append(&mut backquote_tags);
        tags.push(left_tag);
        tags.append(&mut right_tags);

        return (tokens, tags);
    } else {
        unreachable!("left_term should be a variable")
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use expression::app;
    use tag::get_range;

    fn setup() -> Expr {
        let expr = app(
            app(app(app("A", "B"), "C"), app(app("D", "E"), "F")),
            app("G", "H"),
        );
        println!("expr = {:#?}", expr);
        return expr;
    }

    #[test]
    fn test_tokens() {
        let expr = setup();

        let (tokens, tags) = tokens(expr);

        assert_eq!(
            tokens,
            vec![
                Token::Backquote,
                Token::Backquote,
                Token::Backquote,
                Token::Backquote,
                "A".into(),
                "B".into(),
                "C".into(),
                Token::Backquote,
                Token::Backquote,
                "D".into(),
                "E".into(),
                "F".into(),
                Token::Backquote,
                "G".into(),
                "H".into(),
            ]
        );

        assert_eq!(
            tags,
            vec![
                vec![4].into(),
                vec![3].into(),
                vec![2].into(),
                vec![1].into(),
                vec![0].into(),
                vec![1, 0].into(),
                vec![2, 0].into(),
                vec![3, 2].into(),
                vec![3, 1].into(),
                vec![3, 0].into(),
                vec![3, 1, 0].into(),
                vec![3, 2, 0].into(),
                vec![4, 1].into(),
                vec![4, 0].into(),
                vec![4, 1, 0].into(),
            ]
        );
    }

    #[test]
    fn test_get_range() {
        let expr = setup();

        let (tokens, tags) = tokens(expr);

        let range = get_range(tags.clone(), vec![2].into());
        assert_eq!(range, Some(2..7));
        assert_eq!(
            tokens[range.unwrap()],
            vec![
                Token::Backquote,
                Token::Backquote,
                "A".into(),
                "B".into(),
                "C".into()
            ]
        );

        let range = get_range(tags.clone(), vec![3, 1].into());
        assert_eq!(range, Some(8..11));
        assert_eq!(
            tokens[range.unwrap()],
            vec![Token::Backquote, "D".into(), "E".into()]
        );
    }
}
