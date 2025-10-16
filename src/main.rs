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

use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Word(String),
    Number(f64),
    Punctuation(char),
    SpecialCharacter(char),
}

///ricerca testo in file di testo txt
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///cerca la parola o frase o collezione di caratteri
    query: String,

    ///il file nella quale cercare si chiama
    file_path: String,

    ///decidi se ricerca case sensitive (true) o case insensitive (false)
    #[arg(short, long)]
    case_sensitive: bool,

    /// Ignora la punteggiatura e i caratteri speciali nella ricerca (true) o mantieni tutto (false)
    #[arg(short = 's', long)]
    ignore_special: bool,
}

//funzione di suporto per scrivere il testo
impl TokenType {
    fn as_str(&self) -> String {
        match self {
            TokenType::Word(s) => s.clone(),
            TokenType::Number(n) => n.to_string(),
            TokenType::Punctuation(c) => c.to_string(),
            TokenType::SpecialCharacter(c) => c.to_string(),
        }
    }
}

fn tokenize(testo: &str, case_sensitive: bool) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut chars = testo.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() {
            let mut single_word = String::new();
            while let Some(ch) = chars.peek() {
                if ch.is_alphabetic() {
                    single_word.push(chars.next().unwrap())
                } else {
                    break;
                }
            }
            let final_word = if case_sensitive {
                single_word
            } else {
                single_word.to_lowercase()
            };
            tokens.push(TokenType::Word(final_word));
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
        } else if c == ',' || c == '.' || c == '\'' {
            tokens.push(TokenType::Punctuation(chars.next().unwrap()));
        } else {
            tokens.push(TokenType::SpecialCharacter(chars.next().unwrap()));
        }
    }
    tokens
}

fn find_best_token_sequence(
    all_tokens: &[TokenType],
    query_tokens: &[TokenType],
    threshold: f64,
    ignore_special: bool,
) -> Option<Vec<TokenType>> {
    //un controllo "obbligatorio"
    let (all_tokens_to_search, query_tokens_to_search) = if ignore_special {
        let filtered_all = all_tokens
            .iter()
            .filter(|t| matches!(t, TokenType::Word(_)))
            .cloned()
            .collect::<Vec<_>>();
        let filtered_query = query_tokens
            .iter()
            .filter(|t| matches!(t, TokenType::Word(_)))
            .cloned()
            .collect::<Vec<_>>();
        (filtered_all, filtered_query)
    } else {
        (all_tokens.to_vec(), query_tokens.to_vec())
    };

    if query_tokens_to_search.is_empty()
        || all_tokens_to_search.len() < query_tokens_to_search.len()
    {
        return None;
    }

    let mut best_match: Option<&[TokenType]> = None;
    let mut best_score = -1.0; //assegno punteggio impossibile

    for window in all_tokens_to_search.windows(query_tokens_to_search.len()) {
        let score: f64 = query_tokens_to_search
            .iter()
            .zip(window.iter())
            .map(|(query_tok, text_tok)| {
                strsim::normalized_damerau_levenshtein(&query_tok.as_str(), &text_tok.as_str())
            })
            .sum::<f64>()
            / query_tokens.len() as f64;

        if score > best_score {
            best_score = score;
            best_match = Some(window);
        }
    }

    if best_score >= threshold {
        best_match.map(|slice| slice.to_vec())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // Unisce tutti gli argomenti in un'unica frase di ricerca
    let query_phrase = args.query;
    let threshold: f64 = 0.8;
    let file_path = args.file_path;

    // Tokenizza sia la query che il file
    let query_tokens = tokenize(&query_phrase, args.case_sensitive);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut all_tokens = Vec::new();
    for line in reader.lines() {
        all_tokens.extend(tokenize(&line?, args.case_sensitive));
    }

    println!(
        "Cerco la sequenza {:?} con soglia >= {}...",
        query_tokens, threshold
    );

    match find_best_token_sequence(&all_tokens, &query_tokens, threshold, args.ignore_special) {
        Some(found_sequence) => {
            println!(r"\Trovata la migliore corrispondenza: {:?}", found_sequence);
        }
        None => {
            println!("Nessuna corrispondenza sufficientemente simile trovata.");
        }
    }

    Ok(())
}
