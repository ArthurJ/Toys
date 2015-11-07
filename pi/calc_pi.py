import multiprocessing as mp
from time import sleep, time
from subprocess import call
from decimal import getcontext, ROUND_FLOOR, Decimal as decimal

'''
Created on 01/06/2011
'''
__author__ = '@arthurj'


def spi(inicio, fim, digitos):
    return sum(int(((4 << digitos) // (8 * n + 1) -
                    (2 << digitos) // (8 * n + 4) -
                    (1 << digitos) // (8 * n + 5) -
                    (1 << digitos) // (8 * n + 6)) // (16 ** n))
               for n in range(inicio, fim))


def separa_intervalos(ini, fim, num_de_partes, casas):
    shift = (fim - ini) // num_de_partes
    return [(ini + shift * i, shift * (i + 1), casas)
            for i in range(num_de_partes)]


def calc(args):
    global mypi
    mypi += spi(args[0], args[1], args[2])


def float_hex_2_float(str_pi_hex_digits, precisao) -> decimal:
    getcontext().prec = precisao
    getcontext().rounding = ROUND_FLOOR
    mypi10 = decimal(0)
    for i in range(len(str_pi_hex_digits)-1, -1, -1):
        mypi10 += decimal(int(str_pi_hex_digits[i], base=16) * 16 ** -i)
    return mypi10


if __name__ == '__main__':
    # Apresentação e preparação

    call(["clear"])
    print('Fui escrito para calcular os digitos de π em base hexadecimal.')
    print('Leve em consideração que o processador ' +
          'deste computador tem {} núcleo(s).\n'.format(mp.cpu_count()))

    itera = int(input('Número de Iterações à realizar? -> '))
    digitos = 4 * int(itera * 1.4)

    numero_de_partes_do_processo = int(input(
        'Dividir o processo em quantas partes? -> '))

    nomepi = input('\nQual o nome do arquivo de saída?\n' +
                   '[para "meupix", apenas aperte Enter] -> ')
    if nomepi == '':
        nomepi = "meupix"

    piComparado = input('\nQual o nome do arquivo com o qual comparar o resultado?\n' +
                        '[Apenas aperte Enter para tentar o arquivo "npix8k"] -> ')

    if piComparado != '':
        piComparado = open(piComparado, 'r')
        piComparado = piComparado.read()
    else:
        try:
            piComparado = open('npix8k', 'r')
            piComparado = piComparado.read()
        except:
            piComparado != ''

    # Processamento e resultados
    tempo = time()

    mypi = 0
    pool = mp.Pool(numero_de_partes_do_processo)

    intervalos = separa_intervalos(0, itera, numero_de_partes_do_processo,
                                   digitos)
    processos = [0] * numero_de_partes_do_processo

    for i in range(numero_de_partes_do_processo):
        processos[i] = mp.Process(target=calc, args=(intervalos[i],))
        processos[i].start()
        processos[i].run()

    k = 0
    while numero_de_partes_do_processo > 0:
        k = k % numero_de_partes_do_processo
        if not processos[k].is_alive():
            numero_de_partes_do_processo -= 1
        else:
            sleep(.01)
        k += 1
    print('\n')
    print('Valor Hexadecimal: {:x}'.format(mypi))
    mypi = '{:x}'.format(mypi)
    print('Valor Decimal: {}'.format(float_hex_2_float(mypi, int(itera/3.2))))

    PI = open(nomepi, 'w')
    PI.write(mypi)
    tempo = time() - tempo
    print('\nOperação realizada em aproximadamente'
          , format(tempo, '.6f'), 'segundos.')
    print('Número de dígitos obtidos: ' + str(len(mypi)))

    k = 0
    if piComparado != '':
        while True:
            if k < len(mypi) and mypi[k] == piComparado[k].lower():
                k += 1
            else:
                print(k, 'digitos identicos consecutivos.')
                break

    call(['clear'])
    call(['echo', 'Tempo de processamento: ' + format(tempo, '.4f') + ' segundos.'])

    # input('\nPressione Enter fechar esta tela...')