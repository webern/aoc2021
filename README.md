# Advent of Code 2021

This is the first time I have tried this.
I'm doing this in Rust.
Today is day 2, we'll see how long I can keep up.

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

At first I thought that we needed only return the *position* that was best to align the crab submarines on.
For this, I thought there might be some optimization utilizing mode and a tiebreaker using the average.
Then I got wrong answers because I was supposed to be returning the fuel cost.
Anyway, my solution is brute force, though I did at least look up the formula for triangular numbers (my CPU was able to brute force that as well, but it took a while).
