from mini_numpy import MyVector


def test_should_equal():
    assert MyVector([1, 2, 3]) == MyVector([1, 2, 3])


def test_vector_different_length():
    assert MyVector([1, 2, 3]) != MyVector([1, 2])


def test_vector_scalar():
    assert MyVector([1, 2, 3]) != 1


def test_different_vector():
    assert MyVector([1, 2, 3]) != MyVector([3, 2, 1])
