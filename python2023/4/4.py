from pathlib import Path
example = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""


def summer(input: str) -> int:
    winnings = 0
    for line in input.split("\n"):
        if not line:
            continue
        _, numbers = line.split(": ")

        ours_str, winning_str = numbers.split(" | ")
        ours = {int(chars) for chars in ours_str.split(" ") if chars}
        winning = [int(chars) for chars in winning_str.split(" ") if chars]
        hit_count = sum(1 for winner in winning if winner in ours)
        if hit_count > 0:
            winnings += 2**(hit_count - 1)

    return winnings

def summer2(input: str) -> int:
    multipliers = {}
    lines = [l for l in input.split("\n") if l]
    for card_no, line in enumerate(lines):
        multipliers.setdefault(card_no, 1)
        if not line:
            continue
        _, numbers = line.split(": ")

        ours_str, winning_str = numbers.split(" | ")
        ours = {int(chars) for chars in ours_str.split(" ") if chars}
        winning = [int(chars) for chars in winning_str.split(" ") if chars]
        hit_count = sum(1 for winner in winning if winner in ours)

        print(line, list(w for w in winning if w in ours))
        if hit_count:
            print(f"adding to cards {card_no+2}-{card_no + hit_count+ 1}, {multipliers[card_no]} times from {card_no+1}")
            for i in range(card_no + 1, card_no + hit_count +1):
                multipliers.setdefault(i, 1)
                multipliers[i] += multipliers[card_no]


    # for card, count in multipliers.items():
    #     print(card + 1, count)
    return sum(multipliers.values())

if __name__ == "__main__":
    # print(summer(example))
    # print(summer((Path(__file__).parent / "data").read_text()))
    # print(summer2(example))
    print(summer2((Path(__file__).parent / "data").read_text()))
