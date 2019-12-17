use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day(input: String) {
    let reactions = parse(input);
    let mut chemicals = HashSet::<Chemical>::new();

    for reaction in &reactions {
        for inp in &reaction.input {
            chemicals.insert(inp.to_owned());
        }

        chemicals.insert(reaction.output.to_owned());
    }

    for chem in chemicals {
        println!("{:?} => {:?}", chem.name, chem.get_required_ore(&reactions));
    }

    let result_one = 0;
    let result_two = 0;

    println!("Day 14 Result1: {:?}", result_one);
    println!("Day 14 Result2: {:?}", result_two);
}

impl Chemical {
    fn get_required_ore(&self, recipes: &Vec<Reaction>) -> u64 {
        let mut result = 0;

        for recipe in recipes {
            if recipe.output.name != self.name {
                continue;
            }

            for chem in &recipe.input {
                if chem.name == "ORE" {
                    result += chem.quantity / recipe.output.quantity;
                }

                result += chem.get_required_ore(&recipes) * chem.quantity;
            }
        }

        result
    }
}

#[derive(Debug, Clone, Eq)]
struct Chemical {
    name: String,
    quantity: u64,
}

#[derive(Debug)]
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
            quantity: input[0].parse::<u64>().unwrap(),
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
