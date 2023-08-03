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
pub fn count_me(seq:&String)-> Result<HashMap<String,i32>,u8> {
    let seq_byte=seq.as_bytes();
    let mut countA:i32=0;
    let mut countT:i32=0;
    let mut countG:i32=0;
    let mut countC:i32=0;
    let mut temp:i32=0;
    let mut map=HashMap::new();
    for(i,&item) in seq_byte.iter().enumerate(){
        match item{
            b'A' => {
                //let tmp=countA;
                countA +=1;
                //v.push(tmp);
                map.insert(String::from("A"),countA);

            },
            b'T' =>{
                countT+=1;
                map.insert(String::from("T"),countT);

            },
            b'G'=> {
                countG +=1;
                map.insert(String::from("G"),countG);

            },
            b'C'=> {
                countC +=1;
                map.insert(String::from("C"),countC);

            },
            _=>{return Err(item)
            //panic!("It seems your code is not giving the output you want!"),
            }

        };
    }
    println!("Nucleotide count:- {:?}",map);
    Ok(map) 
    
}

}
/*

//transcribing DNA into RNA


pub mod DNA_to_RNA {
    pub struct DNA(String);

    impl DNA {
        pub fn new(input:&str) -> Result<DNA, char>{
            input.chars().map(|c| match c{
                'A' | 'C'| 'G' | 'T' =>Ok(c),
                _=> Err(c),
            })
            .collect::<Result<String,char>>()
            .and_then(|base| Ok(DNA(bases)))
        }
        pub fn to_rna(&self) -> RNA{
            RNA::new(
                self.0.chars().map(|c| match c{
                    'A' =>'U',
                    'C'=>'G',
                    'G'=>'C',
                    'T'=>'A',
                    _=>unreachable!(),
                })
                .collect::<String>().as_str(),
            ).ok()
            .unwrap()
            
        }
    }

    pub struct RNA(String);

    impl RNA{
        pub fn new(input:&str) -> Result<RNA,char>{
            input.chars().map(|c| match c{
                'A' | 'C' |'G' |'U' =>Ok(c),
                _=>Err(c),
            }).collect::<Result<String,char>>().and_then(|base| Ok(RNA(base)))
        }
    }
 

}

*/
pub mod DNA_to_RNA{
        /*
        pub fn check_point(&self,input:&str) -> Result<Converter,char>{
            let transribe=Converter{DNA:String::from("Hello world"),RNA:String::from("Converting...")};
            let input_byte=input.as_bytes();
            let v:Vec<String>=Vec::new();
            for (i,&item) in input_byte.iter().enumerate(){
                match item{
                    b'A' | b'C' | b'G' | b'T' =>Ok(item),
                    _=> Err(item),
                };
            }
            Ok(transribe)
        }
         */
        
        pub fn transcription(input:&str) ->Result<String,u8>{
            let rna_byte=input.as_bytes();
            let mut v:Vec<String>=Vec::new();
            for (i,&item) in rna_byte.iter().enumerate(){
                match item{
                    b'A' => v.push(String::from('U')),
                    b'C'=>v.push(String::from('G')),
                    b'G'=>v.push(String::from('C')),
                    b'T'=>v.push(String::from('A')),
                    _=>{//unreachable!(),
                        return Err(item);
                }
            }
        }
        //Now storing all nt char individually in a vector and then changing vector to a string
        let seq= v.clone().into_iter().collect::<String>();//into_iter() allows us to convert a collection to into an iterator
                                                           //ownership of the colllection is transferred  into the iterator and ]
                                                           // original collection is inaccessable so that's why you have to clone first 
                                                           // .collect() meethod converts iterator to a collection()
        println!("DNA after transcription:- {:?}",seq);
        Ok(seq)
    }
}

