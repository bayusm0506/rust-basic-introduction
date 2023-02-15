fn main() {
    let mut data_one = vec!["batman", "superman", "lobo"];

    data_one.pop();

    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity())
}
