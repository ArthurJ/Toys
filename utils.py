from time import time
from functools import wraps

def time_it(funcao_captora=None, string_explicativa="Tempo de execução: {} segundos"):
    """
    :param string_explicativa: String a mostrar detalhes da função monitorada
    :param funcao_captora: Recebe o tempo, afim de disponíbilizar o resultado do decorator

        Decorator que mede e imprime o tempo de execução em segundos,
        opcionalmente recebe uma função que recebe o tempo, e pode enviar a outro objeto.
    """
    def decorador(method):
        @wraps(method)
        def wrapper(*args, **kw):
            ts = time()
            result = method(*args, **kw)
            te = time()

            if funcao_captora:
                funcao_captora(te-ts)

            print(string_explicativa.format(te-ts))
            return result
        return wrapper
    return decorador

#---------------------------------------------------------------------------------------------
import numpy as np

def walk(matriz, f=lambda x: np.min(x), kernel=3):
    ''' 
        Recebe uma matriz MxN.
        Devolve uma outra MxN (com as bordas excluídas), em que cada píxel [x,y] interno passou por uma
        função f (default: min) cujo argumento é uma matriz quadrada de dimensionalidade k que é a
        vizinhança imediata da posição [x,y] em que f está sendo aplicado
        
        A função walk foi desenvolvida durante o estudo de processamento de imagens digitais.
        A borda é deletada para evitar que o kernel ande para fora da imagem.
        Para evitar a exclusão das bordas, acrescente bordas de valor 0 e tamanho k.
        
        Para usar máscaras, basta colocar na função f.
    '''
    k=(kernel-1)//2
    shape = np.shape(matriz)
    nmatriz = np.zeros(shape, dtype='int')
    for px in range(k,shape[0]-k):
        for py in range(k,shape[1]-k):
            nmatriz[px,py] = f(matriz[px-k:px+k+1,py-k:py+k+1])
    return nmatriz

def equalize(pixel_value, sub, sup):
    ''' 
        Função de equalização de imagem
        sub: menor valor na imagem
        sup: maior valor na imagem
        pixel_value: valor do pixel a ser equalizado
    '''
    b=len(bin(sup)[2:])
    v = round(((2**b)-1)*(pixel_value-sub)/(sup-sub))
    return v
# x = np.arange(625).reshape((25,25))
# print(x)
# print(walk(x))
