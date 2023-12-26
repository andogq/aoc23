use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

advent_of_code::solution!(19);

#[derive(Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

enum Condition {
    Branch {
        category: Category,
        comparision: Ordering,
        threshold: u32,
        workflow: Workflow,
    },
    Default(Workflow),
}

impl TryFrom<char> for Category {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Workflow {
    Accept,
    Reject,
    Custom(String),
}

fn parse_workflows(input: &str) -> HashMap<String, Vec<Condition>> {
    input
        .lines()
        .map(|workflow| {
            let (name, conditions) = workflow.split_once("{").unwrap();

            (
                name.to_string(),
                conditions[..conditions.len() - 1]
                    .split(",")
                    .map(|condition| {
                        if let Some((condition, next_workflow)) = condition.split_once(":") {
                            let mut condition = condition.chars();

                            Condition::Branch {
                                category: Category::try_from(
                                    condition.next().expect("category letter in condition"),
                                )
                                .expect("valid category letter"),
                                comparision: match condition.next().expect("comparision characer") {
                                    '<' => Ordering::Less,
                                    '>' => Ordering::Greater,
                                    _ => panic!("invalid comparision character"),
                                },
                                threshold: condition
                                    .rev()
                                    .enumerate()
                                    .map(|(i, d)| {
                                        10u32.pow(i as u32)
                                            * d.to_digit(10).expect("valid basae 10 digit")
                                    })
                                    .sum(),
                                workflow: Workflow::from(next_workflow),
                            }
                        } else {
                            Condition::Default(Workflow::from(condition))
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

impl<S: AsRef<str>> From<S> for Workflow {
    fn from(str: S) -> Self {
        match str.as_ref() {
            "A" => Self::Accept,
            "R" => Self::Reject,
            workflow => Self::Custom(workflow.to_string()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows);

    Some(
        parts
            .lines()
            .map(|part| {
                let categories = part[1..part.len() - 1]
                    .split(",")
                    .map(|category| {
                        let mut category = category.chars();

                        (
                            Category::try_from(category.next().expect("category letter present"))
                                .expect("valid category letter"),
                            category
                                .skip(1)
                                .map(|d| d.to_digit(10).expect("valid digit for value"))
                                .fold(0, |n, d| (n * 10) + d),
                        )
                    })
                    .fold([None; 4], |mut categories, (category, value)| {
                        categories[category as usize] = Some(value);

                        categories
                    });

                std::array::from_fn(|i| categories[i].unwrap()) as [u32; 4]
            })
            .filter(|part| {
                let mut next_workflow = &Workflow::Custom("in".to_string());

                while let Workflow::Custom(workflow) = next_workflow {
                    let workflow = workflows.get(workflow).expect("requested workflow");

                    next_workflow = workflow
                        .iter()
                        .find_map(|condition| match condition {
                            Condition::Default(workflow) => Some(workflow),
                            Condition::Branch {
                                category,
                                comparision,
                                threshold,
                                workflow,
                            } => {
                                if part[*category as usize].cmp(threshold) == *comparision {
                                    Some(workflow)
                                } else {
                                    None
                                }
                            }
                        })
                        .expect("valid branch in workflow");
                }

                match next_workflow {
                    Workflow::Accept => true,
                    Workflow::Reject => false,
                    Workflow::Custom(_) => unreachable!("custom workflows should be processed"),
                }
            })
            .map(|part| part.into_iter().sum::<u32>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    let workflows = parse_workflows(input.split_once("\n\n").unwrap().0);

    struct Cursor {
        workflow: String,
        bounds: [(u32, u32); 4],
    }

    let mut cursors = VecDeque::from_iter([Cursor {
        workflow: "in".to_string(),
        bounds: [(1, 4000); 4],
    }]);

    let mut final_count = 0;

    while let Some(mut cursor) = cursors.pop_front() {
        let workflow = workflows
            .get(&cursor.workflow)
            .expect("workflow to be present");

        workflow.into_iter().for_each(|condition| {
            let (workflow, bounds) = match condition {
                Condition::Branch {
                    category,
                    comparision,
                    threshold,
                    workflow,
                } => {
                    (workflow, {
                        let mut new_bounds = cursor.bounds.clone();

                        let i = *category as usize;

                        // Restrict bounds
                        match comparision {
                            Ordering::Less => {
                                // Restrict max
                                new_bounds[i].1 = new_bounds[i].1.min(*threshold - 1);

                                cursor.bounds[i].0 = new_bounds[i].1 + 1;
                            }
                            Ordering::Greater => {
                                // Restrict min
                                new_bounds[i].0 = new_bounds[i].0.max(*threshold + 1);

                                cursor.bounds[i].1 = new_bounds[i].0 - 1;
                            }
                            Ordering::Equal => {
                                unreachable!("equals comparision shouldn't be present")
                            }
                        }

                        new_bounds
                    })
                }
                Condition::Default(workflow) => (workflow, cursor.bounds),
            };

            match workflow {
                Workflow::Accept => {
                    final_count += bounds
                        .into_iter()
                        .map(|(min, max)| (max - min + 1) as u128)
                        .product::<u128>();
                }
                Workflow::Custom(workflow) => cursors.push_back(Cursor {
                    workflow: workflow.to_string(),
                    bounds,
                }),
                Workflow::Reject => (),
            }
        });
    }

    Some(final_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
