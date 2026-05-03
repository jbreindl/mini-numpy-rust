import pytest
import mini_numpy


def test_vector_add():
    
    new_vector = mini_numpy.MyVector([1, 2, 3]) + mini_numpy.MyVector([1, 2, 3])
    assert new_vector == mini_numpy.MyVector([2, 4, 6])
