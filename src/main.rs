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

//VERSIONE 5
//ora dobbiamo fare una ricerca parziale

//la logica del fare pezzettini piccolini è sbagliata
/*

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn split_query(my_query: String, n: usize) -> Vec<String> {
    let chars: Vec<char> = my_query.to_lowercase().chars().collect();
    //raccogliamo tutti i pezzetti attraverso una finestra mobile
    chars
        .windows(n)
        .map(|finestra| finestra.iter().collect())
        .collect()
}

//con questa funzione sopra abbiamo trasformato la nostra query in un vettore di stringhe
//dove ci son tutti pezzettini di lunghezza n

//creiamone una per calcolare l'indice di jaccard fra due hash set == i due contenitori dei pieces
fn jaccard(set1: &HashSet<&str>, set2: &HashSet<&str>) -> f64 {
    let intersection = set1.intersection(set2).count() as f64;
    let union = (set1.len() + set2.len()) as f64 - intersection;
    if union == 0.0 {
        1.0
    } else {
        intersection / union
    }
}

fn main() -> io::Result<()> {
    let file_path = "testo.txt";
    let n = 3;
    let my_query = "imeless, an".to_string();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut all_pieces = Vec::new();
    for line_result in reader.lines() {
        let row = line_result?;
        let pieces_of_row = split_query(row, n);
        all_pieces.extend(pieces_of_row);
    }

    //stampiamo i primi 100 pezzetti per vederli
    for n_pieces in all_pieces.iter().take(100) {
        println!("- '{}'", n_pieces);
    }

    let query_pieces = split_query(my_query, n);
    let query_set: HashSet<&str> = query_pieces.iter().map(AsRef::as_ref).collect(); //così ora è una hashset

    let mut best_match: Option<Vec<String>> = None;
    let mut best_score = -1.0; //inizializziamo con punteggio imposibile

    for window in all_pieces.windows(query_pieces.len()) {
        let window_set: HashSet<&str> = window.iter().map(AsRef::as_ref).collect();
        let score = jaccard(&query_set, &window_set);
        if score > best_score {
            best_score = score;
            best_match = Some(window.to_vec());
        }
    }

    if let Some(match_trovato) = best_match {
        println!("La corrispondenza più simile trovata è:");
        println!("- Sequenza: {:?}", match_trovato);
        println!("- Punteggio di similarità (Jaccard): {:.2}", best_score);
    } else {
        println!("Nessuna corrispondenza trovata.");
    }

    Ok(())
}

*/

//nuova strategia versione 5
//riutilizziamo tokenize della versione 4

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

///fuzzy search with score
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///what do you want to search in text?
    query: String,

    ///in which text file do you want to search?
    file_path: String,

    ///choose the min score of the entire string : how similar should the two strings be?
    threshold: f64,

    ///decide if you want case-sensitive search (true) or not case sensitive search (false, default)
    #[arg(short, long)]
    case_sensitive: bool,

    ///decide if you want to include special character (true) or exclude (false, default)
    #[arg(long, default_value_t = false)]
    include_special: bool,
}

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

fn transform_in_string(tokens: &[TokenType]) -> String {
    tokens
        .iter()
        .map(|t| t.as_str())
        .collect::<Vec<String>>()
        .join(" ")
}

fn find_best_token_sequence(
    all_tokens: &[TokenType],
    query_tokens: &[TokenType],
    threshold: f64,
    score_min: f64,
    include_special: bool,
    original_query: &str,
) -> Vec<(Vec<TokenType>, f64)> {
    let (all_tokens_to_search, query_tokens_to_search) = if !include_special {
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
        return Vec::new();
    }

    let mut first_match = Vec::new();

    for window in all_tokens_to_search.windows(query_tokens_to_search.len()) {
        let score: f64 = query_tokens_to_search
            .iter()
            .zip(window.iter())
            .map(|(query_tok, text_tok)| {
                let query_str = query_tok.as_str();
                if let TokenType::Word(_) = query_tok {
                    if query_str.len() <= 3 {
                        if query_tok == text_tok { 1.0 } else { 0.0 }
                    } else {
                        strsim::normalized_damerau_levenshtein(&query_str, &text_tok.as_str())
                    }
                } else if query_tok == text_tok {
                    1.0
                } else {
                    0.0
                }
            })
            .sum::<f64>()
            / query_tokens_to_search.len() as f64;

        if score >= threshold {
            first_match.push(window.to_vec());
        }
    }
    let mut results_with_score = Vec::new();
    for sequence in first_match {
        let match_string = transform_in_string(&sequence);
        let final_score = strsim::normalized_damerau_levenshtein(original_query, &match_string);
        if final_score >= score_min {
            results_with_score.push((sequence, final_score));
        }
    }
    results_with_score
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let query_phrase = args.query;
    let file_path = args.file_path;

    let query_tokens = tokenize(&query_phrase, args.case_sensitive);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut all_tokens = Vec::new();
    for line in reader.lines() {
        all_tokens.extend(tokenize(&line?, args.case_sensitive));
    }

    println!(
        "i search {:?} with a score  >= {}...",
        query_tokens, args.threshold
    );

    let results = find_best_token_sequence(
        &all_tokens,
        &query_tokens,
        0.5,
        args.threshold,
        args.include_special,
        &query_phrase,
    );

    if results.is_empty() {
        println!("no correspondences enough similar ");
    } else {
        println!("find {} correspondences :", results.len());
        for found_sequence in results {
            println!("- {:?}", found_sequence);
        }
    }

    Ok(())
}
