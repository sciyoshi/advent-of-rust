# Day 1: Inverse Captcha

[Problem description](https://adventofcode.com/2017/day/1) | [Rust solution](./mod.rs)

## Quick Python Solution

```python
input = sys.stdin.readline().strip()
chars = len(input)

captcha1 = sum(int(c) for i, c in enumerate(input) if c == input[(i + 1) % chars])
captcha2 = sum(int(c) for i, c in enumerate(input) if c == input[(i + chars // 2) % chars])
```
