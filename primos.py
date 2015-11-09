# from utils import time_it
import sys

primes = [2, 3, 5, 7, 11, 13, 17, 19]


def is_prime(p):
    """
    :param p: Number to be tested
    :return: True if the number is prime, False otherwise

    >>> is_prime(1811)
    True
    >>> is_prime(1813)
    False
    """
    if p in primes:
        return True

    limite_de_teste = int(p**.5)
    ultimo = 0
    for i in primes:
        if p % i == 0:
            return False
        if i > limite_de_teste:
            return True
        ultimo = i
    for i in range(ultimo, limite_de_teste, 2):
        if p % i == 0:
            return False
        if i > limite_de_teste:
            break
    return True


def next_prime(file=sys.stdout):
    last = max(primes)
    while True:
        last += 2
        if is_prime(last):
            primes.append(last)
            print(last, file=file)


# @time_it()
# def test_next_prime(iteracoes):
#     for i in range(iteracoes):
#         next_prime()
# test_next_prime(1000000)
