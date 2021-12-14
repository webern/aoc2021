# Advent of Code 2021

This is the first time I have tried this.
I'm doing this in Rust.
Today is day 2, we'll see how long I can keep up.

### Days 1-4

I did these, but hadn't started journaling yet.

### Day 5

Uh oh, today is the December 6th but I'm working on day 5.
I'm falling behind.
Part 1 took a while and is probably inefficient (it creates a grid like the example);

Uh oh, I just finished Day 5 Part 2 and now it's December 7th.
No time to clean it up, must press on 😅.

### Day 6

Day 6, which I did on December 7th, was easier for me.
The key insight is to model the lifecycle of the blowfish in a vector of size 9.

### Day 7

Yay I did day 7 on December 7 so I am caught up.

At first, I thought that we needed only return the *position* that was best to align the crab submarines on.
For this, I thought there might be some optimization utilizing mode and a tiebreaker using the average.
Then I got wrong answers because I was supposed to be returning the fuel cost.
Anyway, my solution is brute force, though I did at least look up the formula for triangular numbers (my CPU was able to brute force that as well, but it took a while).

### Day 8

Ugh. Day 8 kicked my ass.
I'll have to decide tomorrow whether to try and continue or not.

### Day 9

Much easier than day 8.
I think my answer is O(n).
Though I touch each cell multiple times, there is a constant maximum number of times that I can interact with each cell.

### Day 10

Fairly easy, and I actually did it on December 9th, yay.

### Day 11

It took me a while, as usual, but the flashing octopus problem was not hard.

### Day 12

I've fallen behind again, it's the 13th, and I finished day 12.
Graph with depth first search.
Always takes me forever to get graph DFS right.

### Day 13

Having fallen behind, I've only done the input parsing for day 13 on the 13th, but I need to call it a night.

...two days later...

Finished day 13 on the 15th. My answer looks like this:

```text
###..#....#..#.####...##.###....##.####.
#..#.#....#..#.#.......#.#..#....#.#....
###..#....####.###.....#.#..#....#.###..
#..#.#....#..#.#.......#.###.....#.#....
#..#.#....#..#.#....#..#.#....#..#.#....
###..####.#..#.#.....##..#.....##..#....
```

I entered `BLHFJPJF` which apparently is the right answer.
