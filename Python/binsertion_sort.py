# python3
# TODO Contar o número de comparações
# TODO Implementar o insertion_sort padrão, para comparar
# TODO Testar estabilidade
# TODO Criar testes!

# À Saber: Qual o custo de criar novas listas dessa forma?
# À Saber: Existe forma melhor de criar novas listas?


def binsertion_sort(to_sort):
    to_sort = to_sort[:]
    return _binsertion_sort_(to_sort)


def _binsertion_sort_(to_sort):
    pivo = 0
    length = len(to_sort)
    for i in range(length-1):
        pivo += 1
        # print(pivo, to_sort[pivo], to_sort, '\n', sep='\n')
        pos = binary_search(to_sort[pivo], to_sort[:pivo])
        if pos == 0:
            to_sort = [to_sort[pivo]] + to_sort
            to_sort.pop(pivo+1)
        elif pos == -1:
            continue
        else:
            to_sort = to_sort[:pos] + [to_sort[pivo]] + to_sort[pos:]
            to_sort.pop(pivo+1 if pivo < length-1 else -1)
    return to_sort


def insertion_sort(to_sort):
    to_sort = to_sort[:]
    return _insertion_sort_(to_sort)


def _insertion_sort_(to_sort):
    pivo = 0
    length = len(to_sort)
    for i in range(length-1):
        pivo += 1
        element = to_sort[pivo]
        for i in range(len(to_sort[:pivo-1]), -1, -1):
            if element < to_sort[i]:
                to_sort[i+1], to_sort[i] = to_sort[i], to_sort[i+1]
                # print(element, i)
            else:
                # print('.')
                break
    return to_sort


# Search for the position that item should occupy
def binary_search(item, item_list, start=0, end=None, first_run=True):
    if first_run:
        length = len(item_list)
        if length == 0 or item < item_list[0]:
            return 0
        if item >= item_list[-1]:
            return -1
        end = length-1

    if item == item_list[start]:
        return start+1
    if item == item_list[end]:
        return end+1

    mid = (start+end)//2

    if mid in [start, end]:
        return mid+1

    if item < item_list[mid]:
        return binary_search(item, item_list, start=start, end=mid,
                             first_run=False)
    else:
        return binary_search(item, item_list, start=mid, end=end,
                             first_run=False)
