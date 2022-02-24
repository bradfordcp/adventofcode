#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Signal {
    NOT(Component, Component),
    AND(Component, Component, Component),
    OR(Component, Component, Component),
    XOR(Component, Component, Component),
    LSHIFT(Component, Component, Component),
    RSHIFT(Component, Component, Component),
    VALUE(Component, Component),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Component {
    ID(String),
    VALUE(u16),
}

impl Signal {
    pub fn parse(raw: &str) -> Signal {
        lazy_static! {
            static ref VALUE_RE: Regex = Regex::new(r"^([a-z0-9]+) -> ([a-z]+)$")
                .expect("Could not compile value regular expression");
            static ref NOT_GATE_RE: Regex = Regex::new(r"^NOT ([a-z0-9]+) -> ([a-z]+)$")
                .expect("Could not compile NOT gate regular expression");
            static ref AND_OR_XOR_RE: Regex =
                Regex::new(r"^([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z]+)$")
                    .expect("Could not compile AND, OR, XOR regular expression");
            static ref LSHIFT_RSHIFT_RE: Regex =
                Regex::new(r"^([a-z0-9]+) (LSHIFT|RSHIFT) (\d+) -> ([a-z]+)$")
                    .expect("Could not compile LSHIFT and RSHIFT regular expression");
        }

        if let Some(cap) = VALUE_RE.captures(raw) {
            return Signal::VALUE(
                Component::parse(cap.get(2).expect("Could not find wire ID").as_str()),
                Component::parse(cap.get(1).expect("Could not find value u16").as_str()),
            );
        };

        if let Some(cap) = NOT_GATE_RE.captures(raw) {
            return Signal::NOT(
                Component::parse(cap.get(2).expect("Could not find wire ID").as_str()),
                Component::parse(cap.get(1).expect("Could not find source wire ID").as_str()),
            );
        };

        if let Some(cap) = AND_OR_XOR_RE.captures(raw) {
            match cap.get(2).expect("Could not find operation").as_str() {
                "AND" => {
                    return Signal::AND(
                        Component::parse(
                            cap.get(4).expect("Could not find output wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(1).expect("Could not find source wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(3).expect("Could not find source wire ID").as_str(),
                        ),
                    )
                }
                "OR" => {
                    return Signal::OR(
                        Component::parse(
                            cap.get(4).expect("Could not find output wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(1).expect("Could not find source wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(3).expect("Could not find source wire ID").as_str(),
                        ),
                    )
                }
                "XOR" => {
                    return Signal::XOR(
                        Component::parse(
                            cap.get(4).expect("Could not find output wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(1).expect("Could not find source wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(3).expect("Could not find source wire ID").as_str(),
                        ),
                    )
                }
                op => panic!("Encountered unexpected operation: {}", op),
            }
        };

        if let Some(cap) = LSHIFT_RSHIFT_RE.captures(raw) {
            match cap.get(2).expect("Could not find operation").as_str() {
                "LSHIFT" => {
                    return Signal::LSHIFT(
                        Component::parse(
                            cap.get(4).expect("Could not find output wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(1).expect("Could not find source wire ID").as_str(),
                        ),
                        Component::parse(cap.get(3).expect("Could not find value usize").as_str()),
                    )
                }
                "RSHIFT" => {
                    return Signal::RSHIFT(
                        Component::parse(
                            cap.get(4).expect("Could not find output wire ID").as_str(),
                        ),
                        Component::parse(
                            cap.get(1).expect("Could not find source wire ID").as_str(),
                        ),
                        Component::parse(cap.get(3).expect("Could not find value usize").as_str()),
                    )
                }
                op => panic!("Encountered unexpected operation: {}", op),
            }
        };

        panic!("Could not parse signal: {}", raw);
    }
}

impl Component {
    fn parse(raw: &str) -> Component {
        lazy_static! {
            static ref COMPONENT_RE: Regex =
                Regex::new(r"[a-z0-9]+").expect("Could not compile component regular expression");
        }

        if let Some(caps) = COMPONENT_RE.captures(raw) {
            let cap = caps.get(0).expect("Could not match ID or VALUE").as_str();
            if let Ok(value) = cap.parse::<u16>() {
                Component::VALUE(value)
            } else {
                Component::ID(cap.to_string())
            }
        } else {
            panic!("Failed to match ID or VALUE");
        }
    }
}

pub struct Circuit {
    signals: HashMap<String, u16>,
}

impl Circuit {
    pub fn new(schematic: &Vec<Signal>) -> Self {
        let mut to_resolve: Vec<Signal> = schematic.clone();
        let mut signals = HashMap::<String, u16>::new();

        let mut last_to_resolve_count = to_resolve.len() + 1;

        loop {
            if to_resolve.len() == 0 || last_to_resolve_count == to_resolve.len() {
                break;
            } else {
                last_to_resolve_count = to_resolve.len();
            }

            to_resolve = to_resolve
                .into_iter()
                .filter(|signal| {
                    match signal {
                        Signal::VALUE(Component::ID(id), Component::VALUE(val)) => {
                            signals.insert(id.clone(), *val);
                            false
                        }
                        Signal::VALUE(Component::ID(id), Component::ID(src_id)) => {
                            if let Some(val) = signals.get(&src_id.clone()) {
                                signals.insert(id.clone(), *val);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::NOT(Component::ID(id), Component::ID(src)) => {
                            if let Some(val) = signals.get(&src.clone()) {
                                signals.insert(id.clone(), !val);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::AND(Component::ID(id), Component::ID(in1), Component::ID(in2)) => {
                            if let (Some(a), Some(b)) =
                                (signals.get(&in1.clone()), signals.get(&in2.clone()))
                            {
                                signals.insert(id.clone(), a & b);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::AND(Component::ID(id), Component::VALUE(a), Component::ID(in2)) => {
                            if let Some(b) =
                                signals.get(&in2.clone())
                            {
                                signals.insert(id.clone(), a & b);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::OR(Component::ID(id), Component::ID(in1), Component::ID(in2)) => {
                            if let (Some(a), Some(b)) =
                                (signals.get(&in1.clone()), signals.get(&in2.clone()))
                            {
                                signals.insert(id.clone(), a | b);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::XOR(Component::ID(id), Component::ID(in1), Component::ID(in2)) => {
                            if let (Some(a), Some(b)) =
                                (signals.get(&in1.clone()), signals.get(&in2.clone()))
                            {
                                signals.insert(id.clone(), a ^ b);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::LSHIFT(
                            Component::ID(id),
                            Component::ID(input_id),
                            Component::VALUE(places),
                        ) => {
                            if let Some(input) = signals.get(&input_id.clone()) {
                                signals.insert(id.clone(), input << places);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::RSHIFT(
                            Component::ID(id),
                            Component::ID(input_id),
                            Component::VALUE(places),
                        ) => {
                            if let Some(input) = signals.get(&input_id.clone()) {
                                signals.insert(id.clone(), input >> places);
                                false
                            } else {
                                true
                            }
                        }
                        s => {
                            panic!("Encountered unhandled signal: {:?}", s);
                        }
                    }
                })
                .collect();
        }

        Circuit { signals }
    }

    pub fn get(&self, id: &str) -> Option<&u16> {
        self.signals.get(&id.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod signal {
        use super::super::*;

        #[test]
        fn test_parse_value() {
            let result = Signal::parse("0 -> foo");
            let expected = Signal::VALUE(Component::parse("foo"), Component::parse("0"));
            assert_eq!(expected, result);

            let result = Signal::parse("1 -> bar");
            let expected = Signal::VALUE(Component::parse("bar"), Component::parse("1"));
            assert_eq!(expected, result);

            let result = Signal::parse("foo -> bar");
            let expected = Signal::VALUE(Component::parse("bar"), Component::parse("foo"));
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_not() {
            let result = Signal::parse("NOT foo -> bar");
            let expected = Signal::NOT(Component::parse("bar"), Component::parse("foo"));
            assert_eq!(expected, result);

            let result = Signal::parse("NOT 1 -> bar");
            let expected = Signal::NOT(Component::parse("bar"), Component::parse("1"));
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_and() {
            let result = Signal::parse("a AND b -> c");
            let expected = Signal::AND(
                Component::parse("c"),
                Component::parse("a"),
                Component::parse("b"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("1 AND b -> c");
            let expected = Signal::AND(
                Component::parse("c"),
                Component::parse("1"),
                Component::parse("b"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("a AND 1 -> c");
            let expected = Signal::AND(
                Component::parse("c"),
                Component::parse("a"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_or() {
            let result = Signal::parse("a OR b -> c");
            let expected = Signal::OR(
                Component::parse("c"),
                Component::parse("a"),
                Component::parse("b"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("1 OR b -> c");
            let expected = Signal::OR(
                Component::parse("c"),
                Component::parse("1"),
                Component::parse("b"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("a OR 1 -> c");
            let expected = Signal::OR(
                Component::parse("c"),
                Component::parse("a"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_xor() {
            let result = Signal::parse("a XOR b -> c");
            let expected = Signal::XOR(
                Component::parse("c"),
                Component::parse("a"),
                Component::parse("b"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("1 XOR b -> c");
            let expected = Signal::XOR(
                Component::parse("c"),
                Component::parse("1"),
                Component::parse("b"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("a XOR 1 -> c");
            let expected = Signal::XOR(
                Component::parse("c"),
                Component::parse("a"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_lshift() {
            let result = Signal::parse("a LSHIFT 1 -> b");
            let expected = Signal::LSHIFT(
                Component::parse("b"),
                Component::parse("a"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("1 LSHIFT 1 -> b");
            let expected = Signal::LSHIFT(
                Component::parse("b"),
                Component::parse("1"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_rshift() {
            let result = Signal::parse("a RSHIFT 1 -> b");
            let expected = Signal::RSHIFT(
                Component::parse("b"),
                Component::parse("a"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);

            let result = Signal::parse("1 RSHIFT 1 -> b");
            let expected = Signal::RSHIFT(
                Component::parse("b"),
                Component::parse("1"),
                Component::parse("1"),
            );
            assert_eq!(expected, result);
        }
    }

    mod component {
        use super::super::*;

        #[test]
        fn test_parse_id() {
            let result = Component::parse("foo");
            let expected = Component::ID("foo".to_string());

            assert_eq!(expected, result);
        }

        #[test]
        fn test_parse_value() {
            let result = Component::parse("1");
            let expected = Component::VALUE(1);

            assert_eq!(expected, result);
        }
    }

    mod circuit {
        use super::super::*;

        #[test]
        fn test_processing_signal_value() {
            let c = Circuit::new(&vec![Signal::parse("0 -> foo"), Signal::parse("1 -> bar")]);

            let mut expected = HashMap::new();
            expected.insert("foo".to_string(), 0);
            expected.insert("bar".to_string(), 1);

            assert_eq!(expected, c.signals);

            let c = Circuit::new(&vec![Signal::parse("0 -> foo"), Signal::parse("foo -> bar")]);

            let mut expected = HashMap::new();
            expected.insert("foo".to_string(), 0);
            expected.insert("bar".to_string(), 0);

            assert_eq!(expected, c.signals);
        }

        #[test]
        fn test_processing_signal_not() {
            let c = Circuit::new(&vec![Signal::parse("0 -> a"), Signal::parse("NOT a -> b")]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 65535);

            assert_eq!(expected, c.signals)
        }

        #[test]
        fn test_processing_signal_and() {
            let c = Circuit::new(&vec![
                Signal::parse("0 -> a"),
                Signal::parse("1 -> b"),
                Signal::parse("a AND b -> c"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 1);
            expected.insert("c".to_string(), 0);

            assert_eq!(expected, c.signals);

            let c = Circuit::new(&vec![
                Signal::parse("0 -> a"),
                Signal::parse("1 AND a -> b"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 0);

            assert_eq!(expected, c.signals);
        }

        #[test]
        fn test_processing_signal_or() {
            let c = Circuit::new(&vec![
                Signal::parse("0 -> a"),
                Signal::parse("1 -> b"),
                Signal::parse("a OR b -> c"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 1);
            expected.insert("c".to_string(), 1);

            assert_eq!(expected, c.signals)
        }

        #[test]
        fn test_processing_signal_xor() {
            let c = Circuit::new(&vec![
                Signal::parse("0 -> a"),
                Signal::parse("1 -> b"),
                Signal::parse("a XOR b -> c"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 1);
            expected.insert("c".to_string(), 1);

            assert_eq!(expected, c.signals)
        }

        #[test]
        fn test_processing_signal_lshift() {
            let c = Circuit::new(&vec![
                Signal::parse("1 -> a"),
                Signal::parse("a LSHIFT 2 -> b"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 1);
            expected.insert("b".to_string(), 4);

            assert_eq!(expected, c.signals)
        }

        #[test]
        fn test_processing_signal_rshift() {
            let c = Circuit::new(&vec![
                Signal::parse("1 -> a"),
                Signal::parse("a RSHIFT 2 -> b"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 1);
            expected.insert("b".to_string(), 0);

            assert_eq!(expected, c.signals)
        }
    }
}
