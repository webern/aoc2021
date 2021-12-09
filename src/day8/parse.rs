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
        parse_displays(samples_string)?,
        parse_displays(readout)?,
    ))
}

fn parse_displays(s: &str) -> Result<Vec<String>> {
    s.split(' ').map(|word| parse_word(word)).collect()
}

fn parse_word(s: &str) -> Result<String> {
    // I don't understand why the input is in seemingly random order, so I'm sorting it on the fly.
    let mut a = false;
    let mut b = false;
    let mut c = false;
    let mut d = false;
    let mut e = false;
    let mut f = false;
    let mut g = false;
    for x in s.chars() {
        match x {
            'a' => a = true,
            'b' => b = true,
            'c' => c = true,
            'd' => d = true,
            'e' => e = true,
            'f' => f = true,
            'g' => g = true,
            bad => bail!("Unexpected char '{}'", x),
        }
    }
    let mut sorted = String::new();
    if a {
        sorted.push('a');
    }

    if b {
        sorted.push('b');
    }

    if c {
        sorted.push('c');
    }

    if d {
        sorted.push('d');
    }

    if e {
        sorted.push('e');
    }

    if f {
        sorted.push('f');
    }

    if g {
        sorted.push('g');
    }

    Ok(sorted)
}

#[test]
fn test_parse() {
    let input = test_data().unwrap();
    assert_eq!(input.len(), 10);
    let enigma = input.get(0).unwrap();
    let sample = enigma.sample(1).unwrap();
    let readout = enigma.readout(3).unwrap();
    assert_eq!(sample.deref(), "abcdefg");
    assert_eq!(readout.deref(), "bceg");
}
