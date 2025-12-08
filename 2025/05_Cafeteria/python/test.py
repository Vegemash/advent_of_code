from pathlib import Path

from main import part_1, part_2


def test_1() -> None:
    assert 3 == part_1(Path("../test_input1.txt").read_text())


def test_2() -> None:
    assert 14 == part_2(Path("../test_input1.txt").read_text())
