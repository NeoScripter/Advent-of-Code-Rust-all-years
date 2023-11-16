use color_eyre::eyre::{self, Result};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}
#[derive(Debug, Clone)]
enum Operation {
    Mul(Value),
    Add(Value),
}
impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match &self {
            Operation::Mul(val) => old * val.number(old),
            Operation::Add(val) => old + val.number(old),
        }
    }
}
#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u64),
}
impl Value {
    fn number(&self, old: u64) -> u64 {
        match self {
            Value::Num(n) => *n,
            Value::Old => old,
        }
    }
}
#[derive(Debug, Clone)]
struct Test {
    divisible: u64,
    true_recipient: usize,
    false_recipient: usize,
}

fn parse() -> Option<Vec<Monkey>> {
    let mut monkeys = Vec::new();
    let input = include_str!("input11.txt");
    for block in input.split("\r\n\r\n") {
        let mut lines = block.lines().skip(1);

        let (_, items) = lines.next()?.split_once(": ")?;
        let items = items
        .split_terminator(", ")
        .filter_map(|s| s.parse().ok())
        .collect();

        let (_, in_operation) = lines.next()?.rsplit_once("= old ")?;
        let (operation, operand) = in_operation.split_once(" ")?;
        let operand = match operand {
            "old" => Value::Old,
            _ => {
                let n = operand.parse().ok()?;
                Value::Num(n)
            }
        };

        let (_, divisible) = lines.next()?.rsplit_once("by ")?;
        let divisible = divisible.parse().ok()?;
        let (_, true_recipient) = lines.next()?.rsplit_once("monkey ")?;
        let true_recipient = true_recipient.parse().ok()?;
        let (_, false_recipient) = lines.next()?.rsplit_once("monkey ")?;
        let false_recipient = false_recipient.parse().ok()?;

        let operation = match operation {
            "+" => Operation::Add(operand),
            _ => Operation::Mul(operand),
        };

        let test = Test {
            divisible,
            true_recipient,
            false_recipient,
        };

        let monkey = Monkey {
            items,
            operation,
            test,
        };

        monkeys.push(monkey);
    }
    Some(monkeys)
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    
    let mut monkeys = parse().unwrap();
    let mut inspections = vec![0; monkeys.len()];
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test.divisible).product();

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            let monkey = monkeys[idx].clone();
            for old in items {
                inspections[idx] += 1;
                let new = monkey.operation.apply(old);
                let new = new % common_multiple;
                let idx = if new % monkey.test.divisible == 0 {
                    monkey.test.true_recipient
                } else {
                    monkey.test.false_recipient
                };
                let receiver = &mut monkeys[idx];
                receiver.items.push(new);
            }
        }
    }
    inspections.sort_unstable();
    let monkey_business: u64 = inspections.iter().rev().take(2).product();

    println!("{}", monkey_business);
    Ok(())
}