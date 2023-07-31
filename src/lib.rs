/*

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
/*

use std::collections::HashMap;

pub fn count(nucleotide:char,dna:&str) ->Result<usize,char>{
    let mut map:HashMap<char,usize>=[('A',0),('T',0),('G',0),('C',0)].iter().cloned().collect();
    for c in dna.chars(){
        match c {
            'A' | 'C' | 'G' | 'T' =>{
                let count=map.get_mut(&c).unwrap();
                *count += 1;
            },
            _=>{ return Err(c);}
        }
    }
    map.remove(&nucleotide).ok_or(nucleotide)
}
pub fn nucleotide_counts(dna:&str) -> Result<HashMap<char,usize>,char>{
    let mut map:HashMap<char,usize>=
    [('A',0),('C',0),('G',0),('T',0)].iter().cloned().collect();
    for c in dna.chars(){
        match c{
            'A' | 'C' | 'G' | 'T' =>{
                let count =map.get_mut(&c).unwrap();
                *count +=1;
            },
            _=>{ return Err(c);}
        }
    }
    Ok(map)
}

*/

// now baby it's my style---be ready--
//use std::collections::HashMap;
//use crate::HashMap;

pub mod DNA_count{
use std::collections::HashMap;
pub fn count_me(seq:&String)->i32 {
    let seq_byte=seq.as_bytes();
    let mut countA:i32=1;
    let countT:i32=0;
    let countG:i32=0;
    let countC:i32=0;
    let mut temp:i32=0;
    let mut v:Vec<i32>=Vec::new();
    for(i,&item) in seq_byte.iter().enumerate(){
        match item{
            b'A' => {
                let tmp=countA;
                countA +=1;
                v.push(tmp);
                temp
                
            },
            //b'T' =>countT+1,
            //b'G'=> countG+1,
            //b'C'=> countC+1,
            _=>panic!("It seems your code is not giving the output you want!"),

        };
    }
    //println!("{}",countA);
    //countA=countA+1;
    //println!("{}",countA);
    //return countA
    println!("{:?}",v);
    return temp
}



}


 