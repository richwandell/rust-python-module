from wandell_rust import quick_sort
import random

data = list(random.randrange(1, 100) for i in range(100))
print(data)

def sort_quick(data):
    return quick_sort(data)


def sort_python(data):
    return sorted(data)


if __name__ == "__main__":
    import timeit
    print("Rust Quick Sort: " + str(timeit.timeit('sort_quick(data)', globals=globals())))
    print("Python Sorted function: " + str(timeit.timeit('sort_python(data)', globals=globals())))


