from functools import reduce
from pathlib import Path


def parse_input(pinput: str) -> tuple[list[list[int]], list[str]]:
    lists = pinput.splitlines()
    number_lists = lists[:-1]
    ops = lists[-1].split()
    numbers: list[list[int]] = [
        [int(i) for i in number_list.split()] for number_list in number_lists
    ]

    return numbers, ops


def part_1(pinput: str) -> int:
    sum = 0
    number_lists, ops = parse_input(pinput)

    for i, op in enumerate(ops):
        match op:
            case "*":
                sum += reduce(lambda x, y: x * y, [l[i] for l in number_lists])
            case "+":
                sum += reduce(lambda x, y: x + y, [l[i] for l in number_lists])
            case _:
                raise Exception

    return sum


def part_2(pinput: str) -> int:
    sum = 0
    lines = pinput.splitlines()
    t_lines: list[str] = []
    for x in range(len(lines[0]) - 1, -1, -1):
        t_lines.append("".join(l[x] for l in lines))

    numbers: list[int] = []
    for line in t_lines:
        if line.strip() == "":
            continue
        numbers.append(int(line[:-1]))
        match line[-1]:
            case "*":
                sum += reduce(lambda x, y: x * y, numbers)
                numbers = []
            case "+":
                sum += reduce(lambda x, y: x + y, numbers)
                numbers = []
            case _:
                pass

    return sum


def main():
    print("Part 1: ", part_1(Path("../input1.txt").read_text()))
    print("Part 2: ", part_2(Path("../input1.txt").read_text()))


if __name__ == "__main__":
    main()
