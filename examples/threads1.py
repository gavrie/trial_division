import time
import threading

# from trial_division import trial_division
from primes import trial_division
# from primes import trial_division_no_gil as trial_division


def calculate(i, start, end):
    for n in range(start, end):
        print(f"Thread {i}: {trial_division(n)}")


if __name__ == '__main__':
    start_time = time.time()

    n0 = 123456789
    count = 200

    num_threads = 8
    count_per_thread = count // num_threads

    threads = []
    for i in range(num_threads):
        start = n0 + i * count_per_thread
        end = start + count_per_thread

        t = threading.Thread(target=calculate, args=(i, start, end))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    duration = time.time() - start_time
    print(f"Took: {duration:.3} seconds")
