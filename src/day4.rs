use anyhow::{bail, Context, Error, Result};
use std::{
    collections::HashSet,
    mem::{discriminant, Discriminant},
    str::FromStr,
};

pub fn part1(s: &'static str) -> usize {
    parse(s, "\n\n")
        .filter(|r| r.as_ref().unwrap().is_valid())
        .count()
}
pub fn part2(s: &'static str) -> usize {
    parse(s, "\n\n")
        .filter(|r| {
            let record = r.as_ref().unwrap();
            record.is_valid() && record.is_contents_valid()
        })
        .count()
}

#[derive(Debug, PartialEq, Eq)]
enum Length {
    Centimetre(i32),
    Inch(i32),
    Unknown(i32),
}
#[derive(Debug, PartialEq, Eq)]
enum Field {
    BirthYear(i64),
    IssueYear(i64),
    ExpirationYear(i64),
    Height(Length),
    HairColour(String),
    EyeColour(String),
    PassportId(String),
    CountryId(i64),
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
            Ok(Length::Unknown(s.parse()?))
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
        let ctx = || format!("Failed to parse {}:{}", tokens[0], tokens[1]);
        match tokens[0] {
            "byr" => Ok(Field::BirthYear(tokens[1].parse().with_context(ctx)?)),
            "iyr" => Ok(Field::IssueYear(tokens[1].parse().with_context(ctx)?)),
            "eyr" => Ok(Field::ExpirationYear(tokens[1].parse().with_context(ctx)?)),
            "hgt" => Ok(Field::Height(tokens[1].parse().with_context(ctx)?)),
            "hcl" => Ok(Field::HairColour(tokens[1].to_owned())),
            "ecl" => Ok(Field::EyeColour(tokens[1].to_owned())),
            "pid" => Ok(Field::PassportId(tokens[1].to_owned())),
            "cid" => Ok(Field::CountryId(tokens[1].parse().with_context(ctx)?)),
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
impl Field {
    fn is_valid(&self) -> bool {
        fn is_rgb(s: &String) -> bool {
            let b = s.as_bytes();
            b[0] == b'#'
                && b[1..].len() == 6
                && b[1..].iter().all(|b| (*b as char).is_ascii_hexdigit())
        }
        fn is_eyecolour(s: &String) -> bool {
            match s.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            }
        }
        fn is_passport_id(s: &String) -> bool {
            s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())
        }
        match self {
            Field::BirthYear(year) => *year >= 1920 && *year <= 2002,
            Field::IssueYear(year) => *year >= 2010 && *year <= 2020,
            Field::ExpirationYear(year) => *year >= 2020 && *year <= 2030,
            Field::Height(Length::Centimetre(cm)) => *cm >= 150 && *cm <= 193,
            Field::Height(Length::Inch(inch)) => *inch >= 59 && *inch <= 76,
            Field::Height(_) => false,
            Field::HairColour(s) => is_rgb(&s),
            Field::EyeColour(s) => is_eyecolour(&s),
            Field::PassportId(s) => is_passport_id(&s),
            Field::CountryId(_) => true,
        }
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
    fn is_contents_valid(&self) -> bool {
        self.0.iter().all(Field::is_valid)
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
    impl Record {
        fn minimal() -> Record {
            Record(vec![
                Field::BirthYear(0),
                Field::IssueYear(0),
                Field::ExpirationYear(0),
                Field::Height(Length::Centimetre(0)),
                Field::HairColour(Default::default()),
                Field::EyeColour(Default::default()),
                Field::PassportId(Default::default()),
            ])
        }
    }
    #[test]
    fn record_valid_with_only_required_fields() {
        assert!(Record::minimal().is_valid())
    }
    #[test]
    fn record_valid_with_all_fields() {
        let mut r = Record::minimal();
        r.0.push(Field::CountryId(0));
        assert!(r.is_valid())
    }
    #[test]
    fn record_invalid_with_duplicate() {
        let mut r = Record::minimal();
        r.0.push(Field::BirthYear(0));
        assert!(!r.is_valid())
    }

    #[test]
    fn validates_example_part1() {
        assert_eq!(2, part1(EXAMPLE));
    }

    #[test]
    fn invalidates_example_part2() {
        const EXAMPLE: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(
            4,
            parse(EXAMPLE, "\n\n")
                .flatten()
                .filter(|r| r.is_valid() && !r.is_contents_valid())
                .count()
        )
    }

    #[test]
    fn validates_example_part2() {
        const EXAMPLE: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(4, part2(EXAMPLE))
    }
}
