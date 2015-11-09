from utils import time_it
import sys

primes = [2, 3, 5, 7, 11, 13, 17, 19]


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
    ultimo = 0
    for i in prime_list:
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


@time_it()
def next_prime(n=1, prime_list=primes,  file=sys.stdout):
    """
    :param n: Number of next primes to generate, default=1
    :param prime_list: Initial list of primes
    :param file: Print primes in this file, default=sys.stout

    Generate n primes, prints it, and stores it in the primes list
    """
    last = max(prime_list)
    while True:
        last += 2
        if is_prime(last, prime_list=prime_list):
            n -= 1
            prime_list.append(last)
            print(last, file=file)
            if n <= 0:
                break


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
