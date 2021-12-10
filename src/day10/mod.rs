/*!

# Day 10:Syntax Scoring

You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:

Syntax error in navigation subsystem on line: all of them
All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem
(your puzzle input).

The navigation subsystem syntax is made of several lines containing chunks. There are one or more
chunks on each line, and chunks contain zero or more other chunks. Adjacent chunks are not separated
by any delimiter; if one chunk stops, the next chunk (if any) can immediately start. Every chunk
must open and close with one of four legal pairs of matching characters:

```text
If a chunk opens with (, it must close with ).
If a chunk opens with [, it must close with ].
If a chunk opens with {, it must close with }.
If a chunk opens with <, it must close with >.
```text

So, `()` is a legal chunk that contains no other chunks, as is `[]`. More complex but valid chunks
include `([])`, `{()()()}`, `<([{}])>`, `[<>({}){}[([])<>]]`, and even `(((((((((())))))))))`.

Some lines are incomplete, but others are corrupted. Find and discard the corrupted lines first.

A corrupted line is one where a chunk closes with the wrong character - that is, where the
characters it opens and closes with do not form one of the four legal pairs listed above.

Examples of corrupted chunks include (], {()()()>, (((()))}, and <([]){()}[{}]). Such a chunk can
appear anywhere within a line, and its presence causes the whole line to be considered corrupted.

For example, consider the following navigation subsystem:

```text
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
```

Some of the lines aren't corrupted, just incomplete; you can ignore these lines for now. The
remaining five lines are corrupted:

```text
{([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
[[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
[{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
[<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
<{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
```

Stop at the first incorrect closing character on each corrupted line.

Did you know that syntax checkers actually have contests to see who can get the high score for
syntax errors in a file? It's true! To calculate the syntax error score for a line, take the first
illegal character on the line and look it up in the following table:

```text
): 3 points.
]: 57 points.
}: 1197 points.
>: 25137 points.
```

In the above example, an illegal ) was found twice (2*3 = 6 points), an illegal ] was found once (57 points), an illegal } was found once (1197 points), and an illegal > was found once (25137 points). So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!

Find the first illegal character in each corrupted line of the navigation subsystem. What is the total syntax error score for those errors?

!*/

mod data;

use crate::day10::data::INPUT_DATA;
use std::collections::LinkedList;

pub fn solve() {
    println!("Part 1: {}", solve_part_1(&INPUT_DATA));
    println!("Part 2: {}", solve_part_2(&INPUT_DATA));
}

fn solve_part_1(input: &str) -> usize {
    let mut corruption_score = 0usize;
    'line_loop: for line in input.lines() {
        let mut stack = LinkedList::new();
        for c in line.chars() {
            if is_open(c) {
                stack.push_back(c);
            } else {
                let open_char = match stack.pop_back() {
                    None => {
                        corruption_score += score(c);
                        continue 'line_loop;
                    }
                    Some(o) => o,
                };
                if !is_match(open_char, c) {
                    corruption_score += score(c);
                    continue 'line_loop;
                }
            }
        }
    }
    corruption_score
}

fn is_open(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn is_match(open: char, close: char) -> bool {
    matches!(
        (open, close),
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
    )
}

fn score(c: char) -> usize {
    match c {
        '(' | ')' => 3,
        '[' | ']' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => 0,
    }
}

fn solve_part_2(input: &str) -> usize {
    let mut line_scores = Vec::new();
    'line_loop: for line in input.lines() {
        let mut stack = LinkedList::new();
        for c in line.chars() {
            if is_open(c) {
                stack.push_back(c);
            } else {
                let open_char = match stack.pop_back() {
                    None => {
                        continue 'line_loop;
                    }
                    Some(o) => o,
                };
                if !is_match(open_char, c) {
                    continue 'line_loop;
                }
            }
        }
        if !stack.is_empty() {
            let mut line_score = 0usize;
            for &item in stack.iter().rev() {
                line_score *= 5;
                line_score += complement_points(item);
            }
            line_scores.push(line_score);
        }
    }
    line_scores.sort();
    let middle = line_scores.len() / 2;
    *line_scores.get(middle).unwrap()
}

fn complement_points(c: char) -> usize {
    match c {
        '(' | ')' => 1,
        '[' | ']' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        _ => 0,
    }
}

#[test]
fn solve_part_1_test() {
    let answer = solve_part_1(crate::day10::data::TEST);
    assert_eq!(answer, 26397);
}

#[test]
fn solve_part_2_test() {
    let answer = solve_part_2(crate::day10::data::TEST);
    assert_eq!(answer, 288957);
}
