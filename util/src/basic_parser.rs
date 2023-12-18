use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char, digit1, space1};
use nom::combinator::{map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::IResult;

pub fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((i, number))
}
pub fn isize_list(s: &str) -> IResult<&str, Vec<isize>> {
    separated_list1(space1, parse_isize)(s)
}

pub fn parse_usize(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(s)
}

pub fn usize_list(s: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_usize)(s)
}

pub fn title(s: &str) -> IResult<&str, &str> {
    terminated(take_while(|c| c != ':'), preceded(char(':'), space1))(s)
}

pub fn from_hex(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_parser() {
        assert_eq!(parse_usize("145").unwrap().1, 145);
    }
    #[test]
    fn usize_list_parser() {
        assert_eq!(usize_list("123 442 335").unwrap().1, vec![123, 442, 335]);
    }
    #[test]
    fn isize_parser() {
        assert_eq!(parse_isize("-145").unwrap().1, -145);
    }
    #[test]
    fn isize_list_parser() {
        assert_eq!(isize_list("123 -442 335").unwrap().1, vec![123, -442, 335]);
    }

    #[test]
    fn title_parser() {
        assert_eq!(
            title("Level 3: \t a rabbit").unwrap(),
            ("a rabbit", "Level 3")
        );
    }

    #[test]
    fn hex_parser() {
        assert_eq!(from_hex("70c71"), Ok(461937))
    }
}
