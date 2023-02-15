fn main() {
    let mut data_one = vec!["batman", "superman", "lobo"];
    let mut data_two = vec!["ninja-saga", "thor", "strange"];

    data_one.pop();
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    data_two.remove(1);
    println!("data two: {:?}", data_two);
    println!("length two: {}, capacity two: {}", data_two.len(), data_two.capacity());
}
