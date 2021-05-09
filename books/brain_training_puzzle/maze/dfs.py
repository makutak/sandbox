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

d = [[0, -1], [-1, 0], [0, 1], [1, 0]]


# 最も短いゴールまでの経路を返す
def search(log):
    # 最後の位置を取得
    x, y = log[-1]

    if maze[x][y] == 1:
        # ゴールしたら深さ（経路の長さ）を返す
        return len(log) - 1

    # 深さとして十分大きな値をセット
    depth = [999999]
    # print('depth', depth)
    # print('log', log)

    for move in d:
        if maze[x + move[0]][y + move[1]] != 9:
            if [x + move[0], y + move[1]] not in log:
                # 過去に移動していない場所であれば移動
                log.append([x + move[0], y + move[1]])
                # 再起
                #print('search(log)', search(log))
                depth.append(search(log))
                #print(log, depth)
                log.pop(-1)

    return min(depth)


print(search([[1, 1]]))
