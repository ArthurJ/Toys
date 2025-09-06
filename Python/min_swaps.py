testes = [
    [0, 0, 0, 0, 0, 0],
    [1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 1, 0, 1, 0],
    [0, 0, 1, 0, 0, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 0, 1, 1, 1, 0],
]


def swap(lst, i, j):
    print(f'({i}, {j}): {lst} -> ', end='')
    lst[i], lst[j] = lst[j], lst[i]
    print(f'{lst}')


def sort(lst, comparison):
    moves = 0
    for i in range(len(lst)-1):
        if comparison(lst[i], lst[i+1]):
            for j in range(i, -1, -1):
                if lst[j+1] == lst[j]:
                    break
                moves += 1
                swap(lst, j, j+1)
    print('Finish:', lst)
    print('Moves:', moves, '\n')
    return moves


if __name__ == '__main__':

    for t in testes:
        print('Start:', t, '\n')
        minimum = min(sort(t.copy(), lambda a, b: a > b),
                      sort(t.copy(), lambda a, b: a < b))
        print(f'Minimum moves: {minimum}')
        print(30*'-', '\n', sep='')
