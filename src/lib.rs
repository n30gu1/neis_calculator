use std::error::Error;
use std::env;
use std::fs;

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    println!("{} bytes", calculate(contents));

    Ok(())
}

pub fn parse_arg(mut arg: env::Args) -> Result<String, &'static str>{
    arg.next();

    let filename = match arg.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    Ok(filename)
}

fn calculate(contents: String) -> usize {
    let mut bytes = contents.len();
    bytes += contents.lines().count() - 1;

    println!("{}", contents.lines().count());

    bytes
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn count() {
        let contents = String::from("\
다람쥐 헌 쳇바퀴 타고파.
A quick brown fox jumps over a lazy dog.");

        assert_eq!(76, calculate(contents));
    }
}