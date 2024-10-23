[![Run tests](https://github.com/gchazot/aoc/actions/workflows/run_tests.yml/badge.svg)](https://github.com/gchazot/aoc/actions/workflows/run_tests.yml)

# Advent of code solutions

These are my solutions to the amazing [Advent Of Code](https://adventofcode.com/) coding challenge.

This whole thing is a playground to exercise my coding skills and knowledge of Python and development tools, in the spirit of Craftsmanship.

I had initially set myself a few rules for this challenge that I have mostly respected:
1. only use the standard Python Library
2. write code compatible with Python 2.7 and 3.7 (2016-2019), then 3.10 (2020+)
3. develop using TDD techniques and Clean Code principles

Well... #2 does not make much sense anymore, especially the Python 2 thing. However, I'm still sticking to #1 and #3... at least to #3. The thing is that in 2023, I decided to give a try at Rust language. I'm sticking to the idea though and the "standard Python library" became the "standard Rust library". In order to make all of this manageable, I've written the [list_test_combinations.sh](list_test_combinations.sh) script that deduces from the content of each folder the language used and the versions supported.
