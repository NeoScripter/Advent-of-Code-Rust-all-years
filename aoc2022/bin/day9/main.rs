use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

pub fn main() {
    let input = include_str!("input9.txt");
    let start = Coord { x: 0, y: 0 };
    let mut knots = [start; 10];
    let mut seen = HashSet::new();
    seen.insert(knots[9]);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').expect("Invalid input format");
        let amount = amount.parse().expect("Failed to parse amount");

        for _ in 0..amount {
            // move head
            match dir {
                "U" => knots[0].y += 1,
                "D" => knots[0].y -= 1,
                "L" => knots[0].x -= 1,
                "R" => knots[0].x += 1,
                _ => panic!("Tried to move in invalid direction"),
            };
            
            for i in 1..10 {
                let diff = Coord {
                    x: knots[i - 1].x - knots[i].x,
                    y: knots[i - 1].y - knots[i].y,
                };
                let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;

                if not_touching {
                    knots[i].x += diff.x.signum();
                    knots[i].y += diff.y.signum();
                }
                
                if i == 9 {
                    seen.insert(knots[i]);
                }
            }
        }
    }

    println!("{}", seen.len());
}
