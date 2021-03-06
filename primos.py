from utils import time_it
import sys

primes = [2, 3, 5, 7, 11, 13, 17, 19]


class PrimeListTooShortException(Exception):
    def __init__(self):
        self.msg = 'Prime list is not long enougth to test this number'


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

    limite_de_teste = int(p**.5)
    for i in prime_list:
        if p % i == 0:
            return False
        if i > limite_de_teste:
            return True
    raise PrimeListTooShortException()


@time_it()
def next_prime(n=1, prime_list=primes,  file=sys.stdout, cache_limit=False, start_ahead=False):
    """
    :param n: Number of next primes to generate, default=1
    :param prime_list: Initial list of primes, minimum list must be [2, 3]
    :param file: Print primes in this file, default=sys.stout
    :param cache_limit: Superior limit number to store in the prime_list
    :param start_ahead: Seek for primes starting in this value

    Generate n primes, prints it, and stores it in the primes list
    """
    if not start_ahead:
        last = max(prime_list)
    else:
        if start_ahead % 2 == 0:
            last = start_ahead - 1
        else:
            last = start_ahead - 2
    try:
        while True:
            last += 2
            if is_prime(last, prime_list=prime_list):
                n -= 1
                print(last, file=file)
                if not cache_limit or (cache_limit and last < cache_limit):
                    prime_list.append(last)
                if n <= 0:
                    break
    except PrimeListTooShortException:
        print('Too limited cache to continue!', file=sys.stderr)


@time_it()
def crivo(lim_superior, file=sys.stdout):
    """
    :param lim_superior: Maior numero na lista a ser usada no crivo
    :param file: Arquivo onde serão escritos os valores descobertos,
        default=sys.stout
    :return: lista de primos descobertos

    Essa implementação é mais lenta que o next_prime() para encontrar a
        mesma quantidade de números primos.
    """
    # TODO reimplementar
    candidatos = range(1, lim_superior)
    primialidade = [True]*lim_superior
    primialidade[0] = False

    for i, j in enumerate(candidatos):
        if primialidade[i]:
            for k, l in enumerate(candidatos[i+1:]):
                if primialidade[k+i+1] and l % j == 0:
                    primialidade[k+i+1] = False

    primos_descobertos = []
    for i, j in enumerate(candidatos):
        if primialidade[i]:
            primos_descobertos.append(j)
            print(j, file=file)

    return primos_descobertos
