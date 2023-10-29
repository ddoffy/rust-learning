
// method to pritn the get value
fn value(n:Option<&char>) {
    match n {
        Some(n) => println!("Element of vector {}", n),
        None => println!("None"),
    }
}


fn main() {
    let v = vec!['G', 'E', 'E', 'K', 'S'];

    // here index is the integer non negative value which is smaller than the size of the vector
    //let index: usize = 3;

    // getting value at given index value
    // let ch: char = v[index];

    //let ch: Option<&char> = v.get(index);

    //value(ch);

    // printing the size of vector
    //println!("Size of vector is {}", v.len());
    // println!("{ } \n", ch);



    print!("Vecto r elements: ");
    // loop to iterate elements in vector
    for i in v {
        // iterating through i on the vector 
        print!("{} ", i);
    }

    print!("\n",);
}
