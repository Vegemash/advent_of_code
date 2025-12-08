from pathlib import Path


def part_1(pinput: str) -> int:
    sum = 0
    for digits in pinput.splitlines():
        left = 0
        left_index = 0
        right = 0
        for i, d in enumerate(digits[:-1]):
            if int(d) > left:
                left = int(d)
                left_index = i
        for d in digits[left_index + 1 :]:
            if int(d) > right:
                right = int(d)
        sum += int(str(left) + str(right))

    return sum


def part_2(pinput: str) -> int:
    sum = 0
    for line in pinput.splitlines():
        digits: list[int] = []
        rightmost_index = 0
        for place in range(12):
            digit = -1
            for i in range(rightmost_index, len(line) - (12 - place) + 1):
                if int(line[i]) > digit:
                    digit = int(line[i])
                    rightmost_index = i
            digits.append(int(line[rightmost_index]))
            rightmost_index += 1
        joltage = int("".join(str(i) for i in digits))
        sum += joltage

    return sum


def main() -> None:
    print("Part 1: ", part_1(Path("../input1.txt").read_text()))
    print("Part 2: ", part_2(Path("../input1.txt").read_text()))


if __name__ == "__main__":
    main()
