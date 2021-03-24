# 1 がゴール
maze = [
    [9, 9, 9, 9, 9, 9, 9, 9, 9],
    [9, 0, 0, 0, 9, 9, 0, 9, 9],
    [9, 0, 9, 9, 0, 0, 0, 0, 9],
    [9, 0, 0, 0, 0, 9, 9, 9, 9],
    [9, 9, 0, 9, 0, 9, 0, 1, 9],
    [9, 0, 0, 9, 0, 0, 0, 9, 9],
    [9, 0, 9, 0, 0, 9, 0, 0, 9],
    [9, 9, 9, 9, 9, 9, 9, 9, 9],
]

# 移動する方向
# 上、右、下、左
d = [[0, -1], [-1, 0], [0, 1], [1, 0]]

# 初期値
y, x = 1, 1

# 進行方向をセット
dir = 0

while maze[y][x] != 1:
    move = d[(dir + 1) % 4]  # 進行方向の右側
    if maze[(y + move[0])][x + move[1]] != 9:
        # 壁でなければ移動し、進行方向を右に変える
        dir = (dir + 1) % 4
        y += move[0]
        x += move[1]
        print("y, x", y, x, "num", maze[y][x])
        tmp = maze[y][x]
        maze[y][x] = "*"
        for i in maze:
            print(i)

        maze[y][x] = tmp
        print('')

    else:
        # 壁の場合は進行方向を左に変える
        print('#y: %d, x: %d is wall!' % (y + move[0], x + move[1]))
        dir = (dir + 3) % 4

print('## GOAL ##')
print(y, x)
print('##########')
