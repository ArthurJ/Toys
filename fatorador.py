from numpy import array, fromiter, append, \
    unique, sum, ulonglong, sqrt

dt_padrao = ulonglong
funcao_limitadora = sqrt


def calc_divisores(valor, limite_verificado):
    if limite_verificado % 2 == 0:
        limite_verificado += 1
    divisores = fromiter([i for i in range(limite_verificado, 1, -2) if valor % i == 0],
                         dtype=dt_padrao)
    if valor % 2 == 0:
        divisores = append(divisores, [2])
    # print(divisores)
    return divisores


def calc_divisores_primos(divisores):
    return unique(fromiter([i for x in divisores
                            for i in divisores
                            if x <= i
                            and sum(i % divisores == 0) == 1],
                           dtype=dt_padrao),
                  return_inverse=True)[0]


def fatora(v, divisores_primos):
    while v != 1:
        v_ini = dt_padrao(v)
        for d in divisores_primos:
            while v % d == 0:
                v /= d
                # print(d)
        if v_ini == v:
            divisores = calc_divisores(v_ini, int(funcao_limitadora(v_ini) + 1))
            divisores = append(divisores, v_ini)
            divisores = calc_divisores_primos(divisores)

            divisores_primos = append(divisores_primos, array(divisores,
                                      dtype=dt_padrao))

    return divisores_primos

if __name__ == '__main__':

    entrada = dt_padrao(input('Entre com o valor: '))
    # entrada = dt_padrao(7406 * (31 ** 2) * 1109)
    # entrada = dt_padrao(99991) * dt_padrao(99989) * \
    #   dt_padrao(23 ** 2) * dt_padrao(2 ** 3) * dt_padrao(7 ** 4)

    # Garante que o limite não explodirá para números muito grandes
    limite = funcao_limitadora(entrada)
    limite = int(limite) + 1

    # Encontro os primeiros fatores primos do valor do input
    # (por que é fácil e barato)
    divisores_calculados = calc_divisores(entrada, limite)
    divisores_primos_calculados = calc_divisores_primos(divisores_calculados)

    # Uso os fatores primos menores para encontrar os maiores
    # por fatoração simples
    divisores_primos_calculados = fatora(entrada, divisores_primos_calculados)
    divisores_primos_calculados.sort()

    print('\n\nValor de entrada:', entrada)
    print('Divisores primos:', divisores_primos_calculados)
    print('Maior divisor primo: ', divisores_primos_calculados[-1])
