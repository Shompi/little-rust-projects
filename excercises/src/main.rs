fn main() {
    println!("Hello, world!");
    // Test 1
    // let vect1: Vec<i32> = vec![1, 2, 3];
    // let vect2: Vec<i32> = vec![5, 2, 1, 4];

    // Test 2
    // let vect1 = vec![1, 2, 3];
    // let vect2 = vec![5, 2, 1, 4, 5];

    // Test 3
    let vect1 = vec![3, 3, 3, 2, 5];
    let vect2 = vec![2, 1, 5, 7];
    let vect3 = vec![3, 4, 6, 6];
    let vect4 = vec![1, 2, 3];

    println!(
        "Vector 1: {:?}\nVector 2: {:?}\nVector 3: {:?}\nVector 4: {:?}",
        vect1, vect2, vect3, vect4
    );

    let mut result = filter(&[vect1, vect2, vect3, vect4]);

    result.sort();

    println!("Final Vector: {:?}", result);
}

fn filter(args: &[Vec<i32>]) -> Vec<i32> {
    let mut vecta: Vec<i32> = vec![];

    for arr in args {
        let mut aux = arr.clone();
        aux.sort();
        aux.dedup();

        for element in aux {
            if !vecta.contains(&element) {
                vecta.push(element);
            } else {
                // Index of the element to remove from vecta
                // let index = vecta.iter().position(|to_remove| *to_remove == *element);
                vecta.retain(|&to_remove| to_remove != element);
            }
        }
    }
    vecta
}
