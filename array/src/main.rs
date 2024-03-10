fn main() {
    let mut numbers = [23, 24, 26,27];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("element array ke 0 {data0}");

    let data1 = numbers[1];
    println!("element array ke 1 {data1}");

    numbers[1] = 6;
    numbers[3] = 12;
    println!("array {numbers:?}");

    // predefined array with size 10 and value 0
    let data_number: [i32; 10] = [0; 10];
    println!("{data_number:?}");

    let names = ["jason", "gray", "drake", "damian"];
    let length = names.len();
    println!("array size is {}", length);

    for name in names {
        println!("name is {name}")
    }

    for (i, name) in names.iter().enumerate() {
        println!("name index-{i}: {name}")
    }

    // nested array
    let data_array = [
        ["salad", "fried rice"],
        ["apple", "coconut"],
        ["spinach", "jalapeno"]
    ];

    for sub_arr in data_array{
        for el in sub_arr {
            println!("{el}, ");
        }
        println!();
    }
}
