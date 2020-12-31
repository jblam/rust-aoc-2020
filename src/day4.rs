use anyhow::{bail, Error, Result};
use std::str::FromStr;

pub fn part1(_: &str) {
    todo!()
}
pub fn part2(_: &str) {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
enum Length {
    Centimetre(i32),
    Inch(i32),
}
#[derive(Debug, PartialEq, Eq)]
enum Field {
    BirthYear(i32),
    IssueYear(i32),
    ExpirationYear(i32),
    Height(Length),
    HairColour(String),
    EyeColour(String),
    PassportId(i32),
    CountryId(i32),
}

struct Record(Vec<Field>);

impl FromStr for Length {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("cm") {
            Ok(Length::Centimetre(s[..s.len() - 2].parse()?))
        } else if s.ends_with("in") {
            Ok(Length::Inch(s[..s.len() - 2].parse()?))
        } else {
            bail!("Neither inches nor centimetres")
        }
    }
}
impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.splitn(2, ':').collect();
        if tokens.len() != 2 {
            bail!("Not colon-delimited")
        }
        match tokens[0] {
            "byr" => Ok(Field::BirthYear(tokens[1].parse()?)),
            "iyr" => Ok(Field::IssueYear(tokens[1].parse()?)),
            "eyr" => Ok(Field::ExpirationYear(tokens[1].parse()?)),
            "hgt" => Ok(Field::Height(tokens[1].parse()?)),
            "hcl" => Ok(Field::HairColour(tokens[1].to_owned())),
            "ecl" => Ok(Field::EyeColour(tokens[1].to_owned())),
            "pid" => Ok(Field::PassportId(tokens[1].parse()?)),
            "cid" => Ok(Field::CountryId(tokens[1].parse()?)),
            _ => bail!("unrecognised field {}", tokens[0]),
        }
    }
}
impl FromStr for Record {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace().map(Field::from_str).collect::<Result<Vec<_>>>().map(Record)
    }
}
fn parse(s: &'static str) -> impl Iterator<Item = Result<Record>> {
    const PATTERN: &str = "\n\n";
    s.split(PATTERN)
        .map(str::parse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_numeric_field() {
        assert_eq!(Field::BirthYear(1234), "byr:1234".parse().unwrap())
    }
    #[test]
    fn can_parse_string_field() {
        assert_eq!(Field::HairColour("Gross".to_string()), "hcl:Gross".parse().unwrap())
    }
    #[test]
    fn can_parse_length_field() {
        assert_eq!(Field::Height(Length::Centimetre(123)), "hgt:123cm".parse().unwrap())
    }
}
