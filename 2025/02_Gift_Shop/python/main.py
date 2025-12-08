from pathlib import Path


def part_1(pinput: str) -> int:
    sum = 0
    for id_range in pinput.split(","):
        start, end = id_range.split("-")
        for part in range(int(start), int(end) + 1):
            str_part = str(part)
            plen = len(str_part)
            if len(str_part) % 2 != 0:
                continue
            if str_part[: plen // 2] == str_part[plen // 2 :]:
                sum += int(part)
    return sum


def part_2(pinput: str) -> int:
    sumation = 0
    for id_range in pinput.split(","):
        start, end = id_range.split("-")
        for part in range(int(start), int(end) + 1):
            str_part = str(part)
            for chunk_size in range(1, len(str_part) // 2 + 1):
                if len(str_part) % chunk_size != 0:
                    continue
                if all(a == "" for a in str_part.split(str_part[:chunk_size])):
                    sumation += int(str_part)
                    break
    return sumation


def main():
    print("Part 1: ", part_1(Path("../input1.txt").read_text()))
    print("Part 2: ", part_2(Path("../input1.txt").read_text()))


if __name__ == "__main__":
    main()
