# mysterious claude print file
import mini_numpy as mn


def run(label, fn, *args):
    try:
        result = fn(*args)
        print(f"{label:<28} -> {result!r}")
    except Exception as e:
        print(f"{label:<28} !! {type(e).__name__}: {e}")


print("--- arange ---")
run("arange(5)", mn.arange, 5)
run("arange(2, 6)", mn.arange, 2, 6)
run("arange(0, 10, 3)", mn.arange, 0, 10, 3)
run("arange(5, 0, -2)", mn.arange, 5, 0, -2)
run("arange(5, 5)", mn.arange, 5, 5)  # TODO: empty arrays don't work
run("arange(0, 10, 0)", mn.arange, 0, 10, 0)

print("\n--- ones ---")
run("ones([3])", mn.ones, [3])
run("ones([2, 3])", mn.ones, [2, 3])
run("ones((2, 2))", mn.ones, (2, 2))
run("ones([0])", mn.ones, [0])
run("ones([-1])", mn.ones, [-1])
run("ones(3)", mn.ones, 3)
run("ones([1.5])", mn.ones, [1.5])

print("\n--- zeros ---")
run("zeros([3])", mn.zeros, [3])
run("zeros([2, 3])", mn.zeros, [2, 3])
run("zeros([0])", mn.zeros, [0])
run("zeros([-1])", mn.zeros, [-1])

print("\n--- full ---")
run("full(7, [2, 2])", mn.full, 7, [2, 2])
run("full(2.5, [3])", mn.full, 2.5, [3])
run("full(-3, [2])", mn.full, -3, [2])
run("full(True, [2])", mn.full, True, [2])
run("full(2**40, [1])", mn.full, 2**40, [1])
run("full('a', [2])", mn.full, "a", [2])
run("full(None, [2])", mn.full, None, [2])
run("full(1, [-2])", mn.full, 1, [-2])
run("full(1, 'ab')", mn.full, 1, "ab")
