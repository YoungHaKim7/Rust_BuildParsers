use winnow::{ascii::alpha1, combinator::delimited, PResult, Parser};

fn parse_key(input: &mut &str) -> PResult<String> {
    let key = alpha1.parse_next(input)?;
    Ok(key.to_owned())
}

fn parse_val(input: &mut &str) -> PResult<String> {
    let val = delimited('"', alpha1, '"').parse_next(input)?;
    Ok(val.to_owned())
}

fn main() {
    println!("Hello, world!");
    let input = "width";
    let actual = parse_key.parse(input).unwrap();
    let expected = "width";
    dbg!(input);
    dbg!(actual);
    dbg!(expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key() {
        let input = "width";
        let actual = parse_key.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_val() {
        let input = r#"width"#;
        let actual = parse_val.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected);
    }
}
