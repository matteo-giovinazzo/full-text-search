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

//VERSIONE 4
//va fatto un parser del testo
//dobbiamo categorizzare i singoli caratteri

#[derive(Debug)]
enum TokenType {
    Word(String), //sequenza di caratteri alfabetici quindin una parola
    Number(f64),
    Punctuation(char), //singolo carattere di punteggiatura
}

fn tokenize(testo: &str) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut chars = testo.chars().peekable();
    //si usa peek e non next perchè si sbircia sul crattere successivo senza consumarlo
    //così da capire se continuare a costruire una parola o un numero
    while let Some(&c) = chars.peek() {
        //inizia un while che continua finchè c'è un carattere dopo da sbirciare con peek
        //&c --> c sarà una copia del carattere corrente che decideremo dove buttarlo
        if c.is_alphabetic() {
            //in questo caso sta iniziando una parola o sta continuando
            let mut single_word = String::new();
            while let Some(ch) = chars.peek() {
                if ch.is_alphabetic() {
                    single_word.push(chars.next().unwrap())
                } else {
                    break;
                }
            }
            tokens.push(TokenType::Word(single_word.to_lowercase()));
        } else if c.is_numeric() {
            let mut number = String::new();
            while let Some(ch) = chars.peek() {
                if ch.is_numeric() {
                    number.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if let Ok(num) = number.parse::<f64>() {
                tokens.push(TokenType::Number(num));
            }
        } else if c.is_whitespace() {
            chars.next();
        } else {
            tokens.push(TokenType::Punctuation(chars.next().unwrap()));
        }
    }
    tokens
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "testo.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Tokenizzazione del file '{}':\n", file_path);

    for (row_number, line_result) in reader.lines().enumerate() {
        let row = line_result?;
        if row.is_empty() {
            continue;
        }

        let tokens = tokenize(&row);
        println!("row {}: {:?}", row_number + 1, tokens);
    }

    Ok(())
}
