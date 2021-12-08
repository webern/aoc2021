use crate::day8::data::test_data;
use crate::day8::enigma::SubmarineDisplay;
use crate::day8::Enigma;
use anyhow::{ensure, Context, Result};

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
        parse_displays::<10>(samples_string)?,
        parse_displays::<4>(readout)?,
    ))
}

fn parse_displays<const T: usize>(s: &str) -> Result<[SubmarineDisplay; T]> {
    let mut displays = [SubmarineDisplay::default(); T];
    for (index, word) in s.split(' ').enumerate() {
        ensure!(index < T, "Too many words being parsed, more than {}", T);
        displays[index] = SubmarineDisplay::parse(word)?;
    }
    Ok(displays)
}

#[test]
fn test_parse() {
    let input = test_data().unwrap();
    assert_eq!(input.len(), 10);
    let enigma = input.get(0).unwrap();
    let sample = enigma.sample(1).unwrap();
    let readout = enigma.readout(3).unwrap();
    assert_eq!(sample.lighted(), "abcdefg");
    assert_eq!(readout.lighted(), "bceg");
}
