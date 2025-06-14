use std::collections::HashSet;


use itertools::iproduct;

use super::williamson::{Williamson, SequenceTag};



// * Computes a table of coprimes at compile time

pub fn coprime(mut a : usize, mut b : usize) -> bool {
    // tests if two numbers are coprime or not
    if b < a {
        (a,b) = (b,a);
    }
    while b != 0 {
        (a,b) = (b, a % b)
    }
    a == 1
}

const N : usize = 101;

lazy_static!(
    // calculates at compile-time the table of coprimes up to N
    pub static ref COPRIMES : Vec<Vec<usize>> = {
        let mut list = vec![];

        for i in 0..N {

            let mut coprimes = vec![];

            for j in 1..=i {
                if coprime(i,j) {
                    coprimes.push(j);
                }
            }
            list.push(coprimes);
        }

        list
    };
);




// * defining the lexical order on williamson sequences

pub fn seq_less_than(seq1 : &Vec<i8>, seq2 : &Vec<i8>) -> bool {

    let mut index = 0;
    while index < seq1.len() {
        if seq1[index] < seq2[index] {
            return true;
        }
        else if seq1[index] > seq2[index] {
            return false;
        }
        else {
            index += 1;
        }
    }

    false
}


enum Comp {
    LT, EQUAL, GT
}

fn will_less_than_aux(seq1 : &Vec<i8>, seq2 : &Vec<i8>) -> Comp {

    let mut index = 0;
    while index < seq1.len() {
        if seq1[index] < seq2[index] {
            return Comp::LT;
        }
        else if seq1[index] > seq2[index] {
            return Comp::GT;
        }
        else {
            index += 1;
        }
    }

    Comp::EQUAL
}

pub fn will_less_than(will1 : &Williamson, will2 : &Williamson) -> bool {

    let (a1,b1,c1,d1) = will1.sequences();
    let (a2,b2,c2,d2) = will2.sequences();
    match will_less_than_aux(&a1, &a2) {
        Comp::LT => {true}
        Comp::GT => {false}
        Comp::EQUAL => {
            match will_less_than_aux(&b1, &b2) {
                Comp::LT => {true}
                Comp::GT => {false}
                Comp::EQUAL => {
                    match will_less_than_aux(&c1, &c2) {
                        Comp::LT => {true}
                        Comp::GT => {false}
                        Comp::EQUAL => {
                            seq_less_than(&d1, &d2)
                        }
    }}}}}
}



// * Functions to treat the equivalences

pub fn generate_canonical_representative(seq : &Williamson) -> Williamson{
    let set = generate_equivalence_class(seq);
    let mut mini = seq.clone();
    for elm in set {
        if will_less_than(&elm, &mini) {
            mini = elm;
        }
    }
    mini
}




pub fn generate_equivalence_class(seq : &Williamson) -> HashSet<Williamson> {
    // This function generates the representative of the equivalence class that seq belongs to
    
    let mut class = HashSet::new();
    class.insert(seq.clone());

    loop {
        let mut new = HashSet::new();

        for seq in &class {
            for equivalence in [equivalent_negate, equivalent_uniform_shift, equivalent_reorder, equivalent_alternated_negation, equivalent_automorphism, equivalent_reverse] {
                for equ in equivalence(&seq){
                    if !class.contains(&equ) {
                        new.insert(equ);
                    }
                }
            }

        }


        if new.len() == 0 {break;}
        else {
            for seq in new {
                class.insert(seq);
            }
        }
    }

    class
}


pub fn generate_equivalent_qts(wills : &Vec<Williamson>) -> Vec<Williamson> {

    let mut result = HashSet::new();

    for w in wills {
        if result.contains(w) {
            continue;
        }

        let class = generate_equivalence_class(w);
        for elm in class {
            result.insert(elm);
        }
    }

    result.into_iter().collect()
}



fn swap(will : &mut Williamson, seqtag1 : SequenceTag, seqtag2 : SequenceTag) {
    
    let (a,b,c,d) = will.sequences();

    let (seq1, seq2) = match (&seqtag1, &seqtag2) {
        (SequenceTag::X, SequenceTag::Y) => {(a, b)},
        (SequenceTag::X, SequenceTag::Z) => {(a, c)},
        (SequenceTag::X, SequenceTag::W) => {(a, d)},
        (SequenceTag::Y, SequenceTag::Z) => {(b, c)},
        (SequenceTag::Y, SequenceTag::W) => {(b, d)},
        (SequenceTag::Z, SequenceTag::W) => {(c, d)},
        _ => {panic!("Incorrect tags entered !")}
    };

    will.set_sequence(&seq2, &seqtag1);
    will.set_sequence(&seq1, &seqtag2);
}

pub fn equivalent_reorder(seq : &Williamson) -> Vec<Williamson> {
    // computes all equivalent sequences by reorder

    let mut res = vec![seq.clone()];

    let couples = [(SequenceTag::X, SequenceTag::Y), (SequenceTag::X, SequenceTag::Z), (SequenceTag::X, SequenceTag::W), (SequenceTag::Y, SequenceTag::Z), (SequenceTag::Y, SequenceTag::W), (SequenceTag::Z, SequenceTag::W)];

    for (couple1, couple2) in iproduct!(couples.clone(), couples) {
        let mut new_seq = seq.clone();
        
        let (seq11, seq12) = couple1;
        let (seq21, seq22) = couple2;
        if !(seq11 == seq21 && seq12 == seq22){
            swap(&mut new_seq, seq11, seq12);
            swap(&mut new_seq, seq21, seq22);
        }

        res.push(new_seq);
    
    }

    res
}




pub fn equivalent_uniform_shift(seq : &Williamson) -> Vec<Williamson> {
    // computes all equivalent sequences by shift

    let mut res = vec![seq.clone()];

    for offset in 1..seq.size() {
        let mut s = Williamson::new(seq.size());
        for index in 0..seq.size() {
            s.set_sequence_value(&seq.values((index + offset) % seq.size()), index)
        }
        res.push(s);
    }

    res
}

pub fn equivalent_reverse(seq : &Williamson) -> Vec<Williamson> {
    // computes all equivalent sequences by shift

    let mut res = vec![seq.clone()];

    let mut s = Williamson::new(seq.size());
    for index in 0..seq.size() {
        s.set_sequence_value(&seq.values(seq.size() - 1 - index), index)
    }
    res.push(s);

    res
}



pub fn negated(seq : &Vec<i8>) -> Vec<i8> {
    let mut s = vec![];
    for i in 0..seq.len() {
        s.push(-seq[i]);
    }
    s
}

pub fn alt_negated(seq : &Vec<i8>, frequency : usize) -> Vec<i8> {
    let mut s = vec![];
    let mut count = 0;
    for i in 0..seq.len() {
        if count % frequency == frequency - 1 {
            s.push(-seq[i]);
        }
        else {
            s.push(seq[i]);
        }
        count += 1;
    }
    s
}



pub fn equivalent_negate(seq : &Williamson) -> Vec<Williamson> {
    // computes all equivalent sequences by negation

    let (a,b,c,d) = seq.sequences();
    let (nega_a,nega_b,nega_c,nega_d) = (negated(&a), negated(&b), negated(&c), negated(&d));

    let mut res = vec![];

    for tag_couple in [(SequenceTag::X, SequenceTag::Y), (SequenceTag::X, SequenceTag::Z), (SequenceTag::X, SequenceTag::W), (SequenceTag::Y, SequenceTag::Z), (SequenceTag::Y, SequenceTag::W), (SequenceTag::Z, SequenceTag::W)] {
        // this loops through all the couples of a,b,c,d (ordered couples)
        let quad = match tag_couple {
            (SequenceTag::X, SequenceTag::Y) => {(&nega_a, &nega_b, &c, &d)},
            (SequenceTag::X, SequenceTag::Z) => {(&nega_a, &b, &nega_c, &d)},
            (SequenceTag::X, SequenceTag::W) => {(&nega_a, &b, &c, &nega_d)},
            (SequenceTag::Y, SequenceTag::Z) => {(&a, &nega_b, &nega_c, &d)},
            (SequenceTag::Y, SequenceTag::W) => {(&a, &nega_b, &c, &nega_d)},
            (SequenceTag::Z, SequenceTag::W) => {(&a, &b, &nega_c, &nega_d)},
            _ => {panic!("Incorrect tags entered !")}
        };
        let mut s = Williamson::new(seq.size());
        s.set_all_values(quad);

        res.push(s);
    }

    res
}


pub fn equivalent_alternated_negation(seq : &Williamson) -> Vec<Williamson> {

    if seq.size() % 2 == 1 {
        return vec![seq.clone()];
    }

    let frequency = 2;

    let (a,b,c,d) = seq.sequences();

    let mut res = vec![];

    let quads = (&alt_negated(&a, frequency), &alt_negated(&b, frequency), &alt_negated(&c, frequency), &alt_negated(&d, frequency));

    let mut s = Williamson::new(seq.size());
    s.set_all_values(quads);

    res.push(seq.clone());
    res.push(s);

    res
}




fn permute(seq : &Vec<i8>, coprime : usize) -> Vec<i8> {
    // permutes all elements of seq using
    // the automorphism of the cyclic group defined by a coprime

    let n = seq.len();

    let mut result = vec![];

    for index in 0..n {
        result.push(seq[index * coprime % n]);
    }

    result
}





pub fn equivalent_automorphism(seq : &Williamson) -> Vec<Williamson> {
    // computes all equivalent sequences by permutation

    let mut result = vec![];
    let size = seq.size();

    let mut identity = vec![];
    for i in 0..size {
        identity.push(i);
    }

    for k in COPRIMES[size].iter() {

        let mut will = Williamson::new(size);

        let (a,b,c,d) = seq.sequences();

        let quad = (&permute(&a, *k), &permute(&b, *k), &permute(&c, *k), &permute(&d, *k));

        will.set_all_values(quad);

        result.push(will);

    }


    result
}