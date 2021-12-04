use anyhow::{bail, Result};
use std::iter::Peekable;
use std::str::Chars;

pub(super) fn parse_numbers(delim: char, it: &mut Peekable<Chars>) -> Result<Vec<usize>> {
    let mut values = Vec::new();
    let mut buf = String::new();
    while let Some(c) = it.next() {
        match c {
            the_char if the_char == '\n' || the_char == delim => {
                if !buf.is_empty() {
                    values.push(clear_and_parse(&mut buf)?);
                }
                if c == delim {
                    continue;
                } else {
                    return Ok(values);
                }
            }
            '0'..='9' => buf.push(c),
            bad => bail!("Unexpected char `{}`", bad),
        }
    }
    if !buf.is_empty() {
        values.push(clear_and_parse(&mut buf)?);
    }
    Ok(values)
}

pub(super) fn clear_and_parse(buf: &mut String) -> Result<usize> {
    let value = buf.parse::<usize>()?;
    buf.clear();
    Ok(value)
}
