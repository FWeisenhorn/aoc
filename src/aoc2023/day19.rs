use std::collections::HashMap;

type Range = [[u32; 2]; 4];

const INPUT: &str = include_str!("inputs/day19.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let input = input.split_once("\n\n").unwrap();
    let workflows: HashMap<&str, Workflow> = workflows_from_input(input.0);

    let parts: Vec<[u32; 4]> = input
        .1
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| {
                    if s.ends_with('}') {
                        s[s.find('=').unwrap() + 1..s.len() - 1]
                            .parse::<u32>()
                            .unwrap()
                    } else {
                        s[s.find('=').unwrap() + 1..].parse::<u32>().unwrap()
                    }
                })
                .collect::<Vec<u32>>()
        })
        .map(|x| [x[0], x[1], x[2], x[3]])
        .collect();

    let mut accepted = vec![];

    for part in &parts {
        let mut workflow = "in";
        loop {
            if workflow == "R" {
                break;
            }
            if workflow == "A" {
                accepted.push(part);
                break;
            }
            workflow = apply_workflow(part, &workflows[workflow]);
        }
    }

    accepted
        .iter()
        .map(|x| x.iter().sum::<u32>())
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    let input = input.split_once("\n\n").unwrap();
    let workflows: HashMap<&str, Workflow> = workflows_from_input(input.0);

    let mut accepted: Vec<Range> = vec![];
    let mut to_check: Vec<(Range, &str)> = vec![([[1, 4000]; 4], "in")];

    while let Some(x) = to_check.pop() {
        if x.1 == "R" {
            continue;
        }
        if x.1 == "A" {
            accepted.push(x.0);
            continue;
        }

        let t = &workflows[x.1];
        to_check.append(&mut apply_workflow_to_range(&x.0, t));
    }

    accepted
        .iter()
        .map(|r| {
            r.iter()
                .map(|x| {
                    if x[1] >= x[0] {
                        u128::from(x[1] - x[0] + 1)
                    } else {
                        0
                    }
                })
                .product::<u128>()
        })
        .sum::<u128>()
        .to_string()
}

struct Rule<'a> {
    comp_letter: usize,
    comp_op: char,
    comp_value: u32,
    go_to: &'a str,
}

fn rule_from_input(input: &str) -> Rule {
    // "a<2006:qkq"
    let (out_letter, t, out_op);
    if input.contains('<') {
        (out_letter, t) = input.split_once('<').unwrap();
        out_op = '<';
    } else if input.contains('>') {
        (out_letter, t) = input.split_once('>').unwrap();
        out_op = '>';
    } else {
        unreachable!()
    }

    let (c, d) = t.split_once(':').unwrap();

    let a = match out_letter {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => unreachable!(),
    };

    Rule {
        comp_letter: a,
        comp_op: out_op,
        comp_value: c.parse().unwrap(),
        go_to: d,
    }
}

fn apply_rule<'a>(val: &[u32; 4], rule: &'a Rule) -> Option<&'a str> {
    match rule.comp_op {
        '<' => {
            if val[rule.comp_letter] < rule.comp_value {
                Some(rule.go_to)
            } else {
                None
            }
        }
        '>' => {
            if val[rule.comp_letter] > rule.comp_value {
                Some(rule.go_to)
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn apply_rule_to_range(val: &Range, rule: &Rule) -> [Option<Range>; 2] {
    if rule.comp_op == '<' {
        if val[rule.comp_letter][1] < rule.comp_value {
            // whole range passes
            [Some(*val), None]
        } else if val[rule.comp_letter][0] >= rule.comp_value {
            // range completely fails
            [None, Some(*val)]
        } else {
            let mut t1 = *val;
            let mut t2 = *val;

            t1[rule.comp_letter][1] = rule.comp_value - 1;
            t2[rule.comp_letter][0] = rule.comp_value;

            [Some(t1), Some(t2)]
        }
    } else if rule.comp_op == '>' {
        if val[rule.comp_letter][0] > rule.comp_value {
            // whole range passes
            [Some(*val), None]
        } else if val[rule.comp_letter][1] <= rule.comp_value {
            // range completely fails
            [None, Some(*val)]
        } else {
            let mut t1 = *val;
            let mut t2 = *val;

            t1[rule.comp_letter][0] = rule.comp_value + 1;
            t2[rule.comp_letter][1] = rule.comp_value;

            [Some(t1), Some(t2)]
        }
    } else {
        unreachable!()
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    go_to_end: &'a str,
}

fn workflow_from_string(input: &str) -> Workflow {
    // "a<2006:qkq,m>2090:A,rfg"
    let n = input.split(',').count();
    let k: Vec<Rule> = input.split(',').take(n - 1).map(rule_from_input).collect();
    Workflow {
        rules: k,
        go_to_end: input.split(',').last().unwrap(),
    }
}

fn workflows_from_input(input: &str) -> HashMap<&str, Workflow> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('{').unwrap();
            (a, workflow_from_string(&b[0..b.len() - 1]))
        })
        .collect()
}

fn apply_workflow<'a>(val: &[u32; 4], workflow: &'a Workflow) -> &'a str {
    workflow
        .rules
        .iter()
        .find_map(|rule| apply_rule(val, rule))
        .unwrap_or(workflow.go_to_end)
}

fn apply_workflow_to_range<'a>(val: &Range, workflow: &'a Workflow) -> Vec<(Range, &'a str)> {
    let mut out = vec![];

    let mut t = *val;

    for rule in &workflow.rules {
        match apply_rule_to_range(&t, rule) {
            [Some(a), Some(x)] => {
                out.push((a, rule.go_to));
                t = x;
            }
            [Some(a), None] => {
                out.push((a, rule.go_to));
            }
            [None, Some(x)] => {
                t = x;
            }
            _ => unreachable!(),
        }
    }

    out.push((t, workflow.go_to_end));

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day19.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "19114");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "167409079868000");
    }
}
