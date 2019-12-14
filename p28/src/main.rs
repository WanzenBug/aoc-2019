use std::str::FromStr;
use std::collections::HashMap;
use std::iter::FromIterator;

const INPUT: &str = include_str!("../INPUT");

type Error = Box<dyn std::error::Error + 'static>;

fn main() -> Result<(), Error> {
    let result = run(INPUT)?;
    println!("result = {}", result);
    Ok(())
}

fn run(input: &str) -> Result<usize, Error> {
    let specs: Result<Vec<_>, _> = input.lines()
        .filter(|s| !s.is_empty())
        .map(Reaction::from_str)
        .collect();
    let table: ReactionTable = specs?.into_iter().collect();

    let mut fuel = 1;
    while determine_required_ore(&table, fuel)? <= 1_000_000_000_000 {
        fuel *= 2;
    }
    let mut low = fuel / 2;
    let mut high = fuel;

    while low < high {
        let mid = (low + high) / 2;
        let ore = determine_required_ore(&table, mid)?;
        match ore.cmp(&1_000_000_000_000) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Equal => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
        }
    }

    while determine_required_ore(&table, low)? <= 1_000_000_000_000 {
        low += 1;
    }

    assert!(determine_required_ore(&table, low - 1)? <= 1_000_000_000_000);
    assert!(determine_required_ore(&table, low)? > 1_000_000_000_000);
    Ok(low - 1)
}

fn determine_required_ore(table: &ReactionTable, fuel: usize) -> Result<usize, Error> {
    let mut requirements = HashMap::new();
    requirements.insert("FUEL".to_string(), fuel);

    for reaction in table.clone() {
        let needed_amount = match requirements.remove(&reaction.result.name) {
            Some(v) => v,
            None => continue,
        };
        assert_ne!(needed_amount, 0);
        let reaction_multiplicator = ((needed_amount - 1) / reaction.result.amount) + 1;
        for requirement in reaction.requirements {
            let entry = requirements.entry(requirement.name)
                .or_insert(0);
            *entry += requirement.amount * reaction_multiplicator;
        }
    }

    assert_eq!(requirements.len(), 1);
    let ore_req = requirements.remove("ORE").ok_or_else(|| "Did not reduce to ORE!")?;
    Ok(ore_req)
}


#[derive(Clone)]
struct ReactionTable {
    inner: HashMap<String, Reaction>
}

impl FromIterator<Reaction> for ReactionTable {
    fn from_iter<T: IntoIterator<Item=Reaction>>(iter: T) -> Self {
        let mut inner: HashMap<String, Reaction> = HashMap::new();
        for spec in iter {
            let mut to_check = spec.requirements.clone();

            while let Some(v) = to_check.pop() {
                if v.name == spec.result.name {
                    panic!("Cycle detected");
                }

                if let Some(v) = inner.get(&v.name) {
                    to_check.extend(v.requirements.clone());
                }
            }

            let old = inner.insert(spec.result.name.clone(), spec);
            assert!(old.is_none());
        }

        Self {
            inner
        }
    }
}

impl Iterator for ReactionTable {
    type Item = Reaction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut candidate = None;
        for candidate_name in self.inner.keys() {
            let mut required = false;

            for reactions in self.inner.values() {
                if reactions.requirements.iter().find(|v| &v.name == candidate_name).is_some() {
                    required = true;
                    break;
                }
            }

            if !required {
                candidate = Some(candidate_name.to_string());
                break;
            }
        }

        if let Some(v) = candidate {
            self.inner.remove(&v)
        } else {
            assert_eq!(self.inner.len(), 0);
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ItemSpec {
    name: String,
    amount: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reaction {
    result: ItemSpec,
    requirements: Vec<ItemSpec>,
}

impl FromStr for Reaction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=>");
        let requirements_str = split.next().ok_or_else(|| format!("Error parsing line: {}", s))?;
        let result_str = split.next().ok_or_else(|| format!("Error parsing line: {}", s))?;

        let result = result_str.parse()?;
        let requirements: Result<Vec<_>, Error> = requirements_str.split(",").map(ItemSpec::from_str).collect();
        let requirements = requirements?;

        Ok(Reaction {
            result,
            requirements,
        })
    }
}

impl FromStr for ItemSpec {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let amount = parts.next().ok_or_else(|| format!("Error parsing item: {}", s))?.parse()?;
        let name = parts.next().ok_or_else(|| format!("Error parsing item: {}", s))?.parse()?;

        Ok(ItemSpec {
            name,
            amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parse() {
        let input = "7 A, 1 E => 1 FUEL";
        let spec: Reaction = input.parse().unwrap();

        assert_eq!(spec, Reaction {
            result: ItemSpec { name: "FUEL".to_string(), amount: 1 },
            requirements: vec![
                ItemSpec { name: "A".to_string(), amount: 7 },
                ItemSpec { name: "E".to_string(), amount: 1 },
            ],
        })
    }

    #[test]
    fn test_full() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";

        let result = run(input).unwrap();
        assert_eq!(result, 82892753);
    }
}
