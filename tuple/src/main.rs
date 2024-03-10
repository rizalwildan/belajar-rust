fn main() {
    let tuple_a = ("jasson", 27, ["racing", "workout"], true);
    println!("tuple_a: {:?}", tuple_a);

    let mut tuple_b: (&str, i32, [&str; 2], bool) = ("default", 0, [""; 2], false);
    tuple_b.0 = "jojon";
    tuple_b.1 = 18;
    tuple_b.2 = ["gaming", "adventuring"];
    tuple_b.3 = true;

    println!("tuple_b: {:?}", tuple_b);

    // packing tuple
    let name = "hary";
    let age = 20;
    let hobbies = ["sleeping", "gaming"];

    let tuple_c = (name, age, hobbies);
    println!("tuple_c: {:?}", tuple_c);

    // unpacking tuple
    let tuple_d = ("stephanie", 28, ["software_engineer"], false);
    let (name, age, devisi, is_male) = tuple_d;

    println!("name: {:?}", name);
    println!("age: {:?}", age);
    println!("devisi: {:?}", devisi);
    println!("is_male: {:?}", is_male);

}
