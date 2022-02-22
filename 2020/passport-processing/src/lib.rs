#![feature(test)]
#![feature(str_split_once)]

use std::fs;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^([0-9]+)([cimn]{2})$").unwrap();
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PASSPORT_ID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}
lazy_static! {
    static ref EYE_COLORS: Vec<String> = vec![
        "amb".to_owned(),
        "blu".to_owned(),
        "brn".to_owned(),
        "gry".to_owned(),
        "grn".to_owned(),
        "hzl".to_owned(),
        "oth".to_owned()
    ];
}

#[derive(Default, Debug)]
struct Document {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Document {
    fn passport_fields_present(self: &Self) -> bool {
        self.npc_fields_present() && self.cid.is_some()
    }

    fn npc_fields_present(self: &Self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn validate_passport(self: &Self) -> bool {
        if let None = &self.cid {
            return false;
        }

        self.validate_npc()
    }

    fn validate_npc(self: &Self) -> bool {
        // Birth year
        if let Some(byr) = &self.byr {
            if byr.len() != 4 {
                return false;
            } else {
                if let Ok(val) = byr.parse::<u16>() {
                    if val < 1920 || val > 2002 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }

        // Issue year
        if let Some(iyr) = &self.iyr {
            if iyr.len() != 4 {
                return false;
            } else {
                if let Ok(val) = iyr.parse::<u16>() {
                    if val < 2010 || val > 2020 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }

        // Expiration year
        if let Some(eyr) = &self.eyr {
            if eyr.len() != 4 {
                return false;
            } else {
                if let Ok(val) = eyr.parse::<u16>() {
                    if val < 2020 || val > 2030 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }

        // Height
        if let Some(hgt) = &self.hgt {
            if let Some(cap) = HEIGHT_REGEX.captures(hgt) {
                match cap[2].as_ref() {
                    "cm" => {
                        if let Ok(val) = cap[1].parse::<u8>() {
                            if val < 150 || val > 193 {
                                println!("Captures: {:#?}", cap);
                                return false;
                            }
                        } else {
                            println!("Captures: {:#?}", cap);
                            return false;
                        }
                    }
                    "in" => {
                        if let Ok(val) = cap[1].parse::<u8>() {
                            if val < 59 || val > 76 {
                                println!("Captures: {:#?}", cap);
                                return false;
                            }
                        } else {
                            println!("Captures: {:#?}", cap);
                            return false;
                        }
                    }
                    _ => {
                        println!("Captures: {:#?}", cap);
                        return false;
                    }
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        // Hair Color
        if let Some(hcl) = &self.hcl {
            if !COLOR_REGEX.is_match(hcl) {
                return false;
            }
        } else {
            return false;
        }

        // Eye Color
        if let Some(ecl) = &self.ecl {
            if !EYE_COLORS.contains(ecl) {
                return false;
            }
        } else {
            return false;
        }

        // Passport ID
        if let Some(pid) = &self.pid {
            if !PASSPORT_ID.is_match(pid) {
                return false;
            }
        } else {
            return false;
        }

        return true;
    }
}

#[allow(dead_code)]
fn read_documents(path: &str) -> Vec<Document> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|mld| mld.replace("\n", " "))
        .map(|rd| {
            let mut document = Document::default();

            rd.split(" ").into_iter().for_each(|pair| {
                if let Some((field, value)) = pair.split_once(":") {
                    match field {
                        "byr" => document.byr = Some(value.to_owned()),
                        "iyr" => document.iyr = Some(value.to_owned()),
                        "eyr" => document.eyr = Some(value.to_owned()),
                        "hgt" => document.hgt = Some(value.to_owned()),
                        "hcl" => document.hcl = Some(value.to_owned()),
                        "ecl" => document.ecl = Some(value.to_owned()),
                        "pid" => document.pid = Some(value.to_owned()),
                        "cid" => document.cid = Some(value.to_owned()),
                        other => println!("Found field: {}", other),
                    }
                }
            });

            document
        })
        .collect()
}

#[allow(dead_code)]
fn check_presence(documents: Vec<Document>) -> Vec<Document> {
    documents
        .into_iter()
        .filter(|d| d.passport_fields_present() || d.npc_fields_present())
        .collect()
}

#[allow(dead_code)]
fn validate(documents: Vec<Document>) -> Vec<Document> {
    check_presence(documents)
        .into_iter()
        .filter(|d| d.validate_passport() || d.validate_npc())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_read_documents() {
        let documents = read_documents("test/sample");
        assert_eq!(documents.len(), 4);
    }

    #[test]
    fn test_sample_document_fields() {
        let documents = check_presence(read_documents("test/sample"));
        assert_eq!(documents.len(), 2);
    }

    #[test]
    fn test_sample_document_validation() {
        let documents = validate(read_documents("test/sample"));
        assert_eq!(documents.len(), 2);

        let documents = validate(read_documents("test/sample2"));
        assert_eq!(documents.len(), 4);
    }

    #[test]
    fn test_input_read_documents() {
        let documents = read_documents("test/input");
        assert_eq!(documents.len(), 254);
    }

    #[test]
    fn test_input_document_fields() {
        let documents = check_presence(read_documents("test/input"));
        assert_eq!(documents.len(), 190);
    }

    #[test]
    fn test_input_document_validation() {
        let documents = validate(read_documents("test/input"));
        assert_eq!(documents.len(), 121);
    }
}
