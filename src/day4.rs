pub fn day(input: std::string::String) {
    let input_split = input
        .split('-')
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<i32>>();

    let start_input = input_split[0];
    let end_input = input_split[1];

    let mut result_one = 0;

    for x in start_input..end_input {
        let x_as_string = x.to_string();

        let mut count = false;
        let mut prev_num = 0;

        for y in x_as_string.chars() {
            let digit = y.to_digit(10).unwrap_or_default();

            if digit < prev_num {
                count = false;
                break;
            }

            if digit == prev_num {
                count = true;
            }

            prev_num = digit;
        }

        if count {
            result_one += 1;
        }
    }

    println!("Day 4 Result1: {:?}", result_one);
    println!("Day 4 Result2: {:?}", input);
}
