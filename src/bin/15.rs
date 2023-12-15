advent_of_code::solution!(15);

fn hash(s: &str) -> u32 {
    s.chars()
        .filter(|&c| c != '\n')
        .map(|c| c as u8)
        .fold(0, |current_value, c| {
            ((current_value + c as u32) * 17) % 256
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').map(|step| hash(step)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split(',')
            .map(|step| step.replace('\n', ""))
            .map(|step| (step.clone(), hash(&step[0..2]) as usize))
            .fold(
                std::array::from_fn(|_| Vec::new()),
                |mut lenses: [Vec<(String, u32)>; 256], (step, box_i)| {
                    let lens = &mut lenses[box_i];

                    if let Some((label, focus)) = step.split_once('=').map(|(label, focus)| {
                        (
                            label.to_string(),
                            focus.parse().expect("focus to be valid number"),
                        )
                    }) {
                        if let Some(matching_i) = lens
                            .iter()
                            .enumerate()
                            .find(|(_, (lens_label, _))| lens_label == &label)
                            .map(|(slot_i, _)| slot_i)
                        {
                            lens[matching_i] = (label, focus);
                        } else {
                            lens.push((label, focus));
                        }
                    } else if let Some(label) = step.strip_suffix('-') {
                        if let Some(matching_i) = lens
                            .iter()
                            .enumerate()
                            .find(|(_, (lens_label, _))| lens_label == &label)
                            .map(|(slot_i, _)| slot_i)
                        {
                            lens.remove(matching_i);
                        }
                    } else {
                        unreachable!("invalid step");
                    }

                    lenses
                },
            )
            .into_iter()
            .enumerate()
            .flat_map(|(box_i, r#box)| {
                r#box
                    .into_iter()
                    .enumerate()
                    .map(move |(i, (_, focal_length))| {
                        (box_i as u32 + 1) * (i as u32 + 1) * focal_length
                    })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
