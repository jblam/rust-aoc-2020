use anyhow::{bail, Error, Result};
use std::{
    collections::HashSet,
    mem::{discriminant, Discriminant},
    str::FromStr,
};

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

#[derive(Debug)]
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
        s.split_whitespace()
            .map(Field::from_str)
            .collect::<Result<Vec<_>>>()
            .map(Record)
    }
}
impl Record {
    fn is_valid(&self) -> bool {
        let all_fields = self.0.iter().map(discriminant).collect::<HashSet<_>>();
        const FIELD_KIND_COUNT: usize = 8;
        // JB 2021-01-01: morally const, but std::mem::discriminant is not const fn.
        let optional_field: Discriminant<Field> =
            discriminant(&Field::CountryId(Default::default()));
        (all_fields.len() == self.0.len()) // duplicate field kinds not allowed
            && (all_fields.len() == FIELD_KIND_COUNT // has all the fields, or
                // is missing only the optional field
                || (all_fields.len() == FIELD_KIND_COUNT - 1 && !all_fields.contains(&optional_field)))
    }
}
fn parse(s: &'static str, double_newline: &'static str) -> impl Iterator<Item = Result<Record>> {
    s.split(double_newline).map(str::parse)
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        mem::{discriminant, Discriminant},
    };

    use super::*;

    #[test]
    fn can_parse_numeric_field() {
        assert_eq!(Field::BirthYear(1234), "byr:1234".parse().unwrap())
    }
    #[test]
    fn can_parse_string_field() {
        assert_eq!(
            Field::HairColour("Gross".to_string()),
            "hcl:Gross".parse().unwrap()
        )
    }
    #[test]
    fn can_parse_length_field() {
        assert_eq!(
            Field::Height(Length::Centimetre(123)),
            "hgt:123cm".parse().unwrap()
        )
    }
    #[test]
    fn can_parse_record_line() {
        assert!(vec![Field::BirthYear(1), Field::IssueYear(1)]
            .iter()
            .eq("byr:1 iyr:1".parse::<Record>().unwrap().0.iter()))
    }
    #[test]
    fn can_parse_record_multiline() {
        assert!(vec![Field::BirthYear(1), Field::IssueYear(1)]
            .iter()
            .eq("byr:1\niyr:1".parse::<Record>().unwrap().0.iter()))
    }

    const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn can_parse_records() {
        // JB 2021-01-01: unclear why Rust is not finding any CRs in the above string.
        // I'm pretty sure there's CRs in the *source*.
        let result = parse(EXAMPLE, "\n\n").collect::<Result<Vec<_>>>();
        assert_eq!(result.unwrap().len(), 4)
    }
    #[test]
    fn can_interpret_discriminant() {
        let mut found_records: HashSet<Discriminant<Field>> = Default::default();
        let record = Record(vec![Field::BirthYear(1), Field::IssueYear(2)]);
        for f in record.0 {
            assert!(found_records.insert(discriminant(&f)))
        }
        assert!(!found_records.insert(discriminant(&Field::BirthYear(999))))
    }
    #[test]
    fn record_invalid_with_missing_fields() {
        assert!(!Record(Default::default()).is_valid())
    }
    impl Record {}
    fn make_minimal_record() -> Record {
        Record(vec![
            Field::BirthYear(0),
            Field::IssueYear(0),
            Field::ExpirationYear(0),
            Field::Height(Length::Centimetre(0)),
            Field::HairColour(Default::default()),
            Field::EyeColour(Default::default()),
            Field::PassportId(0),
        ])
    }
    #[test]
    fn record_valid_with_only_required_fields() {
        assert!(make_minimal_record().is_valid())
    }
    #[test]
    fn record_valid_with_all_fields() {
        let mut r = make_minimal_record();
        r.0.push(Field::CountryId(0));
        assert!(r.is_valid())
    }
    #[test]
    fn record_invalid_with_duplicate() {
        let mut r = make_minimal_record();
        r.0.push(Field::BirthYear(0));
        assert!(!r.is_valid())
    }
}
