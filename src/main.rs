use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about = "CLI to perform amyloidogenesis prediction from protein sequences.", long_about = None)]
struct Args {
    /// FASTA formatted string
    sequence: String,

    #[arg(short, long, help = "Path to FASTA file")]
    file: bool
}

struct Fasta {
    description: String,
    sequence: String
}

impl Fasta {
    fn from(s: String) -> Result<Fasta, String> {
        // TODO parsing
        // TODO return line that can't be parsed wrapped in Err()

        return Ok(Fasta {
            description: String::from(""),
            sequence: String::from("")
        })
    }
}

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
M  methionine            *  translation stop
N  asparagine            -  gap of indeterminate length
 */

fn amyloid_pred(_sequence: &str) -> f64 {
    return 3.14159
}

fn main() {
    // TODO handle input/files with multiple sequences
    // TODO clap error handling
    let args = Args::parse();

    if args.file {
        let file = fs::read_to_string(args.sequence);

        match file {
            Ok(contents) => {
                let fasta = Fasta::from(contents);

                match fasta {
                    Ok(s) => {
                        println!("{:.4}", amyloid_pred(&s.sequence));
                    },
                    Err(e) => {
                        println!("Error: cannot parse <SEQUENCE> as FASTA\nFailed at: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Error: cannot open/read file: \n{}", e);
            }
        }
    }
    else {
        let fasta = Fasta::from(args.sequence);

        match fasta {
            Ok(s) => {
                println!("{:.4}", amyloid_pred(&s.sequence));
            },
            Err(e) => {
                println!("Error: cannot parse <SEQUENCE> as FASTA\nFailed at: {}", e);
            }
        }
    }
}