fn main() {
    let numbers = [12, 16, 8, 3];
    println!("numbers   : {:?}, len: {}", numbers, numbers.len());

    // memotong array dari index 0 - 3
    let slice_a = &numbers[0..3];
    println!("slice_a : {:?} len : {:?}", slice_a, slice_a.len());

    // memotong array dari index 1 - 2
    let slice_b = &numbers[1..=2];
    println!("slice_b : {:?} len : {:?}", slice_b, slice_b.len());

    let mut numbers2 = [12, 16, 8 ,3];
    println!("===before===");
    println!("numbers2 : {:?}", numbers2);

    let slice_c = &mut numbers2[..=2];
    println!("===after===");
    println!("slice_c : {:?}", slice_c);

    let scores = [7, 8, 9];
    for score in scores {
        println!("{:?} ", score);
    }

    let mut scores2 = [7, 8, 9];
    println!("before scores2: {:?}", scores2);

    let slice_e = &mut scores2[..];

    for score in slice_e {
        *score += 1;
    }

    println!("after scores2: {:?}", scores2);
}
