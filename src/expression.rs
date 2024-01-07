#[derive(Debug, PartialEq)]
pub enum Expr {
    Var(String),
    App { left: Box<Expr>, right: Box<Expr> },
}

impl From<&str> for Expr {
    fn from(name: &str) -> Self {
        Expr::Var(name.to_string())
    }
}

// ========================================================================== //

pub fn var(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn app<L: Into<Expr>, R: Into<Expr>>(left: L, right: R) -> Expr {
    Expr::App {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var() {
        assert_eq!(var("X"), Expr::Var("X".to_string()));
    }

    #[test]
    fn test_app() {
        assert_eq!(
            app("X", "Y"),
            Expr::App {
                left: Box::new(Expr::Var("X".to_string())),
                right: Box::new(Expr::Var("Y".to_string())),
            }
        );

        let x = var("X");
        let y = var("Y");
        assert_eq!(
            app(x, y),
            Expr::App {
                left: Box::new(var("X")),
                right: Box::new(var("Y"))
            }
        );

        assert_eq!(
            app(app("X", "Y"), "Z"),
            Expr::App {
                left: Box::new(Expr::App {
                    left: Box::new(Expr::Var("X".to_string())),
                    right: Box::new(Expr::Var("Y".to_string())),
                }),
                right: Box::new(Expr::Var("Z".to_string())),
            }
        );
    }
}
