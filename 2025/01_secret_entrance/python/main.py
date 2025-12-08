from pathlib import Path


def parse_doc(doc: str) -> list[int]:
    moves: list[int] = []
    for line in doc.splitlines():
        dir: int = -1 if line[0] == "L" else 1
        moves.append(dir * int(line[1:]))
    return moves


def count_zeros(moves: list[int]) -> int:
    pos = 50
    zero_count = 0
    for move in moves:
        pos += move
        pos %= 100
        if pos == 0:
            zero_count += 1
    return zero_count


def calculate_code(file: Path) -> int:
    return count_zeros(parse_doc(file.read_text()))


def count_zeros2(moves: list[int]) -> int:
    pos = 50
    zero_count = 0
    for move in moves:
        for _ in range(abs(move)):
            assert move != 0
            if move > 0:
                pos += 1
                if pos == 100:
                    pos = 0
            if move < 0:
                pos -= 1
                if pos == -1:
                    pos = 99
            if pos == 0:
                zero_count += 1
    return zero_count


def calculate_code2(file: Path) -> int:
    return count_zeros2(parse_doc(file.read_text()))


def main():
    print("first method: ", calculate_code(Path("../input1.txt")))
    print("second method: ", calculate_code2(Path("../input1.txt")))


if __name__ == "__main__":
    main()
