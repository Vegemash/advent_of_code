from pathlib import Path


def part_1(pinput: str) -> int:
    sum = 0
    grid: list[list[bool]] = []
    for in_row in pinput.splitlines():
        row: list[bool] = []

        for cell in in_row:
            row.append(cell == "@")
        grid.append(row)

    for y, row in enumerate(grid):
        for x, cell in enumerate(row):
            if not cell:
                continue
            adjacent_rolls = 0
            for ya in range(max(0, y - 1), min(y + 2, len(grid))):
                for xa in range(max(0, x - 1), min(x + 2, len(row))):
                    if ya == y and xa == x:
                        continue
                    if grid[ya][xa]:
                        adjacent_rolls += 1
            if adjacent_rolls < 4:
                sum += 1

    return sum


def part_2(pinput: str) -> int:
    sum = 0
    old_sum = -1
    grid: list[list[bool]] = []
    for in_row in pinput.splitlines():
        row: list[bool] = []

        for cell in in_row:
            row.append(cell == "@")
        grid.append(row)

    while old_sum != sum:
        old_sum = sum
        for y, row in enumerate(grid):
            for x, cell in enumerate(row):
                if not cell:
                    continue
                adjacent_rolls = 0
                for ya in range(max(0, y - 1), min(y + 2, len(grid))):
                    for xa in range(max(0, x - 1), min(x + 2, len(row))):
                        if ya == y and xa == x:
                            continue
                        if grid[ya][xa]:
                            adjacent_rolls += 1
                if adjacent_rolls < 4:
                    grid[y][x] = False
                    sum += 1

    return sum


def main():
    print("Part 1: ", part_1(Path("../input1.txt").read_text()))
    print("Part 2: ", part_2(Path("../input1.txt").read_text()))


if __name__ == "__main__":
    main()
