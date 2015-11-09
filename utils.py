from time import time


def time_it(funcao_captora=None, string_explicativa="Tempo de execução: {} segundos"):
    """
    :param string_explicativa: String a mostrar detalhes da função monitorada
    :param funcao_captora: Recebe o tempo, afim de disponíbilizar o resultado do decorator

        Decorator que mede e imprime o tempo de execução em segundos,
        opcionalmente recebe uma função que recebe o tempo, e pode enviar a outro objeto.
    """
    def decorador(method):
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