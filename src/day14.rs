use math::round;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day(input: String) {
    let reactions = parse(input);

    let result_one = get_part_one(&reactions);
    let result_two = get_part_two(&reactions, result_one);

    println!("Day 14 Result1: {:?}", result_one);
    println!("Day 14 Result2: {:?}", result_two);
}

fn get_part_one(reactions: &Vec<Reaction>) -> i64 {
    let mut laboratory = Laboratory::new(&reactions);
    laboratory.consume_chemical(&Chemical {
        name: "FUEL".to_owned(),
        quantity: 1,
    });

    laboratory.consumed_ore_count
}

fn get_part_two(reactions: &Vec<Reaction>, part_one_result: i64) -> i64 {
    const TOTAL_ORE_COUNT: i64 = 1000000000000;
    const HIGH_FACTOR: f64 = 1.5;

    let mut low = round::ceil(TOTAL_ORE_COUNT as f64 / part_one_result as f64, 0) as i64;
    let mut high = round::ceil(low as f64 * HIGH_FACTOR as f64, 0) as i64;
    let mut mid = (low + high) / 2;

    loop {
        let mut laboratory = Laboratory::new(&reactions);

        laboratory.consume_chemical(&Chemical {
            name: "FUEL".to_owned(),
            quantity: mid,
        });

        let consumed_ore = laboratory.consumed_ore_count;
        let diff = TOTAL_ORE_COUNT - consumed_ore;

        if diff.abs() < part_one_result {
            return mid;
        }

        mid = if diff > 0 {
            low = mid;
            (mid + high) / 2
        } else {
            high = mid;
            (low + mid) / 2
        };
    }
}

impl Laboratory {
    fn new(reactions: &Vec<Reaction>) -> Self {
        let mut recipes = HashMap::<String, Reaction>::new();

        for reaction in reactions {
            if recipes
                .insert(reaction.output.name.to_owned(), reaction.to_owned())
                .is_some()
            {
                panic!(
                    "multiple recipes for the same chemical {:?}",
                    reaction.output
                );
            }
        }

        Laboratory {
            inventory: HashSet::new(),
            recipes: recipes,
            consumed_ore_count: 0,
        }
    }

    fn consume_chemical(&mut self, chemical: &Chemical) {
        let mut from_inventory = match self.inventory.take(&chemical) {
            Some(val) => val,
            None => self.create_chemical(&chemical.name),
        };

        let qty_needed = chemical.quantity;

        from_inventory.quantity -= qty_needed;

        if from_inventory.quantity < 0 {
            from_inventory.quantity =
                self.run_recipe(&from_inventory.name, from_inventory.quantity.abs());
        }

        if !self.inventory.insert(from_inventory) {
            panic!("Already in our inventory! {:?}", chemical.name)
        }
    }

    fn run_recipe(&mut self, name: &str, quantity_required: i64) -> i64 {
        if name == "ORE" {
            self.consumed_ore_count += quantity_required;
            return 0;
        }

        let recipe = self.recipes.get(name).unwrap().to_owned();

        let per_recipe_output = recipe.output.quantity;
        let cycle_count =
            round::ceil(quantity_required as f64 / per_recipe_output as f64, 0) as i64;

        for input in &recipe.input {
            let mut request = input.to_owned();
            request.quantity *= cycle_count;
            self.consume_chemical(&request);
        }

        (recipe.output.quantity * cycle_count) - quantity_required
    }

    fn create_chemical(&mut self, name: &str) -> Chemical {
        Chemical {
            name: name.to_owned(),
            quantity: 0,
        }
    }
}

#[derive(Debug)]
struct Laboratory {
    inventory: HashSet<Chemical>,
    recipes: HashMap<String, Reaction>,
    consumed_ore_count: i64,
}

#[derive(Debug, Clone, Eq)]
struct Chemical {
    name: String,
    quantity: i64,
}

#[derive(Debug, Clone)]
struct Reaction {
    input: Vec<Chemical>,
    output: Chemical,
}

impl FromStr for Chemical {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.split_whitespace().collect::<Vec<_>>();
        assert_eq!(input.len(), 2);

        let chem = Chemical {
            name: input[1].to_owned(),
            quantity: input[0].parse::<i64>().unwrap(),
        };

        Ok(chem)
    }
}

impl Hash for Chemical {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Chemical {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn parse(input: String) -> Vec<Reaction> {
    let mut result: Vec<Reaction> = vec![];

    let input_lines = input.lines().map(|x| x.to_owned()).collect::<Vec<_>>();

    for line in input_lines {
        let out = line.split("=>").map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!(out.len(), 2);

        let inputs_strings = out[0].split(',').map(|s| s.trim()).collect::<Vec<_>>();

        let inputs = inputs_strings
            .iter()
            .map(|s| s.parse::<Chemical>().unwrap())
            .collect::<Vec<_>>();

        let output = out[1].parse::<Chemical>().unwrap();

        result.push(Reaction {
            input: inputs,
            output: output,
        });
    }

    result
}
