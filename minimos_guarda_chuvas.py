from numpy import inf

'''Dado que há n pessoas que precisam se proteger da chuva,
 e len(p) tipos de guarda-chuvas, os quais p[i] é
 o número de pessoas que podem se proteger sob o guarda-chuva i.

 Qual o número mínimo de guarda-chuvas que precisam ser utilizados
 para que exatamente as n pessoas estejam protegidas?'''


def combo_min(n, p):
    candidatos = [0] * len(p)
    p = sorted(p, reverse=True)

    for j in range(len(p)):
        n_temp = n; fatores = []
        for i in p[j:]:
            if i <= n_temp:
                candidatos[j] += n_temp//i
                fatores.append(
                    '{0} guarda-chuva(s) cobrindo {1} pessoa(s)'\
                               .format(n_temp//i,i))
                n_temp = n_temp%i
        if n_temp != 0:
            candidatos[j]=inf
        else:
           print(fatores)

    candidato = min(c for c in candidatos if c > 0)
    return candidato

if __name__ == "__main__":
    nps = [ (17, [1, 2, 3, 4, 5, 6, 7, 8]),
            (99,[2,3,4,17,35,11]),
            (13,[1,2,3]),
            (23,[2,3,9]),
            (15,[4,5,1])]

    for np in nps:
        print('-'*60)
        valor=combo_min(*np)
        print()
        if 0 < valor < inf:
            print('{0} pessoas. Tipos do guarda-chuva disponíveis: {1}'.format(*np))
            print('Mínimo necessário: ', valor)
        else:
            print('Não existe uma combinação perfeita.')
        print('-'*60,'\n\n')
