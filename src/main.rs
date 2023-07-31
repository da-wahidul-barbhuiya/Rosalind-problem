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

use DNA_count::DNA_count::count_me;

fn main(){
    let sequence=String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    let a= count_me(&sequence);
}