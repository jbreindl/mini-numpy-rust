import mini_numpy
import pytest


def test_vector_add():
    new_vector = mini_numpy.PyVector([1, 2, 3]) + mini_numpy.PyVector([1, 2, 3])
    assert new_vector == mini_numpy.PyVector([2, 4, 6])


def test_vector_sub():
    new_vector = mini_numpy.PyVector([1, 2, 3]) - mini_numpy.PyVector([1, 2, 3])
    assert new_vector == mini_numpy.PyVector([0, 0, 0])
