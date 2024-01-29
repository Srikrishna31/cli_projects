use crate::PositionList;
use command_utils::MyResult;
use std::ops::Range;

enum ParsePos {
    Range(Range<usize>),
    Digit(usize),
}

fn err_string(n: &str) -> String {
    format!("illegal list value: \"{n}\"")
}
fn parse_range(range: &str) -> MyResult<ParsePos> {
    let parts: Vec<&str> = range.split('-').collect();
    let parse_number = |n: &str| {
        if n.is_empty() || n.as_bytes()[0] as char == '+' {
            return Err(From::from(err_string(n)));
        }
        let res = n.parse::<usize>().map_err(|_| err_string(n))?;
        if res == 0 {
            return Err(From::from(err_string(n)));
        }
        Ok(ParsePos::Digit(res))
    };

    match parts.len() {
        0 => Ok(parse_number(range)?),
        1 => parse_number(parts[0]),
        2 => {
            let ParsePos::Digit(num1) = parse_number(parts[0])? else {
                unreachable!("Unreachable path")
            };
            let ParsePos::Digit(num2) = parse_number(parts[1])? else {
                unreachable!("Unreachable path")
            };
            if num1 >= num2 {
                return Err(From::from(format!(
                    "First number in range ({num1}) must be lower than second number ({num2})"
                )));
            }
            Ok(ParsePos::Range(num1 - 1..num2))
        }
        _ => Err(From::from(
            "A range should be specified using only one '-'.",
        )),
    }
}
pub(super) fn parse_pos(range: &str) -> MyResult<PositionList> {
    if range.is_empty() {
        return Err(From::from("Range cannot be empty"));
    }
    let parts: Vec<_> = range.split(',').collect();

    parts
        .iter()
        .map(|p| -> MyResult<Range<usize>> {
            Ok(match parse_range(p)? {
                ParsePos::Digit(num) => num - 1..num,
                ParsePos::Range(r) => r,
            })
        })
        .collect()
}

#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        // A leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"",);

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"");

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+2\"");

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001-0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
