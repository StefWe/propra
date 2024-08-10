mod check_sum;
mod coding;
mod image;
mod input;

use crate::image::Image;
use input::validation;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let result = match args.len() {
        3 | 4 => match validation(&args) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        _ => {
            print_program_description();
            Err("Programm beendet.")
        }
    };

    match result {
        Ok(s) => run_job(s),
        Err(e) => println!("{}", e),
    }
}

fn run_job(job: coding::Type) {
    match job {
        coding::Type::Image(i) => {
            println!("Das ist die Quelle {:#?}", i);
            Image::produce(i);
            println!("Bild from_propra erzeugt");
        }
        coding::Type::Base(b) => println!("BaseX coding not yet implemented {:#?}", b),
    }
}

fn print_program_description() {
    println!("Das Programm konvertiert und komprimiert Bilder vom TGA-Format in das PROPRA-Format und umgekehrt mit optionaler Lauflängenkodierung.");
    println!();
    println!(
        "Es muss eine Eingabedatei(--input=) und eine Ausgabedatei(--output=) übergeben werden."
    );
    println!("Ein Aufruf sieht dann z.B. wie folgt aus: ");
    println!("--input=../KE1_TestBilder/test_01_uncompressed.tga --output=../KE1_Konvertiert/test_01.propra");
    println!("--input=../KE1_TestBilder/test_03_uncompressed.propra --output=../KE1_Konvertiert/test_03.tga");
    println!();
    println!("oder mit Bild (De)Kompriemierung:");
    println!("--input=../KE1_TestBilder/test_01_rle.tga --output=../KE1_Konvertiert/test_01.propra --compression=uncompressed");
    println!("--input=../KE1_TestBilder/test_03_uncompressed.propra --output=../KE1_Konvertiert/test_03_rle.tga --compression=rle");
    println!();
    println!("Das Programm kann auch Base32 Kodierung und Decodierung.");
    println!("Ein Aufruf sieht dann z.B. wie folgt aus: ");
    println!("--input=../KE2_TestBilder/test_06_base32.propra.base-32 --decode-base-32");
    println!("--input=../KE2_TestBilder/test_02_rle.tga               --encode-base-32");
}
