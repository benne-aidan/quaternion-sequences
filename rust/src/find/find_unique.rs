

use std::collections::HashSet;


use crate::sequences::{equivalence::{generate_equivalence_class, will_less_than}, symmetries::SequenceType, williamson::{QuadSeq, QUADRUPLETS}};



fn equals(seq1 : &Vec<i8>, seq2 : &Vec<i8>) -> bool {
    for index in 0..seq1.len(){
        if seq1[index] != seq2[index] {
            return false;
        }
    }
    true
}



fn contains(set : &Vec<QuadSeq>, seq : &QuadSeq) -> bool {
    let (a1,b1,c1,d1) = seq.sequences();
    for elm in set {
        let (a2,b2,c2,d2) = elm.sequences();
        if equals(&a1, &a2) && equals(&b1, &b2) && equals(&c1, &c2) && equals(&d1, &d2) {
            return true;
        }
    }
    false
}



fn find_minimum(class : &HashSet<QuadSeq>) -> QuadSeq {
    // finds the minimum of a set comparing with the lexical order
    let mut mini = None;

    for elm in class {
        if let Some(best) = mini{
            if will_less_than(elm, best) {
                mini = Some(elm);
            }
        }
        else {
            mini = Some(elm);
        }
    }

    mini.expect("Error: Empty class !").clone()
}



pub fn find(size : usize) -> String{
    // Finds sequences and reduces the set found up to equivalence
    let sequences = find_aux(size);
    
    eprintln!("The function found {} sequences before equivalences", sequences.len());

    let mut classes : Vec<HashSet<QuadSeq>> = vec![];

    for seq in sequences {
        let mut new_seq = true;
        for class in &classes {
            if class.contains(&seq) {
                new_seq = false;
                break;
            }
        }
        if new_seq {
            let new_class = generate_equivalence_class(&seq, SequenceType::QuaternionType);
            classes.push(new_class);
        }
    }

    let count : usize = classes.iter().map(|c| c.len()).sum();
    for c in &classes {
        for elm in c {
            assert!(elm.to_qs().is_perfect());
            eprintln!("{}", elm.to_qs().to_string_raw());
        }
    }
    eprintln!("The function found a total of {count} sequences without any equivalences");

    classes.iter()
        .map(|c| find_minimum(c))
        .map(|w| w.to_qs().to_string_raw())
        .fold("".to_string(), |s,t| s + &t  + &"\n")
}


pub fn find_aux(size : usize) -> Vec<QuadSeq>{
    let mut will = QuadSeq::new(size);

    let mut result = vec![];

    find_recursive(&mut will, size, 0, &mut result);

    result
}

fn find_recursive(will : &mut QuadSeq, size : usize, index : usize, sequences : &mut Vec<QuadSeq>){
    // This is the naive approach to finding QTS

    if index >= size{
        if will.to_qs().is_perfect() {
            sequences.push(will.clone());
        }
        return;
    }
    

    for value_to_test in QUADRUPLETS.iter(){
        will.set_sequence_value(value_to_test, index);
        find_recursive(will, size, index+1, sequences);
    }
}





pub fn reduce_to_equivalence(sequences : &Vec<QuadSeq>, seqtype : SequenceType) -> Vec<QuadSeq> {
    // This function reduces a set of QTS up to the Sequence equivalence defined in our paper
    
    let mut classes : Vec<HashSet<QuadSeq>> = vec![];

    for seq in sequences {
        let mut new_seq = true;
        for class in &classes {
            if class.contains(&seq) {
                new_seq = false;
                break;
            }
        }
        if new_seq {
            let new_class = generate_equivalence_class(&seq, seqtype.clone());
            classes.push(new_class);
        }
    }
    
    let count : usize = classes.iter().map(|c| c.len()).sum();
    eprintln!("The function found a total of {count} sequences without any equivalences");

    classes.iter()
        .map(|c| find_minimum(c))
        .collect()
}