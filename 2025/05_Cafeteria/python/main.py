from pathlib import Path


def parse_input(pinput: str) -> tuple[list[tuple[int, int]], list[int]]:
    ranges_str, ids_str = pinput.split("\n\n", maxsplit=1)
    ranges: list[tuple[int, int]] = []
    for range_str in ranges_str.splitlines():
        a, b = range_str.split("-", maxsplit=1)
        ranges.append((int(a), int(b)))
    ids = [int(i) for i in ids_str.splitlines()]
    return ranges, ids


def part_1(pinput: str) -> int:
    sum = 0
    ranges, ids = parse_input(pinput)
    for id in ids:
        for low, high in ranges:
            if id >= low and id <= high:
                sum += 1
                break

    return sum


def part_2(pinput: str) -> int:
    sum = 0
    ranges, _ = parse_input(pinput)
    high_water = 0
    for low, high in sorted(ranges):
        if high <= high_water:
            continue
        low = max(low, high_water + 1)
        sum += high - low + 1
        high_water = high

    return sum


def main():
    print("Part 1: ", part_1(Path("../input1.txt").read_text()))
    print("Part 2: ", part_2(Path("../input1.txt").read_text()))


if __name__ == "__main__":
    main()
