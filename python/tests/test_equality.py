import pytest
from mini_numpy import PyVector


def test_should_equal():
    assert PyVector([1, 2, 3]) == PyVector([1, 2, 3])


def test_vector_different_length():
    assert PyVector([1, 2, 3]) != PyVector([1, 2])


def test_vector_scalar():
    assert PyVector([1, 2, 3]) != 1


def test_different_vector():
    assert PyVector([1, 2, 3]) != PyVector([3, 2, 1])


def test_invalid_type():
    with pytest.raises(TypeError):
        PyVector([1.0, 2.0, 3.0]) == PyVector([1.0, 2.0, 3.0])
