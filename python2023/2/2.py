from functools import reduce
from typing import Any, Iterator
from pathlib import Path


example = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

def construct(string: str) -> Iterator[Any]:
    for i, g in enumerate(string.split("\n")):
        if not g:
            continue
        game = g.split(":")[1].split(";")
        yield i+1, [[(int(ball.split(" ")[1]),ball.split(" ")[2])for ball in balls.split(",")] for balls in game]



def summer(string: str) -> int:
    sum = 0
    for game_num, game in construct(string):
        valid = True
        for hand in game:
            for count, color in hand:
                if count > {'red': 12, 'blue': 14, 'green': 13}[color]:
                    valid = False
        if valid:
            sum += game_num
    return sum


def powerer(string: str) -> int:
    sum = 0
    for game_num, game in construct(string):
        valid = True
        maximums = {}
        for hand in game:
            for count, color in hand:
                if color not in maximums or maximums[color] < count:
                    maximums[color] = count
        if valid:
            sum += reduce(lambda x, y: x * y, maximums.values(), 1)
    return sum

print(summer(example))
print(powerer(example))
print(summer((Path(__file__).parent / "data").read_text()))
print(powerer((Path(__file__).parent / "data").read_text()))


