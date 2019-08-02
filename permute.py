from itertools import permutations

# _ + _ * _^2 + _^3 - _ = 399

for l in list(permutations([2, 3, 5, 7, 9])):
    if l[0] + l[1] * l[2] * l[2] + l[3] * l[3] * l[3] - l[4] == 399:
        print(l)
        
