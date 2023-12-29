use color_eyre::eyre;

#[derive(Debug)]
struct Boxes (Vec<Vec<(String, u32)>>);

impl Boxes {
    fn new() -> Boxes {
        Boxes (vec![Vec::new(); 256])
    }

    fn dash(&mut self, box_num: usize, label: String) {
        if let Some(lens) = self.0[box_num].iter().position(|x| x.0 == label) {
            self.0[box_num].remove(lens);
        }
    }
    fn equals(&mut self, box_num: usize, label: String, foc_len: u32) {
        match self.0[box_num].iter().position(|x| x.0 == label) {
            Some(lens) => self.0[box_num][lens] = (label, foc_len),
            None => self.0[box_num].push((label, foc_len)),
        }
    }
    fn find_focusing_power(&self) -> u32 {
        self.0.iter().enumerate().map(|(box_idx, b)| {
            b.iter().enumerate().map(|(lens_idx, lens)| {
                (1 + box_idx as u32) * (lens_idx as u32 + 1) * lens.1
            }).sum::<u32>()
        }).sum::<u32>()
    }
}

fn part1(input: &str) -> u32 {
    input.split(',').map(|step| {
        step.chars().fold(0, |value, ch| {
            let code = ch as u8;
            (value + code as u32) * 17 % 256
        })
    }).sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let mut boxes = Boxes::new();
    input.split(',').for_each(|step| {
        match step.split_once('=') {
            Some((label, num)) => {
                boxes.equals(algorytm(label), label.to_string(), num.parse::<u32>().unwrap())
            },
            None => {
                let label = step.trim_end_matches('-');
                boxes.dash(algorytm(label), label.to_string())
            },
        }
    });
    boxes.find_focusing_power()
}

fn algorytm(input: &str) -> usize {
    input.chars().fold(0, |value, ch| {
        let code = ch as u8;
        (value + code as usize) * 17 % 256
    })
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input15.txt");
    println!("{:?}", part2(input));
    Ok(())
}