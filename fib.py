__inicial = {1: 1, 2: 1, 3: 2}


def fib(n, descartar_anteriores=False, seq_fib=__inicial):
    if n in seq_fib:
        return seq_fib[n]
    elif descartar_anteriores:
        while n > max(seq_fib):
            seq_fib[max(seq_fib) + 1] = \
                seq_fib[(max(seq_fib))] + seq_fib[(max(seq_fib) - 1)]
            del seq_fib[max(seq_fib) - 3]
    else:
        while n > max(seq_fib):
            seq_fib[max(seq_fib) + 1] = \
                seq_fib[(max(seq_fib))] + seq_fib[(max(seq_fib) - 1)]
    return seq_fib[n]


# TODO implementar matricial
