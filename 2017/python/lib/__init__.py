from os.path import split, join, realpath, splitext

from functools import reduce

def readinput(src_file, strip=True):
    full_path = realpath(src_file)
    src_dir, file_name = split(full_path)
    day, _ = splitext(file_name)

    with open(join(src_dir, '../inputs', day)) as f:
        data = f.read()
        return data.strip() if strip else data

def scan(f, seq, init):
    value = init
    yield value

    for x in seq:
        value = f(value, x)
        yield value

def apply_n(f, n, i):
    return reduce(lambda s, _: f(s), range(n), i)

# from https://en.wikipedia.org/wiki/Cycle_detection#Floyd.27s_Tortoise_and_Hare
def floyd(f, x0):
    tortoise = f(x0)
    hare = f(f(x0))
    while tortoise != hare:
        tortoise = f(tortoise)
        hare = f(f(hare))

    mu = 0
    tortoise = x0
    while tortoise != hare:
        tortoise = f(tortoise)
        hare = f(hare)
        mu += 1

    lam = 1
    hare = f(tortoise)
    while tortoise != hare:
        hare = f(hare)
        lam += 1

    return lam, mu
