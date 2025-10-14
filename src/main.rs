//l'obiettivo è quello di scrivere un codice che compia una ricerca in un file di testo

//versione base
//ricerca semplice di un'occorrenza in un testo

//VERSIONE 2
//adesso, integriamo nella ricerca la gestione degli errori
//se nel testo è presente palone o pallonr invece di pallone, la funzione dovrà comunque trovare tale occorrenza
//sarà quindi una sorta di fuzzy search
//utilizzeremo la distanza di Levenshtein per valutare quanto la stringa trovata
//sia distante dalla stringa corretta e decidere se accettarla o no
//quindi se la distanza fra la parola cercata e ciascuna occorrenza che le assomigli
//renda una distanza di lev minore del limite prefissato, andrà bene

//VERSIONE 3
//utlizziamo la distanza di Damerau-Levenshtain
//a differenza della precedente che implementava le operazioni di insertions, deletions and substitutions,
//essa permette anche la trasposizione dei caratteri ed assegna uno score alle operazioni da compiere per arrivare alla parola cercata
//inoltre questa versione dovrà scegliere se ricercare in modo case insensitive (come fatto finora) oppure
//in modo case sensitive, ottenendo uno score più basso di 1 nel caso in cui si stia cercando pallone e si trova Pallone
//utilizziamo la funzione normalized_damerau_levenshtain la quale calcola uno score normalizzato incluso in [0, 1]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use strsim::normalized_damerau_levenshtein;

fn remove_punctuation(testo: &str) -> String {
    testo
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

fn fuzzy_search(
    file_path: &str,
    parola: &str,
    treshold: f64,
    nosense: String,
) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let read = BufReader::new(file);

    let mut words_finder = Vec::new();

    for riga_letta in read.lines() {
        let riga = riga_letta?;

        for word_in_line in riga.split_whitespace() {
            let no_punt = remove_punctuation(word_in_line);
            let distanza_damlev: f64 = if nosense == "false" {
                normalized_damerau_levenshtein(&parola.to_lowercase(), &no_punt.to_lowercase())
            } else {
                normalized_damerau_levenshtein(parola, &no_punt)
            };

            if distanza_damlev >= treshold {
                words_finder.push(word_in_line.to_string());
            }
        }
    }
    Ok(words_finder) //restituisco il vettore con le corrispondenze 
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let testo = "testo.txt";
    let treshold: f64 = 0.6; //score distanza damlev
    let word_to_find = &args[1];
    let no_sensitive = &args[2];

    println!(
        "cerchiamo la parola {} nel testo {} con una tolleranza di {}",
        word_to_find, testo, treshold
    );

    match fuzzy_search(testo, word_to_find, treshold, no_sensitive.to_string()) {
        Ok(words_finder) => {
            if words_finder.is_empty() {
                println!("nessuna corrispondenza per {}", word_to_find);
            } else {
                println!(
                    "trovate {} corrispondenze per {}",
                    words_finder.len(),
                    word_to_find
                );
            }
        }
        Err(e) => {
            eprintln!("errore nella lettura del file {}", e);
        }
    }
    Ok(())
}
