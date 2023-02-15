fn main() {
    let mut data_one = vec!["batman", "superman", "lobo"];
    let mut data_two = vec!["ninja-saga", "thor", "strange"];

    data_one.pop();
    data_one.push("constantine");
    data_one[2] = "red hood";
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    data_two.remove(1);
    data_two.push("trigon");
    data_two.push("darkseid");
    data_two[2] = "black hood";
    println!("data two: {:?}", data_two);
    println!("length two: {}, capacity two: {}", data_two.len(), data_two.capacity());

    let is_vector_empty = data_one.is_empty();
    println!("result: {:?}", is_vector_empty);

    data_one.clear();
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    data_two.clear();
    println!("data: {:?}", data_two);
    println!("length: {}, capacity: {}", data_two.len(), data_two.capacity());

    let mut result_one = vec![3, 5,1];

    let mut data_three = vec![9, 4, 2];
    result_one.append(&mut data_three);

    result_one.append(&mut vec![4, 7]);

    println!("data: {:?}", result_one);
    println!("length: {}, capacity: {}", result_one.len(), result_one.capacity());

    data_one.sort();
    data_two.sort();
    result_one.sort();
    println!("data_one: {:?}", data_one);
    println!("data_two: {:?}", data_two);
    println!("result_one: {:?}", result_one);

    // let mut vector_4 = vec![1, 2, 3];
    // let mut vector_5: Vec<i64>= vec![1, 2, 3];
    // let mut vector_6: Vec<&str> = vec![];
    // let mut vector_7: Vec<&str> = Vec::new();

    let vec_eight = vec![1, 2, 3];
    for e in vec_eight {
        print!("{e} ");
    }

    let vec_nine = vec![1, 2, 3];
    for i in 0..vec_nine.len() {
        print!("{} ", vec_nine[i]);
    }

    let vec_ten = vec![1, 2, 3];
    for e in vec_ten.iter() {
        print!("{e} ");
    }

    for i in 0..vec_ten.len() {
        print!("{} ", vec_ten[i]);
    }
}
