use nom::lib::std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/4.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let raw_passports: Vec<&str> = input.split("\n\n").collect();

    let mut counter = 0;
    let mut counter2 = 0;

    for raw_passport in raw_passports {
        let p = Document::from_string(raw_passport);
        if p.is_valid_naive() {
            counter += 1
        }
        if p.is_valid() {
            counter2 += 1;
        }
    }

    println!("First: {}, Second: {}", counter, counter2);
}

#[derive(Default, Debug)]
struct Document {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Document {
    fn from_string(input: &str) -> Document {
        let inp = input.replace("\n", " ");
        let parts: Vec<&str> = inp.split(' ').collect();

        let mut hm: HashMap<&str, String> = HashMap::new();
        for part in parts {
            let split_part: Vec<&str> = part.split(':').collect();
            hm.insert(split_part[0], split_part[1].to_string());
        }

        let mut doc = Document::default();
        doc.byr = match hm.get("byr") {
            Some(byr) => Some(byr.parse().unwrap()),
            None => None,
        };
        doc.iyr = match hm.get("iyr") {
            Some(byr) => Some(byr.parse().unwrap()),
            None => None,
        };
        doc.eyr = match hm.get("eyr") {
            Some(byr) => Some(byr.parse().unwrap()),
            None => None,
        };
        doc.hgt = hm.get("hgt").cloned();
        doc.hcl = hm.get("hcl").cloned();
        doc.ecl = hm.get("ecl").cloned();
        doc.pid = hm.get("pid").cloned();
        doc.cid = hm.get("cid").cloned();

        doc
    }

    fn is_valid_naive(&self) -> bool {
        let mut count = 0;

        if self.byr.is_some() {
            count += 1
        };
        if self.iyr.is_some() {
            count += 1
        };
        if self.eyr.is_some() {
            count += 1
        };
        if self.hgt.is_some() {
            count += 1
        };
        if self.hcl.is_some() {
            count += 1
        };
        if self.ecl.is_some() {
            count += 1
        };
        if self.pid.is_some() {
            count += 1
        };
        if self.cid.is_some() {
            count += 1
        };

        count == 8 || (count == 7 && self.cid.is_none())
    }

    fn is_valid(&self) -> bool {
        // birth year
        match self.byr {
            Some(byr) => {
                if byr < 1920 || byr > 2002 {
                    return false;
                }
            }
            None => return false,
        }

        // issue year
        match self.iyr {
            Some(iyr) => {
                if iyr < 2010 || iyr > 2020 {
                    return false;
                }
            }
            None => return false,
        }

        // expiration year
        match self.eyr {
            Some(eyr) => {
                if eyr < 2020 || eyr > 2030 {
                    return false;
                }
            }
            None => return false,
        }

        let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match &self.ecl {
            Some(ecl) => {
                if !valid_ecl.contains(&ecl.as_str()) {
                    return false;
                }
            }
            None => return false,
        }

        match &self.pid {
            Some(pid) => {
                if pid.len() == 9 && pid.parse::<u64>().is_ok() {
                } else {
                    return false;
                }
            }
            None => return false,
        }

        match &self.hgt {
            Some(hgt) => {
                if !Self::validate_height(hgt) {
                    return false;
                }
            }
            None => return false,
        }

        match &self.hcl {
            Some(hcl) => {
                if !Self::validate_hair_color(hcl) {
                    return false;
                }
            }
            None => return false,
        }

        true
    }

    fn validate_height(input: &str) -> bool {
        let len = input[0..input.len() - 2].parse::<u32>();
        if len.is_err() {
            return false;
        }

        let len = len.unwrap();

        match &input[input.len() - 2..] {
            "cm" => {
                if len < 150 || len > 193 {
                    return false;
                }
            }
            "in" => {
                if len < 59 || len > 76 {
                    return false;
                }
            }
            _ => return false,
        }

        true
    }

    fn validate_hair_color(input: &str) -> bool {
        if &input[0..1] != "#" {
            return false;
        }
        if input.len() != 7 {
            return false;
        }
        let num_is_ok = u32::from_str_radix(&input[1..], 16).is_ok();

        if !num_is_ok {
            return false;
        }
        true
    }
}
