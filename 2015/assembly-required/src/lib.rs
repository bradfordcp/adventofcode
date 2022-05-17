#[macro_use]
extern crate lazy_static;

pub mod signal;

use std::collections::HashMap;
use signal::*;

pub trait Circuit {
    fn new(schematic: &Vec<Signal>) -> Self;
    fn get(&self, id: &str) -> Option<u16>;
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
                            signals.insert(id.clone(), val.clone());
                            false
                        }
                        Signal::VALUE(Component::ID(id), Component::ID(src_id)) => {
                            if let Some(val) = signals.get(&src_id.clone()) {
                                let val = val.clone();
                                signals.insert(id.clone(), val);
                                false
                            } else {
                                true
                            }
                        }
                        Signal::NOT(Component::ID(id), Component::ID(src)) => {
                            if let Some(val) = signals.get(&src.clone()) {
                                let val = val.clone();
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
                                let a = a.clone();
                                let b = b.clone();
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
                                let a = a.clone();
                                let b = b.clone();

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
                                let a  = a.clone();
                                let b = b.clone();

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
                                let a  = a.clone();
                                let b = b.clone();

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
                                let input = input.clone();
                                let places = places.clone();

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
                                let input = input.clone();
                                let places = places.clone();
                                
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

    fn get(&self, id: &str) -> Option<u16> {
        self.signals.get(&id.to_string()).map(|v| v.clone())
    }
}

pub struct GraphCircuit {
    signals: HashMap<String, Signal>
}

impl Circuit for GraphCircuit {
    fn new(schematic: &Vec<Signal>) -> Self {
        let mut signals = HashMap::new();

        schematic.iter().for_each(|signal| {
            match signal.clone() {
                Signal::VALUE(Component::ID(id), _) => {signals.insert(id.clone(), signal.clone());},
                Signal::NOT(Component::ID(id), _) => {signals.insert(id.clone(), signal.clone());},
                Signal::AND(Component::ID(id), _, _) => {signals.insert(id.clone(), signal.clone());},
                Signal::OR(Component::ID(id), _, _) => {signals.insert(id.clone(), signal.clone());},
                Signal::XOR(Component::ID(id), _, _) => {signals.insert(id.clone(), signal.clone());},
                Signal::LSHIFT(Component::ID(id), _, _) => {signals.insert(id.clone(), signal.clone());},
                Signal::RSHIFT(Component::ID(id), _, _) => {signals.insert(id.clone(), signal.clone());},
                _ => panic!("Encountered unhandled signal {:?}", signal)
            }
        });

        GraphCircuit { signals }
    }

    fn get(&self, id: &str) -> Option<u16> {
        if let Some(signal) = self.signals.get(&id.to_string()) {
            match signal.clone() {
                Signal::VALUE(_, Component::VALUE(val)) => Some(val),
                _ => panic!("Encountered unhandled signal {:?}", signal)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    mod graph_circuit {
        use super::super::*;

        #[test]
        fn test_processing_signal_value() {
            let a = Signal::parse("0 -> foo");
            let b = Signal::parse("1 -> bar");
            let g = GraphCircuit::new(&vec![a.clone(), b.clone()]);

            let mut expected = HashMap::new();
            expected.insert("foo".to_string(), a);
            expected.insert("bar".to_string(), b);

            assert_eq!(expected, g.signals);

            let a = Signal::parse("0 -> foo");
            let b = Signal::parse("foo -> bar");
            let g = GraphCircuit::new(&vec![a.clone(), b.clone()]);

            let mut expected = HashMap::new();
            expected.insert("foo".to_string(), a);
            expected.insert("bar".to_string(), b);

            assert_eq!(expected, g.signals);
        }

        #[test]
        fn test_processing_signal_not() {
            let a = Signal::parse("0 -> a");
            let b = Signal::parse("NOT a -> b");
            let g = GraphCircuit::new(&vec![a.clone(), b.clone()]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);

            assert_eq!(expected, g.signals)
        }

        #[test]
        fn test_processing_signal_and() {
            let a = Signal::parse("0 -> a");
            let b = Signal::parse("1 -> b");
            let c = Signal::parse("a AND b -> c");
            let g = GraphCircuit::new(&vec![
                a.clone(),
                b.clone(),
                c.clone(),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);
            expected.insert("c".to_string(), c);

            assert_eq!(expected, g.signals);

            let a = Signal::parse("0 -> a");
            let b = Signal::parse("1 AND a -> b");
            let g = GraphCircuit::new(&vec![
                a.clone(),
                b.clone(),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);

            assert_eq!(expected, g.signals);
        }

        #[test]
        fn test_processing_signal_or() {
            let a = Signal::parse("0 -> a");
            let b = Signal::parse("1 -> b");
            let c = Signal::parse("a OR b -> c");
            let g = GraphCircuit::new(&vec![
                a.clone(),
                b.clone(),
                c.clone(),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);
            expected.insert("c".to_string(), c);

            assert_eq!(expected, g.signals)
        }

        #[test]
        fn test_processing_signal_xor() {
            let a = Signal::parse("0 -> a");
            let b = Signal::parse("1 -> b");
            let c = Signal::parse("a XOR b -> c");
            let g = GraphCircuit::new(&vec![
                a.clone(),
                b.clone(),
                c.clone(),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);
            expected.insert("c".to_string(), c);

            assert_eq!(expected, g.signals)
        }

        #[test]
        fn test_processing_signal_lshift() {
            let a = Signal::parse("1 -> a");
            let b = Signal::parse("a LSHIFT 2 -> b");
            let g = GraphCircuit::new(&vec![
                a.clone(),
                b.clone(),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);

            assert_eq!(expected, g.signals)
        }

        #[test]
        fn test_processing_signal_rshift() {
            let a = Signal::parse("1 -> a");
            let b = Signal::parse("a RSHIFT 2 -> b");
            let g = GraphCircuit::new(&vec![
                a.clone(),
                b.clone(),
            ]);

            let mut expected = HashMap::new();
            expected.insert("a".to_string(), a);
            expected.insert("b".to_string(), b);

            assert_eq!(expected, g.signals)
        }
    }

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

        #[test]
        fn test_get_signal() {
            let c = SimpleCircuit::new(&vec![Signal::parse("1 -> a")]);

            let expected = Some(1_u16);
            assert_eq!(expected, c.get("a"))
        }
    }
}
