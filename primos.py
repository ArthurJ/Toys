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
            break

# #TODO
# #Implementar: Crivo de Erastostenes
# #
# #Exige ter um limite estabelecido para o usuario
# #implementando o crivo, trabalharia com duas listas, uma com os numeros e outra com boleanos indicando a validade
# #dos valores da primeira lista, e depois filtraria os valores da primeira com base na segunda.
# #
# #Deixar como opção no crivo um pré-teste para eliminar valores divisiveis por uma lista vinda de arquivo,
# #assim dá pra filtrar primos de blocos de números separadamente:
# #    0-99, 100-199 (usando tb os primos encontrados entre 0 e 99 salvos previamente), e assim por diante.
# #isso deve permitir algum paralelismo!
