use std::{
    fs::File,
    io::{self, BufRead},
};

pub(crate) fn parse_file<F, T>(input: File, parser: F) -> Vec<T>
where
    F: FnMut(String) -> T,
{
    io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .map(parser)
        .collect()
}
