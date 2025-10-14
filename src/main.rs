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

use levenshtein::levenshtein;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn rimuovi_punteggiatura(testo: &str) -> String {
    testo
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

fn fuzzy_search(
    file_path: &str,
    parola: &str,
    tolleranza: usize,
    nosense: bool,
) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let read = BufReader::new(file);

    let mut parole_trovate = Vec::new(); //voglio tutte le occorrenze di una certa parola che cerco 
    //sennò quando scrivo AI mi trova la A iniziale e basta perchè soddisfa una tolleranza di 1 e si ferma lì

    for riga_letta in read.lines() {
        let riga = riga_letta?;

        for parola_nella_riga in riga.split_whitespace() {
            let no_punt = rimuovi_punteggiatura(parola_nella_riga);
            //controlliamo le parole in ciascuna riga una per una splittando per gli spazi bianchi
            //NOTA BENE
            //è POSSIBILE CHE ALCUNE PAROLE SIANO FRA () OPPURE ABBIANO UN . ALLA FINE O UNA ,
            //--> DOBBIAMO ELIMINARE LA PUNTEGGIATURA
            //let p: &[_] = &[',', '.', '(', ')', '"'];
            //let no_punteggiatura = parola_nella_riga.trim_matches(p);
            //VA TOLTA ANCHE LA PUNTEGGIATURA ALL'INTERNO DELLE STRINGHE, AD ESEMPIO LE APOSTROFO E LE -
            //come fare ?
            //gemini consiglia di utilizzare una funzione esterna che toglie ogni cosa tutta insieme

            let distanza: usize = if nosense {
                levenshtein(&parola.to_lowercase(), &no_punt.to_lowercase())
            } else {
                levenshtein(parola, &no_punt)
            };

            if distanza <= tolleranza {
                parole_trovate.push(parola_nella_riga.to_string());
            }
        }
    }
    Ok(parole_trovate) //restituisco il vettore con le corrispondenze 
}

//adattiamo la main per gestire un risultato più ricco

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let testo = "testo.txt";
    //let parola_cercata = "tsle";
    let tollera = 0; //numero di cancellazione/sostituzione/inserzione fra la parola cercata e quella nel testo  
    let no_sensitive = false;
    let parola_cercata = &args[1];

    println!(
        "cerchiamo la parola {} nel testo {} con una tolleranza di {}",
        parola_cercata, testo, tollera
    );

    match fuzzy_search(testo, parola_cercata, tollera, no_sensitive) {
        Ok(parole_trovate) => {
            if parole_trovate.is_empty() {
                println!("nessuna corrispondenza per {}", parola_cercata);
            } else {
                println!(
                    "trovate {} corrispondenze per {}",
                    parole_trovate.len(),
                    parola_cercata
                );
            }
        }
        Err(e) => {
            eprintln!("errore nella lettura del file {}", e);
        }
    }
    Ok(())
}
