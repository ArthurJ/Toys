from utils import time_it
import sys

primes = [2, 3]


class PrimeListTooShortException(Exception):
    def __init__(self):
        self.msg = 'Prime list is not long enough to test this number'


def is_prime(p, prime_list=primes):
    """
    :param p: Number to be tested
    :param prime_list: lista de primos a usar como base
    :return: True if the number is prime, False otherwise

    >>> is_prime(1811)
    True
    >>> is_prime(1813)
    False
    """
    if p in prime_list:
        return True

    limite_de_teste = root_limit(p) 
    for i in prime_list:
        if p % i == 0:
            return False
        if i > limite_de_teste:
            return True
    raise PrimeListTooShortException()


def root_limit(p):
    return int(p**.5)

#@time_it()
def next_prime(prime_list=primes):
    if not prime_list:
        prime_list = [2, 3]
    last = prime_list[-1]
    while True:
        last += 2
        if is_prime(last, prime_list):
            prime_list.append(last)
            return last
        


def print_primes(qtd):
    [print(f"{next_prime(primes)}, ","\n" if i%20==0 else " ", end='') 
     for i in range(qtd)]


if __name__=='__main__':
    print_primes(50_000)