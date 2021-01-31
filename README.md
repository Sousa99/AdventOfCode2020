# Advent Of Code - 2020
Advent of code 1st attempt (2020) - https://adventofcode.com/

---
## Motivation 🚂
There was always the desire to experiment with new coding languages that I have been hearing about, but I always found it difficult to have the creativity to give myself projects that would lead to a better learning of these languages... Lets just say **creativity** is not my strong  suit.

I have always loved this kind of challenges, the challenges is not in terms of the programming complexity necessarily but more on the abstraction needed to solve the problem well and in a reasonable time. And believe me when I say that some of the problems were hard for me to understand how to improve the methods used to achieve the result in a reasonable amount of time.

But like I was saying, I wanted to learn new languages, so I decided to learn **Rust**.

---

## Why Rust 🦀
To be fair the first time I learned of Rust or even heard of it was in discord community of my course. What made me more curious about this specific language is how they said that it made sure during compile time that its `execution would be safe`. This paradigm of making sure that there were no unexpected cases surely perplexed me I wanted to give it a look by myself.

---

## Methodology ✍
Just like any other language I like to start of straight into the practical side of it, try to use and when I it a roadblock (which could be as simple as not knowing how to do a for loop), search for it on the internet, typically this works out fine and I am able to learn it without focusing too much on the small details which change absolutely nothing.

I must admit that `Rust did not work like that`. Yes sure, I think I got through the first challenge by doing this, but I quickly understood that there some structures (like `Result`, `Option` and `macros`) that kept appearing and I could deal with them but did not know exactly what they meant behind the scenes.

So a change in the methodology was required in order to give the proper opportunity to the language. So I decided to follow the [book of Rust][Rust book], and boy if I should have started by doing that. Typically documents such as this I tend to use and consult when needed, but this one is particularly so  well written and with such a good progression on information.

In terms of the resolution of the challenges themselves, there was no particular methodology, since most challenges had a clear and concrete objective I only committed code when it was properly working. Since the whole point of the `Advent of Code` is doing one each day, I tried to follow this strategy, although having started it a long time after 1st of December. Most of the times I was successful on delivering one a day, sometimes even more than one, but there were a few ones that definitely made me scratch my head.

---

## Things to keep  in  mind next time 🙊
Next time I try to complete the Advent of Code, I would like to change a few things:

1. Try to complete them on the day they are supposed to be solved. If I get stuck then the next day try to solve the next one, the solution or at least a different approach will come to mind, there is no point in getting stuck.
2. Maybe even create a private scoreboard with a few people. The sense of competition tends to motivate work, as long as this competition is friendly (and local because of time zones).
3. Try to write a bit about the challenge and what I felt about it. What was the hardest, easiest, the challenge, how I overcame, etc. Most of the challenges are really interesting and lead to good observations and conclusions, but most of them leave my head soon after completing, so leaving it to the end is not a good choice.

---

## Most interesting challenges 💪

- ### [Day 13](https://adventofcode.com/2020/day/13)

This definitely was the first challenge where I hit a wall. To be fair it wasn't hard to solve the challenge, the problem was that I was trying to solve it in a `brute force` way.

Firstly I was testing out all the number possibilities with a simple while loop which was incremented. This was not definitely a good approach for long term. Then I made a simple change, instead of incrementing by 1 I incremented by the highest value registered, since the `least common multiple` would need to be at least greater than this.

To my surprise, still not good enough, I scratched my head for a while, but I was stuck, so ... I went looking for a hint, I found reference to a `Chinese Remainder Theorem`. I immediately stopped searching for hints and tried to remember how it worked (since I had learned it in a class) by looking at some papers which exemplified its use. I was still not to happy since my implementation would only work for prime number between themselves, but it server the purpose so I left it, but it should not be (too) hard to implement logic which would take this into consideration.

- ### [Day 17](https://adventofcode.com/2020/day/17)

Although it wasn't that hard to implement, I found it interesting because I intend to create some sort of animation in which this algorithm (each iteration) is visible.

- ### [Day 18](https://adventofcode.com/2020/day/18)

In principle it should not be hard to implement. I had a course on Compilers, and this included a deep knowledge of grammars and parsers. As soon as I saw the challenge I started searching for parsers crates for Rust. The main challenge that immediately came to mind was that ideally I would need a parser which rules are established on run-time. This wasn't hard to find and implement, and it worked straight away for the first part.

The second part used recursivity in grammar, which should already be tolerable by the parser, but apparently this type of parsers had a problem with consuming the end of the input.

So I abandoned the parser idea and created structs which would deal with priorities and solve them in orders. Sometimes the most direct solution is the best one (I guess).

- ### [Day 20](https://adventofcode.com/2020/day/20)

The idea was so interesting to me, the idea of a thing so simple or at least mundane to us *humans* but with no apparent strategy, or at least a strategy simple enough to implement through a simple algorithm.

In the end I wasn't too happy with my solution since it relied on randomness to remove pieces from the board when it was impossible to add anu more pieces. But ideally I should have implemented the possibility to move sets of pieces around, and remove only pieces on the ends and not randomly in the middle of the pieces already set.

Although interesting, I could not find the will to change something that worked *relatively* well into something that more than likely would take a lot of work to get it working well. `I should revisit this challenge in the future`.

---

## What comes next 🔮
My initial idea was to start of (when time presents itself, since now I should be focusing on exams) a new year of `Advent of Code`, probably 2019, with a new language. The one which I have in mind is [Go (Golang)][Go]. I have already messed a bit with it, but not to much, and it is a language that I have also heard quite a bit about.

But since Rust and this years' edition was so interesting, I would like to develop an animation of one of the challenges, inspired by one that I have seen when searching for a hint to another challenge. Currently I have found one crate, [Amethyst][Amethyst] which seems to allow for what I intend, but I need to develop further research.

---

## Acknowledgments 🤗

Such a fun assortment of challenges developed by [Eric Wastl][Eric Wastl Webpage]. What made these challenges so interesting was the progression in them. In the begining most of them are straight forward, easy enough to implement, more towards the end some of them are quite difficult, not of implementing but more of developing something that solves it in a reasonable amount of time and the puzzle that comes after one of these that are harder are typically quite easy which renovates the energy and will to solve more of them.

The easter eggs which are invisible are quite fun as well, thanks.


[Rust book]: https://doc.rust-lang.org/book/
[Go]: https://golang.org/
[Amethyst]: https://amethyst.rs/
[Eric Wastl Webpage]: http://was.tl/