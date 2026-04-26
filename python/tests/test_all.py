import pytest
import mini_numpy


def test_sum_as_string():
    assert mini_numpy.sum_as_string(1, 1) == "2"
