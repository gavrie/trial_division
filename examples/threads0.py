import time

from trial_division import trial_division


def calculate(start, end):
    for n in range(start, end):
        print(trial_division(n))


if __name__ == '__main__':
    start_time = time.time()

    n0 = 123456789
    count = 8

    calculate(n0, n0 + count)

    duration = time.time() - start_time
    print(f"Took: {duration:.3} seconds")
