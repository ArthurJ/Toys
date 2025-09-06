import itertools
from itertools import product
from sys import stdout


''' Avaliar probabilidades de vitória em somas de rolagens de conjuntos de dados de jogadores competindo entre si.'''


def ifprint(*args, should_doit=True,**kwargs):
    if should_doit:
        print(*args, **kwargs)


def matches(rolagens_desafiante=1, rolagens_desafiado=1, lados_desafiante=6,
            lados_desafiado=6, modificador=0, print_rolagens=False, saida=stdout):

    desafiante = product([i+1 for i in range(lados_desafiante)],
                        repeat=rolagens_desafiante)
    desafiado = product([i+1 for i in range(lados_desafiado)],
                        repeat=rolagens_desafiado)

    resultados = product(desafiante, desafiado)
    vitorias, derrotas, empates = 0, 0, 0
    for i, j in resultados:
        ifprint(i, j, file=saida, end='; ', should_doit=print_rolagens)
        i, j = sum(i)+modificador, sum(j)
        if i > j:
            ifprint('Vitória!', file=saida, should_doit=print_rolagens)
            vitorias += 1
        elif i < j:
            ifprint('Derrota!', file=saida, should_doit=print_rolagens)
            derrotas += 1
        else:
            ifprint('Empate.', file=saida, should_doit=print_rolagens)
            empates += 1

        ifprint('\nVitórias:', vitorias, '\nEmpates:', empates, '\nDerrotas:',
            derrotas, file=saida , should_doit=print_rolagens)
    return (vitorias, empates, derrotas)


if __name__ == '__main__':

    with open('resultados.txt','w') as log:

        def logprint(*args, **kwargs):
            print(*args, **kwargs, file=log)

        def combine (*args):
            return itertools.combinations_with_replacement(*args)

        contabilizar_empates = True
        for i, j in product([k+1 for k in range(5)], repeat=2):
        #for i, j in combine([k+1 for k in range(10)], 2):

            logprint('\t\t', '*'*5, '\t\tDesafiante x Desafiado\t\t', '*'*5)

            vitorias, empates, derrotas = matches(i, j, print_rolagens=False)
            chance_vitoria = vitorias/(vitorias + derrotas +
                                        (empates if contabilizar_empates else 0))
            chance_empate = empates/(vitorias + derrotas + empates)

            logprint('Rolagens:\t', i, 'D4 x ', j, 'D4', sep='')
            logprint('Número de vitórias:', vitorias)
            logprint('Número de derrotas:', derrotas)
            logprint('Número de empates:', empates)
            logprint('Probabilidade de Derrota:\t', '{:.3%}'.format(1-chance_vitoria-chance_empate))

            if contabilizar_empates: logprint('Probabilidade de Empate:\t',
                                              '{:.3%}'.format(chance_empate))

            logprint('Probabilidade de Vitória:\t', '{:.3%}'.format(chance_vitoria))

            logprint('-'*80, '\n')
