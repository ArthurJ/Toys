from numpy import array, fromiter, append, \
			    	unique, sum, sqrt, log, ulonglong

dt_padrao = ulonglong
valor = dt_padrao(input('Entre com o valor: '))

# Garante que o limite n'ao explodirá para números grandes
limite_verificado = log(valor) 
limite_verificado = int(limite_verificado)+1
if limite_verificado % 2 == 0:
	limite_verificado += 1

	
def calc_divisores(valor, limite_verificado):
	return fromiter([i for i in range(limite_verificado, 1, -2) 
						if valor % i == 0], dtype=dt_padrao)

						
def calc_divisores_primos(divisores):
    return unique(fromiter([i for x in divisores
                                for i in divisores 
                                if x <= i
                                and sum(i % divisores == 0)==1], 
                            dtype=dt_padrao),
                    return_inverse=True)[0]


def fatora(v, divisores_primos):
	while v != 1:
		v_ini = v
		for d in divisores_primos:
			if v % d == 0:
				v/=d
				break
		if v_ini == v:
			if v % 2 != 0:
				divisores_primos = append(divisores_primos, 
									  array([v_ini], 
									  dtype=dt_padrao))
			else:
				v/=2
				divisores_primos = append(divisores_primos, 
									  array([2], 
									  dtype=dt_padrao))
	return divisores_primos

	
# Encontro os primeiros fatores primos do valor do input
# (por que é fácil e barato)

divisores = calc_divisores(valor, limite_verificado)
divisores_primos = calc_divisores_primos(divisores)

# Uso os fatores primos menores para encontrar os maiores
# por fatoração simples

divisores_primos = fatora(valor, divisores_primos)			  
divisores_primos.sort()

print('\n\nValor de entrada:', valor)
print('Divisores primos:', divisores_primos)
print('Maior divisor primo: ', divisores_primos[-1])
