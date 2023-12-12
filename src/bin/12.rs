use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(Clone, Copy, Debug)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::Broken),
            '.' => Ok(Self::Working),
            '?' => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

fn solve(input: &str, repeat: usize) -> u64 {
    input
        .lines()
        .map(|line| line.split_once(' ').expect("whitespace to split"))
        .map(|(springs, groups)| {
            (
                {
                    let springs = springs
                        .chars()
                        .map(|c| Spring::try_from(c).expect("invalid spring character"))
                        .collect::<Vec<_>>();

                    (0..repeat).fold(Vec::new(), |mut v, i| {
                        if i != 0 {
                            v.push(Spring::Unknown);
                        }

                        v.append(&mut springs.clone());

                        v
                    })
                },
                {
                    let groups = groups
                        .split(',')
                        .map(|n| n.parse::<u64>().expect("valid number"))
                        .collect::<Vec<_>>();

                    (0..repeat).fold(Vec::new(), |mut v, _| {
                        v.append(&mut groups.clone());

                        v
                    })
                },
            )
        })
        .map(|(springs, groups)| {
            /// State for the BFS
            #[derive(Clone, Debug, Hash, Eq, PartialEq)]
            struct State {
                /// Current spring that the search is up to
                springs_i: usize,

                /// Current group that is being attempted to fit
                groups_i: usize,

                /// How many springs are left to be fit in this group
                group_remaining: u64,
            }

            struct Dfs {
                cache: HashMap<State, u64>,
                springs: Vec<Spring>,
                groups: Vec<u64>,
            }

            impl Dfs {
                fn solve(&mut self, state: State) -> u64 {
                    if let Some(&valid) = self.cache.get(&state) {
                        return valid;
                    }

                    let mut valid = 0;

                    if state.springs_i == self.springs.len()
                        && state.groups_i >= self.groups.len() - 1
                        && state.group_remaining == 0
                    {
                        valid = 1;
                    } else if state.springs_i < self.springs.len()
                        && state.groups_i <= self.groups.len()
                    {
                        let current_spring = self.springs[state.springs_i].clone();

                        if matches!(current_spring, Spring::Working | Spring::Unknown) {
                            if state.group_remaining == 0 {
                                if state.groups_i == self.groups.len() {
                                    // No more groups to advance to
                                    valid += self.solve(State {
                                        springs_i: state.springs_i + 1,
                                        groups_i: state.groups_i,
                                        group_remaining: 0,
                                    });
                                } else {
                                    // This group is complete, safe to advance
                                    valid += self.solve(State {
                                        springs_i: state.springs_i + 1,
                                        groups_i: state.groups_i + 1,
                                        group_remaining: self
                                            .groups
                                            .get(state.groups_i + 1)
                                            .cloned()
                                            .unwrap_or_default(),
                                    });
                                }
                            } else if state.group_remaining == self.groups[state.groups_i] {
                                // Group hasn't been started yet, safe to advance
                                valid += self.solve(State {
                                    springs_i: state.springs_i + 1,
                                    groups_i: state.groups_i,
                                    group_remaining: state.group_remaining,
                                });
                            } else {
                                // This group wasn't completed before it could advance, don't
                                // continue
                            }
                        }

                        if matches!(current_spring, Spring::Broken | Spring::Unknown) {
                            if state.group_remaining > 0 {
                                // Attempt to fit
                                valid += self.solve(State {
                                    springs_i: state.springs_i + 1,
                                    groups_i: state.groups_i,
                                    group_remaining: state.group_remaining - 1,
                                });
                            } else {
                                // Can't fit spring in here, so invalid combination
                            }
                        }
                    }

                    self.cache.insert(state, valid);

                    valid
                }
            }

            let initial_state = State {
                springs_i: 0,
                groups_i: 0,
                group_remaining: groups[0],
            };

            Dfs {
                cache: HashMap::new(),
                springs,
                groups,
            }
            .solve(initial_state)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
