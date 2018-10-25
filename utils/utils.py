from typing import Dict, Any, NamedTuple

import os
import math
import heapq
import requests

__all__ = ['inp']


COOKIES = {
	'session': '53616c7465645f5f578fe8241d3d3ecb14f7131063bb1c65d383f6983cc4d9b4ec05f214017bb153fa2cb60acf5a18be'
}


def inp(day: int, year: int = 2017):
	path = os.path.join(os.path.dirname(__file__), 'inputs', f'day{day}')
	url = f'https://adventofcode.com/{year}/day/{day}/input'

	if os.path.exists(path):
		return open(path).read()

	data = requests.get(url, cookies=COOKIES)

	if data.status_code != 200:
		raise Exception(data.text)

	with open(path, 'w') as f:
		f.write(data.text)

	return data.text


class Pt(NamedTuple('Pt', [('x', int), ('y', int)])):
	def __add__(self, other):
		return type(self)(self.x + other.x, self.y + other.y)

	def __sub__(self, other):
		return type(self)(self.x - other.x, self.y - other.y)

	def __mul__(self, other):
		return type(self)(self.x * other, self.y * other)

	def __div__(self, other):
		return type(self)(self.x / other, self.y / other)

	def __floordiv__(self, other):
		return type(self)(self.x // other, self.y // other)

	def __neg__(self):
		return type(self)(-self.x, -self.y)

	@property
	def rot90r(self):
		return type(self)(self.y, -self.x)

	@property
	def rot90l(self):
		return type(self)(-self.y, self.x)

	@property
	def norm1(self):
		return abs(self.x) + abs(self.y)

	@property
	def normi(self):
		return max(abs(self.x), abs(self.y))

	@property
	def norm2(self):
		return math.sqrt(self.x ** 2 + self.y ** 2)

	@property
	def nb4(self):
		return [self + d for d in [Pt.N, Pt.E, Pt.S, Pt.W]]

	@property
	def nb8(self):
		return [self + d for d in [Pt.N, Pt.NE, Pt.E, Pt.SE, Pt.S, Pt.SW, Pt.W, Pt.NW]]


Pt.N = Pt(0, 1)
Pt.NE = Pt(1, 1)
Pt.E = Pt(1, 0)
Pt.SE = Pt(1, -1)
Pt.S = Pt(0, -1)
Pt.SW = Pt(-1, -1)
Pt.W = Pt(-1, 0)
Pt.NW = Pt(-1, 1)


class Grd(Dict[Pt, Any]):
	def __init__(self, w=None, h=None):
		if w is not None and h is not None:
			for x in range(w):
				for y in range(h):
					self[Pt(x, y)] = None

	def nb4(self, point):
		for pt in point.nb4:
			if pt in self:
				yield pt

	def nb8(self, point):
		for pt in point.nb8:
			if pt in self:
				yield pt


def astar_search(start, h_func, moves_func):
	# A priority queue, ordered by path length, f = g + h
	frontier = [(h_func(start), start)]
	# start state has no previous state; other states will
	previous = {start: None}
	# The cost of the best path to a state.
	path_cost = {start: 0}
	while frontier:
		(f, s) = heapq.heappop(frontier)
		if h_func(s) == 0:
			return Path(previous, s)
		for s2 in moves_func(s):
			new_cost = path_cost[s] + 1
			if s2 not in path_cost or new_cost < path_cost[s2]:
				heapq.heappush(frontier, (new_cost + h_func(s2), s2))
				path_cost[s2] = new_cost
				previous[s2] = s
	return dict(fail=True, front=len(frontier), prev=len(previous))


def Path(previous, s):
	return ([] if (s is None) else Path(previous, previous[s]) + [s])
