def trial_division(n):
    """Return a list of the prime factors for a natural number."""
    a = []
    f = 2
    while n > 1:
        if n % f == 0:
            a.append(f)
            n /= f
        else:
            f += 1
    return a
