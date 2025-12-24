from pathlib import Path

from main import part_1


def test_1() -> None:
    assert 40 == part_1(Path("../test_input1.txt").read_text(), 10)


# def test_2() -> None:
#     assert 40 == part_2(Path("../test_input1.txt").read_text())
