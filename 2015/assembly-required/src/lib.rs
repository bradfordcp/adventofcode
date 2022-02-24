#[macro_use]
extern crate lazy_static;

pub mod signal;

use std::collections::HashMap;
use signal::*;

pub trait Circuit {
    fn new(schematic: &Vec<Signal>) -> Self;
    fn get(&self, id: &str) -> Option<&u16>;
}

pub struct SimpleCircuit {
    signals: HashMap<String, u16>,
}

impl Circuit for SimpleCircuit {
    fn new(schematic: &Vec<Signal>) -> Self {
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

        SimpleCircuit { signals }
    }

    fn get(&self, id: &str) -> Option<&u16> {
        self.signals.get(&id.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    

    mod simple_circuit {
        use super::super::*;

        #[test]
        fn test_processing_signal_value() {
            let c = SimpleCircuit::new(&vec![Signal::parse("0 -> foo"), Signal::parse("1 -> bar")]);

            let mut expected = HashMap::new();
            expected.insert("foo".to_string(), 0);
            expected.insert("bar".to_string(), 1);

            assert_eq!(expected, c.signals);

            let c = SimpleCircuit::new(&vec![Signal::parse("0 -> foo"), Signal::parse("foo -> bar")]);

            let mut expected = HashMap::new();
            expected.insert("foo".to_string(), 0);
            expected.insert("bar".to_string(), 0);

            assert_eq!(expected, c.signals);
        }

        #[test]
        fn test_processing_signal_not() {
            let c = SimpleCircuit::new(&vec![Signal::parse("0 -> a"), Signal::parse("NOT a -> b")]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 65535);

            assert_eq!(expected, c.signals)
        }

        #[test]
        fn test_processing_signal_and() {
            let c = SimpleCircuit::new(&vec![
                Signal::parse("0 -> a"),
                Signal::parse("1 -> b"),
                Signal::parse("a AND b -> c"),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), 0);
            expected.insert("b".to_string(), 1);
            expected.insert("c".to_string(), 0);

            assert_eq!(expected, c.signals);

            let c = SimpleCircuit::new(&vec![
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
            let c = SimpleCircuit::new(&vec![
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
            let c = SimpleCircuit::new(&vec![
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
            let c = SimpleCircuit::new(&vec![
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
            let c = SimpleCircuit::new(&vec![
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
