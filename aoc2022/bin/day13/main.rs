use color_eyre::eyre;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input13.txt");
    let mut pairs = vec![];

    input.split("\r\n\r\n").map(|pair| {
        pair.lines().map(|line| {
            let _char = line.chars();
           
        })
    });
    for pair in input.split("\r\n\r\n") {
        pairs.push(pair);
    }

    println!("{:?}", pairs);
    Ok(())
}