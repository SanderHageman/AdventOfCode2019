use std::fs;

pub fn day2() {
    let input = fs::read_to_string("input/day2.txt").expect("Something went wrong!");
    let input_vec = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<_>>();

    let mut result_vec = input_vec.to_vec();

    let mut i = 0;
    while i < input_vec.len() {
        let opcode = result_vec[i];
        if opcode == 99 {
            break;
        };

        let parameter_one_index = result_vec[i + 1] as usize;
        let parameter_two_index = result_vec[i + 2] as usize;
        let parameter_result_index = result_vec[i + 3] as usize;

        let parameter_one = result_vec[parameter_one_index];
        let parameter_two = result_vec[parameter_two_index];

        result_vec[parameter_result_index] = match opcode {
            1 => parameter_one + parameter_two,
            2 => parameter_one * parameter_two,
            _ => panic!("uncovered opcode found {:?} at {:?}", opcode, i),
        };

        i += 4;
    }

    println!("Result1: {:?}", result_vec[0]);
    // println!("Result2: {:?}", p2_result);
}
