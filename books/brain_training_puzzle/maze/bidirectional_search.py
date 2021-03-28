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

# 探索済みのログ
log_fw = [[1, 1]]
log_bw = [[4, 7]]

# キュー(x座標、y座標のリスト)
fw = [[1, 1]]
bw = [[4, 7]]


# キュートログから次の位置のリストを得る
def get_next(queue, log):
    result = []

    for x, y in queue:
        for move in d:
            xd, yd = x + move[0], y + move[1]

            if maze[xd][yd] != 9:
                if [xd, yd] not in log:
                    # 過去に移動していない場所の場合
                    # ログとキューに追加する
                    log.append([xd, yd])
                    result.append([xd, yd])

    return result


# 双方向から同じ位置にいるか調べる
def check_duplicate(fw, bw):
    for i in fw:
        if i in bw:
            return True
    return False
