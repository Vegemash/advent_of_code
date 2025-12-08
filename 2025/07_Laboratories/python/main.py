from pathlib import Path


def parse_input(pinput: str) -> tuple[int, list[list[int]]]:
    lines: list[str] = pinput.splitlines()
    start_pos = 0
    for i, c in enumerate(lines[0]):
        if c == "S":
            start_pos = i
            break
    splitter_lines: list[list[int]] = []
    for line in lines[1:]:
        splitters: list[int] = []
        for i, c in enumerate(line):
            if c == "^":
                splitters.append(i)
        splitter_lines.append(splitters)

    return start_pos, splitter_lines


def part_1(pinput: str) -> int:
    sum = 0
    start, lines = parse_input(pinput)
    beams: set[int] = {start}
    for splitters in lines:
        new_beams = beams.copy()
        for beam in beams:
            if beam in splitters:
                new_beams.remove(beam)
                new_beams.add(beam - 1)
                new_beams.add(beam + 1)
                sum += 1
        beams = new_beams

    return sum


def part_2(pinput: str) -> int:
    start, lines = parse_input(pinput)

    # forward pass collecting which splitters are active
    beams: dict[int, int] = {start: 1}
    for splitters in lines:
        new_beams = beams.copy()
        for beam, timelines in beams.items():
            if beam in splitters:
                new_beams[beam - 1] = new_beams.setdefault(beam - 1, 0) + timelines
                new_beams[beam + 1] = new_beams.setdefault(beam + 1, 0) + timelines
                del new_beams[beam]
        beams = new_beams.copy()

    return sum(beams.values())


def main():
    print("Part 1: ", part_1(Path("../input1.txt").read_text()))
    print("Part 2: ", part_2(Path("../input1.txt").read_text()))


if __name__ == "__main__":
    main()
