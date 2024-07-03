use clap::Parser;
use std::fs;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "CLI to perform amyloidogenesis prediction from protein sequences.",
    long_about = None,
    color = clap::ColorChoice::Never
)]
struct Args {
    #[arg(
        help = "Path to FASTA file",
        long_help = "Path to file with one or more FASTA formatted sequences, each of which must match: \nr\"(?<des>>.*)\\n(?<seq>([A-IK-NP-Z]+\\n?)+)\""
    )]
    path: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Fasta {
    description: String,
    sequence: String
}

impl Fasta {
    fn from(des: &str, seq: &str) -> Fasta {
        return Fasta {
            description: String::from(des),
            sequence: String::from(seq)
        }
    }
}

fn parse_fasta(s: String) -> Result<Vec<Fasta>, String> {
    let mut fastas: Vec<Fasta> = Vec::new();

    let re = Regex::new(r"(?<des>>.*)\n(?<seq>([A-IK-NP-Z]+\n?)+)").unwrap();

    for (_, [des, seq, _]) in re.captures_iter(&s).map(|c| c.extract()) {
        fastas.push(Fasta::from(des, seq));
    }

    if fastas.len() == 0 {
        return Err(String::from("no sequences found"))
    }
    else {
        return Ok(fastas)
    }
}

fn amyloid_pred(_sequence: &str) -> f64 {
    /*
    see https://blast.ncbi.nlm.nih.gov/doc/blast-topics/#fasta
    A  alanine               P  proline
    B  aspartate/asparagine  Q  glutamine
    C  cystine               R  arginine
    D  aspartate             S  serine
    E  glutamate             T  threonine
    F  phenylalanine         U  selenocysteine
    G  glycine               V  valine
    H  histidine             W  tryptophan
    I  isoleucine            Y  tyrosine
    K  lysine                Z  glutamate/glutamine
    L  leucine               X  any
    M  methionine
    N  asparagine
     */

    return std::f64::consts::PI
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(args.path);

    match file {
        Ok(contents) => {
            let fastas = parse_fasta(contents);

            match fastas {
                Ok(f) => {
                    for fasta in f {
                        println!("{:.4}", amyloid_pred(&fasta.sequence));
                    }
                },
                Err(e) => println!("error: {}", e)
            }
        },
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // parse file with single FASTA formatted sequence
    fn single_fasta_parse() {
        let file = fs::read_to_string("test/single.fasta").unwrap();
        let fastas = parse_fasta(file).unwrap();

        assert_eq!(1, fastas.len());
    }

    #[test]
    // parse file with five FASTA formatted sequences
    fn multi_fasta_parse() {
        let file = fs::read_to_string("test/multi.fasta").unwrap();
        let fastas = parse_fasta(file).unwrap();

        assert_eq!(5, fastas.len());
    }

    #[test]
    // parse file with single FASTA formatted sequence
    // with additional non-sequence characters
    fn single_fuzzy_fasta_parse() {
        let file = fs::read_to_string("test/single_fuzzy.fasta").unwrap();
        let fastas = parse_fasta(file).unwrap();

        assert_eq!(1, fastas.len());
    }

    #[test]
    // parse file with five FASTA formatted sequences
    // with additional non-sequence characters
    fn multi_fuzzy_fasta_parse() {
        let file = fs::read_to_string("test/multi.fasta").unwrap();
        let fastas = parse_fasta(file).unwrap();

        assert_eq!(5, fastas.len());
    }
}