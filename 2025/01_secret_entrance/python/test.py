from pathlib import Path

from main import calculate_code, calculate_code2, count_zeros2


def test_1() -> None:
    assert 3 == calculate_code(Path("../test_input1.txt"))


def test_2() -> None:
    assert 6 == calculate_code2(Path("../test_input1.txt"))


def test_2_big_rot() -> None:
    assert 10 == count_zeros2([-1000])
    assert 10 == count_zeros2([1000])
