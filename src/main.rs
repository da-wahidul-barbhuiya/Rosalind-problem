/*
//mod lib;
use DNA_count::{nucleotide_counts,count};
fn main(){
    //println!("Hello world!");
    let sequence=String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    let a=nucleotide_counts(&sequence);
    let a=count('A',&sequence);
}

*/

use Nucleotide_operation::{DNA_count::count_me,DNA_to_RNA::transcription};
// Nucleotide_operation is coming from Cargo.toml file and provides a relative path to lib.rs modules




fn main(){
    //let sequence=String::from("AGACTCTGCGCGGACTACTAAGCCGGGTCCCTGGTTGGAGCGTATTCCTGGTGCAGAGAATCCACGGTTGATGGAACAAGAGGACACCACCATGACGTGGCTGAAGCGGTGCGCGCCATGTCCCAGAACTTGGTAAGTCCCTCCAGCTCGGACCGCGCTCTTGTTATACACGCGAGAACGATAGATCGGCCGGCGATGAGGTGCCTAATATCACGAGGCTCCGGTCAGCTGGTGCGACCGGGTTCGTCCGGTGTGGCAAAAAGGTCCTCCACGTGCATTATCGCACTAAAGCCCAGCTAAAATGGTGGCCGTAACCAGGGGGGTGTGGGGGGAGAGCCTTTTCGTTAGCTTGCGATTACGATTCTATTTGCTTATTTTTTGTGATAGCCGCGTATGCAATCAATAGGGCAACGTCATGGTAGCAGCTCTACGGCTTCTTGTGTCGATGAAACCTCCGTTCTACTCCGCTCATACTCTGCTGCTTGGCGACCTGTCCCTAAAAAGTATTTGCCATGTTGTGTATACCGTTTTCCGCGACTAATGGTCAGGCCGGATTTAAGGGTAACTGATCAATTCGTGGGTCCGACTAAACGGCTGAAGGGACCCCAGCAACCTGCCGCTTCCGAGAACCCATCATCCAAATTCTACCGTGCTCCTCCATAGCGCCCGTCGGTCTCGGTGTTAATGGAGAAGGTCGAGAGTGGTCCAAGTGCACCCATTGTAGTTGAGACCATTCGAGTAGATCCAGGCCTCACTCTGCTCGCTGTCTTGGGTGAGACATAATCCAACGGCCATGAGTTACGGACACATCTCGAACAAAGTTAGTGTACTAAGCGCATACAAGCTTCCGGGCCTCTCATCCCAAAGACTTTGCTGGTAGCGCGCTGCCAATACGGTGAGTACTCAAGGACGTTTTAATGGTACGTGGGCCCAACGCCG");
    let sequence=String::from("ATGAGTTCA");
    let a= transcription(&sequence);
    let b=count_me(&sequence);
    //let b=check_point(&sequence);
}