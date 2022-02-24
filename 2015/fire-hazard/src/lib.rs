#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

impl Instruction {
    pub fn parse(instruction: &str) -> Instruction {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").expect("Could not compile instruction regular expression");
        }

        let cap = INSTRUCTION_RE
                .captures(instruction)
                .expect("Could not find any captures");
        let tl = (
            cap.get(2)
                .expect("TL X coordinate not found")
                .as_str()
                .parse::<usize>()
                .expect("Could not parse coordinate"),
            cap.get(3)
                .expect("TL Y coordinate not found")
                .as_str()
                .parse::<usize>()
                .expect("Could not parse coordinate"),
        );
        let br = (
            cap.get(4)
                .expect("BR X coordinate not found")
                .as_str()
                .parse::<usize>()
                .expect("Could not parse coordinate"),
            cap.get(5)
                .expect("BR Y coordinate not found")
                .as_str()
                .parse::<usize>()
                .expect("Could not parse coordinate"),
        );

        match cap.get(1).expect("Could not match instruction").as_str() {
            "turn on" => Instruction::TurnOn(tl, br),
            "turn off" => Instruction::TurnOff(tl, br),
            "toggle" => Instruction::Toggle(tl, br),
            inst => panic!("Unexpected instruction encountered {}", inst),
        }
    }
}

fn x_y_to_index(width: &usize, x: &usize, y: &usize) -> usize {
    (y * width) + x
}

fn rect_to_ranges(width: &usize, tl: &(usize, usize), br: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::with_capacity(br.1 - tl.1);

    for i in tl.1..=br.1 {
        let range = (
            x_y_to_index(width, &tl.0, &i),
            x_y_to_index(width, &br.0, &i),
        );
        res.push(range);
    }

    res
}

pub struct LightDisplay {
    width: usize,
    lights: Vec<bool>,
}

impl LightDisplay {
    pub fn new(width: usize, height: usize) -> LightDisplay {
        let lights = vec![false; width * height];

        LightDisplay { width, lights }
    }

    pub fn process_instructions(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.process_instruction(instruction);
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(tl, br) => {
                let ranges = rect_to_ranges(&self.width, tl, br);
                for range in ranges {
                    for i in range.0..=range.1 {
                        if let Some(light) = self.lights.get_mut(i) {
                            *light = true
                        }
                    }
                }
            }
            Instruction::TurnOff(tl, br) => {
                let ranges = rect_to_ranges(&self.width, tl, br);
                for range in ranges {
                    for i in range.0..=range.1 {
                        if let Some(light) = self.lights.get_mut(i) {
                            *light = false
                        }
                    }
                }
            }
            Instruction::Toggle(tl, br) => {
                let ranges = rect_to_ranges(&self.width, tl, br);
                for range in ranges {
                    for i in range.0..=range.1 {
                        if let Some(light) = self.lights.get_mut(i) {
                            *light = !*light
                        }
                    }
                }
            }
        }
    }

    pub fn total_lit(&self) -> usize {
        self.lights.iter().filter(|e| **e).count()
    }
}

pub struct DimmableLightDisplay {
    width: usize,
    lights: Vec<usize>,
}

impl DimmableLightDisplay {
    pub fn new(width: usize, height: usize) -> DimmableLightDisplay {
        let lights = vec![0; width * height];

        DimmableLightDisplay { width, lights }
    }

    pub fn process_instructions(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.process_instruction(&instruction);
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(tl, br) => {
                let ranges = rect_to_ranges(&self.width, tl, br);
                for range in ranges {
                    for i in range.0..=range.1 {
                        if let Some(light) = self.lights.get_mut(i) {
                            *light = *light + 1
                        }
                    }
                }
            }
            Instruction::TurnOff(tl, br) => {
                let ranges = rect_to_ranges(&self.width, tl, br);
                for range in ranges {
                    for i in range.0..=range.1 {
                        if let Some(light) = self.lights.get_mut(i) {
                            if *light != 0 {
                                *light = *light - 1
                            }
                        }
                    }
                }
            }
            Instruction::Toggle(tl, br) => {
                let ranges = rect_to_ranges(&self.width, tl, br);
                for range in ranges {
                    for i in range.0..=range.1 {
                        if let Some(light) = self.lights.get_mut(i) {
                            *light = *light + 2
                        }
                    }
                }
            }
        }
    }

    pub fn total_brightness(&self) -> usize {
        self.lights.iter().fold(0, |acc, ele| acc + ele)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_x_y_to_index() {
        assert_eq!(x_y_to_index(&10, &0, &0), 0);
        assert_eq!(x_y_to_index(&10, &1, &1), 11)
    }

    #[test]
    fn test_rect_to_ranges() {
        let ranges = rect_to_ranges(&1000, &(0, 0), &(1, 1));
        let expected = vec![(0, 1), (1000, 1001)];

        assert_eq!(ranges.len(), expected.len());
        assert_eq!(ranges, expected);
    }

    mod instruction {
        use super::super::*;

        #[test]
        fn test_parse_turn_on() {
            let raw = "turn on 0,0 through 0,0";
            let expected = Instruction::TurnOn((0, 0), (0, 0));
            let result = Instruction::parse(raw);
            assert_eq!(expected, result);

            let raw = "turn on 0,0 through 1,1";
            let expected = Instruction::TurnOn((0, 0), (1, 1));
            let result = Instruction::parse(raw);
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_turn_off() {
            let raw = "turn off 0,0 through 0,0";
            let expected = Instruction::TurnOff((0, 0), (0, 0));
            let result = Instruction::parse(raw);
            assert_eq!(expected, result);

            let raw = "turn off 0,0 through 1,1";
            let expected = Instruction::TurnOff((0, 0), (1, 1));
            let result = Instruction::parse(raw);
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_toggle() {
            let raw = "toggle 0,0 through 0,0";
            let expected = Instruction::Toggle((0, 0), (0, 0));
            let result = Instruction::parse(raw);
            assert_eq!(expected, result);

            let raw = "toggle 0,0 through 1,1";
            let expected = Instruction::Toggle((0, 0), (1, 1));
            let result = Instruction::parse(raw);
            assert_eq!(expected, result);
        }
    }

    mod light_display {
        use super::super::*;

        #[test]
        fn test_new() {
            let ld = LightDisplay::new(1000, 1000);
            let expected = false;

            assert_eq!(ld.lights.capacity(), 1_000_000);
            assert_eq!(ld.lights.get(0), Some(&expected));
        }

        #[test]
        fn test_process_instruction_turn_on() {
            let mut ld = LightDisplay::new(10, 10);
            ld.process_instruction(&Instruction::TurnOn((0, 0), (0, 0)));

            assert_eq!(ld.lights[0], true)
        }

        #[test]
        fn test_process_instruction_turn_off() {
            let mut ld = LightDisplay::new(10, 10);
            ld.process_instruction(&Instruction::TurnOff((0, 0), (0, 0)));

            assert_eq!(ld.lights[0], false)
        }

        #[test]
        fn test_process_instruction_toggle() {
            let mut ld = LightDisplay::new(10, 10);

            ld.process_instruction(&Instruction::Toggle((0, 0), (0, 0)));
            assert_eq!(ld.lights[0], true);

            ld.process_instruction(&Instruction::TurnOn((0, 1), (1, 1)));
            assert_eq!(ld.lights[10], true);
            assert_eq!(ld.lights[11], true);
        }

        #[test]
        fn test_total_lit() {
            let mut ld = LightDisplay::new(10, 10);

            ld.process_instruction(&Instruction::Toggle((0, 0), (0, 0)));
            assert_eq!(ld.total_lit(), 1);
        }
    }

    mod dimmable_light_display {
        use super::super::*;

        #[test]
        fn test_new() {
            let ld = DimmableLightDisplay::new(1000, 1000);
            let expected = 0;

            assert_eq!(ld.lights.capacity(), 1_000_000);
            assert_eq!(ld.lights.get(0), Some(&expected));
        }

        #[test]
        fn test_process_instruction_turn_on() {
            let mut ld = DimmableLightDisplay::new(10, 10);
            ld.process_instruction(&Instruction::TurnOn((0, 0), (0, 0)));

            assert_eq!(ld.lights[0], 1)
        }

        #[test]
        fn test_process_instruction_turn_off() {
            let mut ld = DimmableLightDisplay::new(10, 10);
            ld.process_instruction(&Instruction::TurnOff((0, 0), (0, 0)));

            assert_eq!(ld.lights[0], 0);

            ld.process_instruction(&Instruction::Toggle((0, 0), (0, 0)));
            ld.process_instruction(&Instruction::TurnOff((0, 0), (0, 0)));

            assert_eq!(ld.lights[0], 1);
        }

        #[test]
        fn test_process_instruction_toggle() {
            let mut ld = DimmableLightDisplay::new(10, 10);

            ld.process_instruction(&Instruction::Toggle((0, 0), (0, 0)));
            assert_eq!(ld.lights[0], 2);
        }

        #[test]
        fn test_total_brightness() {
            let mut ld = DimmableLightDisplay::new(10, 10);

            ld.process_instruction(&Instruction::Toggle((0, 0), (0, 0)));
            assert_eq!(ld.total_brightness(), 2);
        }
    }
}
