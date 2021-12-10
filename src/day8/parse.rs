use crate::day8::data::test_data;
use crate::day8::Enigma;
use anyhow::{bail, ensure, Context, Result};
use std::convert::{Infallible, TryInto};
use std::ops::Deref;

pub(super) fn parse(s: &str) -> Result<Vec<Enigma>> {
    s.lines().map(|line| parse_one_line(line)).collect()
}

fn parse_one_line(s: &str) -> Result<Enigma> {
    let mut iter = s.split(" | ");
    let samples_string = iter
        .next()
        .context("Unable to find signal samples when parsing")?;
    let readout = iter
        .next()
        .context("Unable to find readout samples when parsing")?;
    Ok(Enigma::new(
        parse_displays(samples_string),
        parse_displays(readout),
    ))
}

fn parse_displays(s: &str) -> Vec<&str> {
    s.split(' ').map(|word| word).collect()
}

#[test]
fn test_parse() {
    let input = test_data().unwrap();
    assert_eq!(input.len(), 10);
    let enigma = input.get(0).unwrap();
    let sample = enigma.samples()[1];
    let readout = enigma.readouts()[3];
    assert_eq!(sample.deref(), "cfbegad");
    assert_eq!(readout.deref(), "gcbe");
}
