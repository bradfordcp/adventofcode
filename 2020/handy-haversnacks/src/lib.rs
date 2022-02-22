use std::{collections::{HashMap, HashSet}, fs, io};

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

lazy_static! {
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    static ref RULE_REGEX: Regex = Regex::new(r"(.*) contain (.+)\.").unwrap();
    static ref BAG_REGEX: Regex = Regex::new(r"(([0-9]+) )?([a-z ]+) bags?").unwrap();
}

type Bags = HashMap<String, Bag>;

#[derive(Debug, Default)]
struct Bag {
    color: String,
    contains: Vec<(usize, String)>,
    contained_by: HashSet<String>
}

#[allow(dead_code)]
fn parse_rules(path: &str) -> Result<Bags, io::Error> {
    let lines = fs::read_to_string(path)?;
    let bags = extract_bags(lines.as_str());

    Ok(bags)
}

fn extract_bags(lines: &str) -> Bags {
    let bags: Vec<Bag> = lines.lines()
        .map(|line| {
            let captures = RULE_REGEX.captures(line);
            let container = captures.as_ref().unwrap().get(1).unwrap().as_str();
            let contained = captures.as_ref().unwrap().get(2).unwrap().as_str();

            (container, contained)
        })
        .map(|pair| {
            let contains: Vec<(usize, String)> = pair.1.split(", ")
                .map(|bags| {
                    match bags {
                        "no other bags" => {
                            None
                        },
                        bags => {
                            BAG_REGEX.captures(bags)
                                .map(|capture| {
                                    let color = capture.get(3).unwrap().as_str().to_owned();
                                    let qty = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                                    
                                    Some((qty, color))
                                }).unwrap()
                        }
                    }
                })
                .fold(Vec::new(), |mut acc, ele| {
                    if let Some(pair) = ele {
                        acc.push(pair);
                    }
                    
                    acc
                });
            
            let bag = BAG_REGEX.captures(pair.0)
                .map(|capture| {
                    let color = capture.get(3).unwrap().as_str().to_owned();
                    Bag{
                        color,
                        contains,
                        ..Default::default()
                    }
                }).unwrap();
            
            
            bag
        })
        .collect();
    
    let mut graph = Bags::new();
    
    // Populate graph with entire dataset
    bags.into_iter().for_each(|bag| {
        println!("{:?}", bag);
        graph.insert(bag.color.clone(), bag);
    });

    // Iterate over all graph entries and update contained_by fields
    graph.values().clone()
        .for_each(|bag| {
            let contains = bag.contains.clone();
            
            contains.into_iter()
                .map(|pair| pair.1)
                .for_each(|color| {
                    let contained_bag = graph.get_mut(&color);

                });
        });

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules_sample() {
        let bags = parse_rules("test/sample");

        assert!(bags.is_ok());
        assert_eq!(bags.unwrap().len(), 9);
    }

    #[test]
    fn test_parse_rules_input() {
        let bags = parse_rules("test/input");

        assert!(bags.is_ok());
        assert_eq!(bags.unwrap().len(), 594);
    }
}
