fn main() {
    let var1= "Hello";
    println!("{}", var1);

    let var2 = "Hello \
        \"Jhon\" \
        and \
        \"Doe\"";
    println!("{}", var2);

    let var3 = "Hello Jhon,
Learn, Unlearn, 
Relearn,
Just keep moving
    ";
    println!("{}", var3);

    let var4 = r#"
        {
            "name": "Albert Einsten",
            "gender": "Male"
        }
    "#;
    println!("{}", var4);

    let var5 = "
        {
            \"name\": \"Jhon Piter\",
            \"gender\": \"Male\"
        }
    ";
    println!("{var5}");
}