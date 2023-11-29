import pytest


def test_01_unit():
    assert True


def test_02_unit():
    assert True


def test_03_unit():
    assert True


def test_04_unit():
    assert True


def test_05_unit():
    assert True


def test_06_unit():
    assert False


@pytest.mark.skip(reason="no way of currently testing this")
def test_07_unit():
    assert False
