pub fn day(input: std::string::String) {
    let input_vec = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let input_size = input_vec.len();
    assert!(input_size % layer_size == 0);

    let layer_count = input_size / layer_size;

    let mut layers: Vec<&[u32]> = vec![];
    for i in 0..layer_count {
        layers.push(&input_vec[layer_size * i..layer_size * (i + 1)])
    }

    let result_one = get_part_one(&layers);
    let result_two = 0;

    println!("Day 8 Result1: {:?}", result_one);
    println!("Day 8 Result2: {:?}", result_two);
}

fn get_part_one(layers: &Vec<&[u32]>) -> usize {
    let mut fewest_zero = usize::max_value();
    let mut fewest_zero_index = 0;

    for (i, layer) in layers.iter().enumerate() {
        let zeroes = layer.iter().filter(|x| **x == 0).count();

        fewest_zero = fewest_zero.min(zeroes);
        if fewest_zero == zeroes {
            fewest_zero_index = i;
        }
    }

    let layer = layers[fewest_zero_index];
    let one_count = layer.iter().filter(|x| **x == 1).count();
    let two_count = layer.iter().filter(|x| **x == 2).count();

    one_count * two_count
}
