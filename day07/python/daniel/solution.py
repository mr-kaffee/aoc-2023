from collections import Counter
from functools import cmp_to_key

ENABLE_JOKER = True

value_dict = {'A': 14, 'K': 13, 'Q': 12, 'J': 1 if ENABLE_JOKER else 11, 'T': 10}
for i in range(2, 10):
    value_dict[str(i)] = i

def compare_hands(lhs: tuple[str, int], rhs: tuple[str, int]) -> int:
    if ENABLE_JOKER:
        lmc = [mc[0] for mc in Counter(lhs[0]).most_common()]
        rmc = [mc[0] for mc in Counter(rhs[0]).most_common()]

        lhs_cnt = Counter(lhs[0].replace('J', lmc[0] if lmc[0] != 'J' or len(lmc) == 1 else lmc[1]))
        rhs_cnt = Counter(rhs[0].replace('J', rmc[0] if rmc[0] != 'J' or len(rmc) == 1 else rmc[1]))
    else:
        lhs_cnt = Counter(lhs[0])
        rhs_cnt = Counter(rhs[0])

    for l, r in zip(lhs_cnt.most_common(), rhs_cnt.most_common()):
        if l[1] == r[1]:
            continue
        return 1 if l[1] > r[1] else -1
    
    for l, r in zip(lhs[0], rhs[0]):
        val_l, val_r = value_dict[l], value_dict[r]
        if val_l == val_r:
            continue
        return 1 if val_l > val_r else -1

    return 0

with open("../../../inputs/input07.txt") as f:
    hands: list[tuple[str, int]] = []
    for line in f:
        hand, bet = line.split(" ")
        hands.append((hand, int(bet)))

sum = 0
for i, (_, bid) in enumerate(sorted(hands, key=cmp_to_key(compare_hands))):
    sum += (i+1)*bid
print(sum)