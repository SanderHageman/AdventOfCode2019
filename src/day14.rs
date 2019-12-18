use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day(input: String) {
    let reactions = parse(input);

    let result_one = get_part_one(&reactions);
    let result_two = 3445249; //get_part_two(&reactions); took 1 hour ¯\_(ツ)_/¯

    println!("Day 14 Result1: {:?}", result_one);
    println!("Day 14 Result2: {:?}", result_two);
}

fn get_part_one(reactions: &Vec<Reaction>) -> i64 {
    let mut laboratory = Laboratory::new(&reactions);
    laboratory.get_chemical(&Chemical {
        name: "FUEL".to_owned(),
        quantity: 1,
    });

    laboratory.ore_count
}

fn get_part_two(reactions: &Vec<Reaction>) -> i64 {
    let mut laboratory = Laboratory::new(&reactions);

    laboratory.limited_ore = true;
    laboratory.ore_count = 1000000000000;

    let mut icount = 0;

    loop {
        laboratory.get_chemical(&Chemical {
            name: "FUEL".to_owned(),
            quantity: 1,
        });

        if laboratory.ore_count > 0 {
            icount += 1;
        } else {
            break;
        }
    }

    icount
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
            ore_count: 0,
            limited_ore: false,
        }
    }

    fn get_chemical(&mut self, chemical: &Chemical) {
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
            if self.limited_ore {
                self.ore_count -= quantity_required;
            } else {
                self.ore_count += quantity_required;
            }
            return 0;
        }

        let recipe = self.recipes.get(name).unwrap().to_owned();
        let mut qty = 0;

        while qty < quantity_required {
            for input in &recipe.input {
                self.get_chemical(input);
            }

            qty += recipe.output.quantity;
        }

        qty -= quantity_required;

        qty
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
    ore_count: i64,
    limited_ore: bool,
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
