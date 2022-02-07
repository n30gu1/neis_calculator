use std::error::Error;
use std::env;
use std::fs;

pub fn run(filenames: Vec<String>) -> Result<(), Box<dyn Error>> {
    for filename in filenames {
        let contents = fs::read_to_string(&filename)?;
        println!("{} bytes, {}", calculate(contents), filename);
    }

    Ok(())
}

pub fn parse_arg(mut arg: env::Args) -> Result<Vec<String>, &'static str>{
    arg.next();

    let mut filenames = Vec::new();


    loop {
        match arg.next() {
            Some(arg) => filenames.push(arg),
            None => break,
        };
    }

    if filenames.len() < 1 {
        return Err("Didn't get a file path")
    }

    Ok(filenames)
}

fn calculate(contents: String) -> usize {
    let mut bytes = contents.len();
    bytes += contents.lines().count() - 1;

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