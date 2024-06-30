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
    /// FASTA formatted string
    sequence: String,

    #[arg(short, long, help = "Path to FASTA file")]
    file: bool
}

#[derive(Debug)]
struct Fasta {
    description: String,
    sequence: String
}

impl Fasta {
    fn from(s: String) -> Result<Fasta, String> {
        // TODO handle input/files with multiple sequences

        let re_des = Regex::new(r"^>.*").unwrap();
        let re_seq = Regex::new(r"^[A-IK-NP-Z]+$").unwrap();

        let lines: Vec<&str> = s.splitn(2,"\n").collect();
        let des = lines[0];
        let seq = lines[1].replace("\n", "");

        if re_des.is_match(des) {
            if re_seq.is_match(&seq) {
                return Ok(Fasta {
                    description: String::from(des),
                    sequence: seq
                })
            }
            else {
                return Err(seq)
            }
        }
        else {
            return Err(String::from("Bare sequence not allowed"))
        }
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
                        println!("error: cannot parse <SEQUENCE> as FASTA\n{}", e);
                    }
                }
            },
            Err(e) => {
                println!("error: {}", e);
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
                println!("error: cannot parse <SEQUENCE> as FASTA\n{}", e);
            }
        }
    }
}

#[cfg(test)]
// TODO write tests
mod tests {
    use super::*;

    #[test]
    fn single_fasta_parse() {
        let file = fs::read_to_string("test/Q31Q05.fasta").unwrap();
        let fasta = Fasta::from(file).unwrap();

        assert_eq!(fasta.description, String::from(">sp|Q31Q05|RAF1_SYNE7 RuBisCO accumulation factor 1 OS=Synechococcus elongatus (strain ATCC 33912 / PCC 7942 / FACHB-805) OX=1140 GN=raf1 PE=1 SV=1"));
        assert_eq!(fasta.sequence, String::from("MREFTPTTLSEEERQELLGQLRRKEGRWLAWARACQTLLKNGLNPQTLFEATGFEPIQQNQITVAMQVYDSILRQDPPAHVRETYQEWGSDLLYELRELDQEQRSLCAQLALERKLDADQIREVAKATKDFCRLPKQPENFDRHPGDAVAHQCWRLAQERTDLTERSRLIARGLQFAQSAGARALIEALLLDLSGVPSRKPPMLPIYRLETEEDLPRLLPFAGTLPLSSSQIEAIAAVEAEGPFGLVSSPQGQQWLALPGWQAILTAEDPIACLEQIDRLPNAPEGPTEAVVLVVDRADRDWDADHFFLVEQAEGARIQWSPSAIAAPILGRLVLILRPKRVLDEAAIATPWQFEE"));
    }

    #[test]
    fn bare_fasta_parse() {
        let file = fs::read_to_string("test/Q31Q05_bare.fasta").unwrap();
        let fasta = Fasta::from(file);

        assert_eq!(true, fasta.is_err());
    }
}