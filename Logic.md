# Exercícios



### Mínimo de Swaps

##### Input

Lista de tamanho qualquer, com números 0 e 1.

##### Objetivo 

Calcular e imprimir quantas trocas entre vizinhos na lista são necessárias para que todos os "zeros" fiquem em segregados (e por consequência, todos os "uns" também)

#### Exemplos

##### A

[0,0,0,1,0] -> 1 troca entre a penúltima e o última casa segrega os valores ([0,0,0,**0**,**1**])

##### B

[0,1,0,1,0] -> [**1**,**0**,0,1,0] -> [1,0,**1**,**0**,0] -> [1,**1**,**0**,0,0]  (3 swaps)

[0,1,0,1,0] -> [0,**0**,**1**,1,0] -> [0,0,1,**0**,**1**] -> [0,0,**0**,**1**,1]  (3 swaps)

##### C

[1,1,0,1,1,1] -> [1,**0**,**1**,1,1,1] -> [**0**,**1**,1,1,1,1]  (2 swaps -> resposta esperada)

[1,1,0,1,1,1] -> [1,1,**1**,**0**,1,1] -> [1,1,1,**1**,**0**,1] -> [1,1,1,1,**0**,**1**] -> [1,1,1,1,**1**,**0**] (4 swaps)



### Fatorador

##### Input

Um número inteiro maior que zero

##### Output

Uma lista de números que, multiplicados, resultam no mesmo valor de input.

#### Exemplos

170 -> [2, 5, 17]

1 -> [1]

55 -> [5, 11]



### Fizzbuzz (1):

##### Objetivo

Imprimir números de 0 a 100, um por linha. 

Os números divisíveis por 3 devem ser substítuidos pela palavra "Fizz".

Os números divisíveis por 5 devem ser substítuidos pela palavra "Buzz".



### Fizzbuzz (2):

##### Input

Dois números *X* e *Y* primos;

##### Objetivo 

Imprimir números de 0 a 100, um por linha. 

Os números divisíveis por *X* devem ser substítuidos pela palavra "Fizz".

Os números divisíveis por *Y* devem ser substítuidos pela palavra "Buzz".



### Motor de Caixa Eletrônico

##### Input

- Um dicionário com as chaves: 2, 5, 10, 20, 50, 100 (os tipos de notas em uso), e cujos valores são a quantidade de notas disponíveis. Ex.: {2:54, 5:12} -> 54 notas de 2 reais, 12 notas de 5 reais, nenhuma das demais notas
- Um valor inteiro maior ou igual a 10, que será o valor a ser sacado

##### Objetivo

Calcular e imprimir quais as notas que compõe (por soma) a o valor a ser sacado, minimizando a quantidade total de notas a serem usadas, e imprimir o dicionário com a quantidade de notas restantes. Se não for possível compor o valor, imprimir mensagem de aviso.

#### Exemplos

{2:5, 5:1, 10:1}, 17 -> [10, 5, 2], {2:4, 5:0, 10:0}

{2:5, 5:2, 10:1}, 21 -> [10, 5, 2, 2, 2], {2:2, 5:1, 10:0}

{2:5, 5:2, 10:2}, 22 -> [10, 10, 2], {2:4, 5:2, 10:0}
