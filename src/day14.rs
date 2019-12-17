pub fn day(input: String) {
    let parsed_input = parse(input);

    let result_one = parsed_input;
    let result_two = 0;

    println!("Day 10 Result1: {:?}", result_one);
    println!("Day 10 Result2: {:?}", result_two);
}

fn parse(input: String) -> Vec<Moon> {
    let moon_data_lines = input.lines().collect::<Vec<_>>();
    let mut result: Vec<Moon> = vec![];

    // <x=-7, y=17, z=-11>
    let blacklist = vec!['<', 'x', '=', 'y', 'z', '>'];
    for line in moon_data_lines {
        let mut extract = line.to_owned();
        extract.retain(|c| !blacklist.contains(&c));

        //-7, 17, -11
        let position_vec = extract
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if position_vec.len() != 3 {
            panic!("Unable to extract correctly for moon {:?}", line);
        }

        let position = Vec3::new(position_vec[0], position_vec[1], position_vec[2]);
        result.push(Moon::new(position));
    }

    result
}
