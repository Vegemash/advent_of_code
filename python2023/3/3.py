from pathlib import Path
import pytest


example = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""


def parser(string: str):
    numbers = []
    symbols = []
    for lno, line in enumerate(string.splitlines()):
        start = 0
        accumulation = ""
        for cno, char in enumerate(line):
            last_char = cno + 1 == len(line)
            digits = "1234567890"

            if char in digits:
                if not accumulation:
                    start = cno
                accumulation += char

            if (char in digits and last_char) or (accumulation and char not in digits):
                numbers.append(((start, cno, lno), int(accumulation)))
                accumulation = ""

            if char not in digits and char != ".":
                symbols.append((cno, lno))

    return numbers, symbols

def gear_parser(string: str):
    numbers = []
    gears = []
    for lno, line in enumerate(string.splitlines()):
        start = 0
        accumulation = ""
        for cno, char in enumerate(line):
            last_char = cno + 1 == len(line)
            digits = "1234567890"

            if char in digits:
                if not accumulation:
                    start = cno
                accumulation += char

            if (char in digits and last_char) or (accumulation and char not in digits):
                numbers.append(((start, cno, lno), int(accumulation)))
                accumulation = ""

            if char == "*":
                gears.append((cno, lno))

    return numbers, gears

def internal(symbols, c_x, c_xlast, c_y) -> bool:
    for sx, sy in symbols:
        for cx in range(c_x, c_xlast):
            if abs(cx - sx) < 2 and abs(c_y - sy) < 2:
                return True
    return False


def summer(numbers, symbols) -> int:
    sum = 0
    for coords, num in numbers:
        if internal(symbols, *coords):
            sum += num
    return sum


if __name__ == "__main__":
    print(summer(*parser(example)))
    print(summer(*parser((Path(__file__).parent / "data").read_text())))


@pytest.mark.parametrize(
    "string,expected",
    [
        ("123", 0),
        ("123.@", 0),
        ("123@", 123),
        ("123\n..@", 123),
    ],
)
def test_summer(string: str, expected: int) -> None:
    assert summer(*parser(string)) == expected
