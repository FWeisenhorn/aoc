use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("inputs/day20.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    // println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let mut modules: HashMap<&str, Module> = read_input(input);
    let mut pulses_to_process: VecDeque<(&str, &str, Pulse)> = VecDeque::new();

    let mut lows = 0usize;
    let mut highs = 0usize;

    for _ in 0..1000 {
        pulses_to_process.push_front(("button", "broadcaster", Pulse::Low));

        while let Some((from, to, signal)) = pulses_to_process.pop_back() {
            match signal {
                Pulse::High => {
                    highs += 1;
                }
                Pulse::Low => {
                    lows += 1;
                }
            }

            let t = modules.get_mut(to).unwrap();

            if let Some(signal) = t.process_signal(from, signal) {
                for n in &t.downstream {
                    pulses_to_process.push_front((to, n, signal));
                }
            }
        }
    }

    (lows * highs).to_string()
}

fn part_b(input: &str) -> String {
    String::new()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    downstream: Vec<&'a str>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjunction {
        upstream: Vec<&'a str>,
        last_seen: Vec<Pulse>,
    },
    Broadcaster,
    // Button(ButtonModule),
    // OutputOnly
}

impl Module<'_> {
    fn process_signal(&mut self, from: &str, p: Pulse) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::FlipFlop(_) if p == Pulse::High => None,
            ModuleType::FlipFlop(false) => {
                self.module_type = ModuleType::FlipFlop(true);
                Some(Pulse::High)
            }
            ModuleType::FlipFlop(true) => {
                self.module_type = ModuleType::FlipFlop(false);
                Some(Pulse::Low)
            }
            ModuleType::Conjunction {
                upstream: ups,
                last_seen: las,
            } => {
                let n = ups.iter().position(|&x| x == from).unwrap();
                las[n] = p;
                if las.iter().all(|&x| x == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleType::Broadcaster => Some(p),
        }
    }
}

fn read_input(input: &str) -> HashMap<&str, Module> {
    let mut out: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            let t: Vec<&str> = b.split(", ").collect();

            if let Some(x) = a.strip_prefix('%') {
                (
                    x,
                    Module {
                        module_type: ModuleType::FlipFlop(false),
                        downstream: t,
                    },
                )
            } else if let Some(x) = a.strip_prefix('&') {
                (
                    x,
                    Module {
                        module_type: ModuleType::Conjunction {
                            last_seen: vec![],
                            upstream: vec![],
                        },
                        downstream: t,
                    },
                )
            } else {
                (
                    a,
                    Module {
                        module_type: ModuleType::Broadcaster,
                        downstream: t,
                    },
                )
            }
        })
        .collect();

    for node in out.clone() {
        node.1.downstream.iter().for_each(|&x| {
            if let Some(t) = out.get_mut(x) {
                if let ModuleType::Conjunction {
                    upstream: ups,
                    last_seen: las,
                } = &mut t.module_type
                {
                    ups.push(node.0);
                    las.push(Pulse::Low);
                }
            } else {
                out.insert(
                    x,
                    Module {
                        module_type: ModuleType::Broadcaster,
                        downstream: vec![],
                    },
                );
            };
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST_A1: &str = include_str!("tests/day20-a1.txt");
    const _TEST_A2: &str = include_str!("tests/day20-a2.txt");
    const _TESTRESULT_A1: &str = "32000000";
    const _TESTRESULT_A2: &str = "11687500";

    #[test]
    fn test_a1() {
        assert_eq!(part_a(_TEST_A1), _TESTRESULT_A1);
    }

    #[test]
    fn test_a2() {
        assert_eq!(part_a(_TEST_A2), _TESTRESULT_A2);
    }
}
