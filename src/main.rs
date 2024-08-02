mod check_sum;
mod coding;
mod image;
mod input;

use crate::image::compression::Compression;
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

    let image = coding::image::ImageCoding {
        input_path: "/home/stefan/Dokumente/uni/1584_Propra/KE1/KE1_TestBilder/einPixel.tga"
            .to_string(),
        output_path: "/home/stefan/Dokumente/rust/propra/img/einPixel.propra".to_string(),
        compression: Compression::Uncompressed,
    };
    let result: Result<coding::Type, &'static str> = Ok(coding::Type::Image(image));

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

use crate::check_sum::CheckSum;
use std::fs::File;
use std::io::Read;

pub fn reader() -> std::io::Result<()> {
    let mut f =
        File::open("/home/stefan/Dokumente/uni/1584_Propra/KE1/KE1_TestBilder/Oversize.propra")?;
    let mut buffer: [u8; 9000] = [0; 9000];

    let mut checksum = CheckSum::new();

    // read up to 10 bytes
    let mut n = f.read(&mut buffer[..])?;
    let mut offset = 30;
    while n > 0 {
        for i in &buffer[offset..n] {
            checksum.add(*i);
        }
        offset = 0;

        n = f.read(&mut buffer[..])?;
    }
    println!("Prüfsumme beträgt {}", checksum.hex());
    println!("The bytes: {:?}", &buffer[..100]);
    Ok(())
}
