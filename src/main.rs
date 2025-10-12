//l'obiettivo è quello di scrivere un codice che compia una ricerca in un file di testo

//versione base

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parolaesiste(file_path: &str, parola: &str) -> io::Result<bool> {
    let file = File::open(file_path)?;
    let read = BufReader::new(file);

    for pippo in read.lines() {
        let pluto = pippo?;

        if pluto.contains(parola) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn main() -> io::Result<()> {
    let risposta = parolaesiste("filetesto.txt", "Classic");
    if risposta? {
        println!("la parola cercata c'è");
    } else {
        println!("la parola non è stata trovata");
    }

    Ok(())
}
