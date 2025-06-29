
use crate::sequences::williamson::{QuadSeq, QUADRUPLETS};




pub fn find(size : usize, condition : fn(&QuadSeq) -> bool) -> usize{
    let mut will = QuadSeq::new(size);

    return find_recursive(&mut will, size, 1, condition);
}

fn find_recursive(will : &mut QuadSeq, size : usize, index : usize, condition : fn(&QuadSeq) -> bool) -> usize{

    if index >= will.search_size(){
        if condition(will) {
            if !will.to_qs().is_perfect(){
                //println!("{}", will.to_string());
            }
            else{
                println!("{}", will.to_qs().to_string_raw());
                return 1;
            }
        }
        
        return 0;
    }

    let mut count = 0;
    for value_to_test in QUADRUPLETS.iter(){
        let mut will1 = will.clone();
        will1.set_sequence_value(value_to_test, index);
        count += find_recursive(&mut will1, size, index+1, condition);
    }

    count
}


