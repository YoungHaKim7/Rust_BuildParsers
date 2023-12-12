use winnow::{ascii::alphanumeric1, PResult, Parser};

fn parse_key(input: &mut &str) -> PResult<String> {
    let key = alphanumeric1.parse_next(input)?;
    Ok(key.to_owned())
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "width";
        let actual = parse_key.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected);
    }
}
