# Day 1: Inverse Captcha

[Problem Description](https://adventofcode.com/2017/day/1)

## Quick Python Solution

```
input = sys.stdin.readline().strip()
chars = len(input)

captcha1 = sum(int(c) for i, c in enumerate(input) if c == input[(i + 1) % chars])
captcha2 = sum(int(c) for i, c in enumerate(input) if c == input[(i + chars // 2) % chars])
```
