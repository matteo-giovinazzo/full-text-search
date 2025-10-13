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

use levenshtein::levenshtein;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn fuzzy_search(file_path: &str, parola: &str, tolleranza: usize) -> io::Result<Option<String>> {
    let file = File::open(file_path)?;
    let read = BufReader::new(file);

    for riga_letta in read.lines() {
        let riga = riga_letta?;

        for parola_nella_riga in riga.split_whitespace() {
            //controlliamo le parole in ciascuna riga una per una splittando per gli spazi bianchi
            let distanza = levenshtein(parola, parola_nella_riga); //definiamo la 
            //distanza di lev fra ciascuna parola nella riga e la parola cercata
            if distanza <= tolleranza {
                return Ok(Some(parola_nella_riga.to_string()));
            }
        }
    }
    Ok(None) //se non trovo nessuna corripondenza entro la tolleranza stabilita
}

//adattiamo la main per gestire un risultato più ricco

fn main() -> io::Result<()> {
    let testo = "testo.txt";
    let parola_cercata = "bran";
    let tollera = 1; //1 cancellazione oppure 1 sostituzione fra la parola cercata e quella nel testo  

    println!(
        "cerchiamo la parola {} nel testo {} con una tolleranza di {}",
        parola_cercata, testo, tollera
    );

    match fuzzy_search(testo, parola_cercata, tollera) {
        Ok(Some(parola_trovata)) => {
            println!("trovata una corrispondenza simile : {} ", parola_trovata);
        }
        Ok(None) => {
            println!("nessuna corrispondenza simile trovata ");
        }
        Err(e) => {
            eprintln!("errore nella lettura del file {}", e);
        }
    }
    Ok(())
}
