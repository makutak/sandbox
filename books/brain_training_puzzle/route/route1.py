def search(w, h):
    if (w == 1) and (h == 1):
        # 到達したらカウント
        return 1

    cnt = 0

    if w > 1:
        #右方向に動ければ移動
        cnt += search(w - 1, h)

    if h > 1:
        #上方向に動ければ移動
        cnt += search(w, h - 1)

    return cnt


m = search(2, 3)
n = search(4, 2)

print(m * n)
