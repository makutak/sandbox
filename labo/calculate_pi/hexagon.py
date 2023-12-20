import matplotlib.pyplot as plt
import numpy as np

# 単位円を描くための角度と半径を定義
theta = np.linspace(0, 2 * np.pi, 100)
r = 1

# 単位円のxとy座標
x_circle = r * np.cos(theta)
y_circle = r * np.sin(theta)

# 正六角形の頂点を計算（単位円に内接）
hexagon_vertices = [(np.cos(2 * np.pi / 6 * i), np.sin(2 * np.pi / 6 * i)) for i in range(6)]

# 正六角形のxとy座標（最初の頂点を最後にも追加して閉じる）
x_hexagon = [v[0] for v in hexagon_vertices] + [hexagon_vertices[0][0]]
y_hexagon = [v[1] for v in hexagon_vertices] + [hexagon_vertices[0][1]]

# プロットの設定
plt.figure(figsize=(6, 6))
plt.plot(x_circle, y_circle, label='Unit Circle')  # 単位円
plt.plot(x_hexagon, y_hexagon, label='Hexagon')    # 正六角形

# 正六角形の対角線を描画
for i in range(6):
    plt.plot([hexagon_vertices[i][0], hexagon_vertices[(i+3) % 6][0]],
             [hexagon_vertices[i][1], hexagon_vertices[(i+3) % 6][1]],
             color='red')

# 軸の設定
plt.xlim(-1.5, 1.5)
plt.ylim(-1.5, 1.5)
plt.gca().set_aspect('equal', adjustable='box')

# 凡例の追加
plt.legend()

# グラフを表示
plt.show()
