#[derive(Debug)]
#[allow(dead_code)]
pub struct BaseCoding {
    input_path: String,
    alphabet: String,
    coding: Coding,
}

#[derive(Debug, PartialEq, Eq)]
enum Coding {
    DecodeBase32,
    EncodeBase32,
    DecodeBaseN,
    EncodeBaseN,
}

impl BaseCoding {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        let input_path = match BaseCoding::set_input_path(args) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let coding = match BaseCoding::set_coding(args) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        if coding == Coding::EncodeBaseN && !input_path.ends_with(".base-n") {
            return Err("Dateiendung ungültig. Endung .base-n erwartet.");
        }

        let mut alphabet = String::new();
        if coding == Coding::EncodeBaseN {
            alphabet = match BaseCoding::set_alphabet(args) {
                Ok(a) => a,
                Err(e) => return Err(e),
            };
        }

        Ok(BaseCoding {
            input_path,
            alphabet,
            coding,
        })
    }

    fn set_input_path(args: &[String]) -> Result<String, &'static str> {
        for s in args.iter() {
            match s {
                s if s.starts_with("--input=") => return Ok(s.clone().split_off(8)),
                _ => (),
            }
        }
        Err("Kein --input Parameter gefunden")
    }

    fn set_coding(args: &[String]) -> Result<Coding, &'static str> {
        for s in args.iter() {
            match s {
                s if s.eq("--decode-base-32") => return Ok(Coding::DecodeBase32),
                s if s.eq("--encode-base-32") => return Ok(Coding::EncodeBase32),
                s if s.eq("--decode-base-n") => return Ok(Coding::DecodeBaseN),
                s if s.starts_with("--encode-base-n") => return Ok(Coding::EncodeBaseN),
                _ => (),
            }
        }
        Err("Keine gültige Codierung angegeben")
    }

    fn set_alphabet(args: &[String]) -> Result<String, &'static str> {
        for s in args.iter() {
            if s.starts_with("--encode-base-n") && (s.len() > 16) {
                return Ok(s.clone().split_off(16));
            }
        }
        Err("Kein Alphabet für die Codierung angegeben.")
    }
}
