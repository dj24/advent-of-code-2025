use std::collections::HashSet;
use std::{fs, thread};

struct IndicatorLights(Vec<bool>);

impl IndicatorLights {
    fn apply_schematic(&mut self, schematic: &ButtonWiringSchematic) {
        for &index in &schematic.0 {
            if let Some(light) = self.0.get_mut(index as usize) {
                *light = !*light;
            }
        }
    }
}

struct ButtonWiringSchematic(Vec<u32>);

struct JoltageRequirements(Vec<u32>);

struct Output {
    indicator_lights: IndicatorLights,
    button_wiring_schematics: Vec<ButtonWiringSchematic>,
    joltage_requirements: JoltageRequirements,
}

fn parse_indicator_segment(segment: &str) -> IndicatorLights {
    let lights = segment
        .replace("[", "")
        .replace("]", "")
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => false,
        })
        .collect::<Vec<bool>>();
    IndicatorLights(lights)
}

fn parse_button_wiring_segment(segment: &str) -> ButtonWiringSchematic {
    let wiring = segment
        .replace("(", "")
        .replace(")", "")
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    ButtonWiringSchematic(wiring)
}

fn parse_joltage_segment(segment: &str) -> JoltageRequirements {
    let requirements = segment
        .replace("{", "")
        .replace("}", "")
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    JoltageRequirements(requirements)
}

fn extract_output_from_line(line: &str) -> Output {
    let mut parts = line.split(" ").collect::<Vec<&str>>();
    let first = parts.remove(0);
    let last = parts.pop().unwrap();

    let indicator_lights = parse_indicator_segment(first);
    let button_wiring_schematics = parts.into_iter().map(parse_button_wiring_segment).collect();
    let joltage_requirements = parse_joltage_segment(last);

    Output {
        indicator_lights,
        button_wiring_schematics,
        joltage_requirements,
    }
}

fn main() {
    match fs::read_to_string("./day-10/assets/input.txt") {
        Ok(contents) => {}
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indicator_segment() {
        let segment = "[#..#.]";
        let indicator_lights = parse_indicator_segment(segment);
        assert_eq!(indicator_lights.0, vec![true, false, false, true, false]);
    }

    #[test]
    fn test_parse_button_wiring_segment() {
        let segment = "(1,2,3,4,5)";
        let button_wiring = parse_button_wiring_segment(segment);
        assert_eq!(button_wiring.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_joltage_segment() {
        let segment = "{10,20,30,40}";
        let joltage_requirements = parse_joltage_segment(segment);
        assert_eq!(joltage_requirements.0, vec![10, 20, 30, 40]);
    }

    #[test]
    fn test_extract_output_from_line() {
        let line = "[#..#.] (1,2,3) (4,5,6) {10,20,30}";
        let output = extract_output_from_line(line);
        assert_eq!(output.indicator_lights.0, vec![true, false, false, true, false]);
        assert_eq!(output.button_wiring_schematics.len(), 2);
        assert_eq!(output.button_wiring_schematics[0].0, vec![1, 2, 3]);
        assert_eq!(output.button_wiring_schematics[1].0, vec![4, 5, 6]);
        assert_eq!(output.joltage_requirements.0, vec![10, 20, 30]);
    }

    #[test]
    fn test_apply_schematic() {
        let mut indicator_lights = IndicatorLights(vec![true, false, false, true, false]);
        let schematic = ButtonWiringSchematic(vec![1, 3]);
        indicator_lights.apply_schematic(&schematic);
        assert_eq!(indicator_lights.0, vec![true, true, false, false, false]);
        let schematic2 = ButtonWiringSchematic(vec![0, 4]);
        indicator_lights.apply_schematic(&schematic2);
        assert_eq!(indicator_lights.0, vec![false, true, false, false, true]);
    }
}
