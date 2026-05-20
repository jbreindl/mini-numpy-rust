import mini_numpy

vec = mini_numpy.PyVector([1, 2, 3])
print(vec)
print(vec + vec)
print(mini_numpy.PyVector([1.0, 2.0, 3.0]) + mini_numpy.PyVector([1.1, 2.2, 3.3]))

vec[0] = 10
print(vec)
