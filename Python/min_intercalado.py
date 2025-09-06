from math import floor


def correct(balls):
    return (balls,('B.'*balls)[:-1])

def to_num(buckets):
    return [1 if b=='B' else 0 for b in buckets]

def shift_list(buckets, positions):
    return buckets[positions:]+buckets[:positions]

def shift_prepositions(buckets):
    pre_positions = 0
    for b in buckets:
        if b == '.':
            pre_positions += 1
        else:
            break
    return shift_list(buckets, pre_positions)

def min_moves(buckets):
    balls = len([b for b in buckets if b=='B'])

    buckets=shift_prepositions(buckets)

    i, subbucket = correct(balls)
    moves = balls
    for j in range(balls):
        buckets = shift_prepositions(buckets)
        diffs = [(i,v) for i,v in enumerate(zip(buckets, subbucket+'.'*(len(buckets)+1-i*2))) if v[0]!=v[1]]
        print(floor(len(diffs)/2), diffs)
        moves = min(moves, floor(len(diffs)/2))
        buckets = shift_list(buckets, 1)
    return moves

if __name__ == '__main__':
    min_moves('.B..B..B')