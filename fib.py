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

# --------------------------------------------------------------------

import numpy as np
from numpy import array

def mfib(n, seq_fib=array([[1,1,],[1, 0]], dtype=np.ulonglong)):
    out_seq = np.array(seq_fib)
    _ = [out_seq.dot(seq_fib, out=out_seq) for i in range(1, n)]
    return out_seq[0,0]
