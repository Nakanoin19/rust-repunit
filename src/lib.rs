#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn convert(repcount: usize) -> usize {
    let rep1: String = "1".to_string();
    let rep2 = rep1.repeat(repcount);
    let repunit: usize = rep2.to_string().parse().unwrap();
    return repunit;
}

pub fn restore(repunit: usize) -> usize {
    let rep3: String = repunit.to_string();
    let rep4: &str = &rep3;
    let repcount: usize = rep4.match_indices("1").count();
    return repcount;
}