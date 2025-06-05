import matplotlib.pyplot as plt
import numpy as np
import sys

# コマンドライン引数から n を取得（デフォルトは 0）
n = int(sys.argv[1]) if len(sys.argv) > 1 else 0

# 単位円を描くための角度と半径を定義
theta = np.linspace(0, 2 * np.pi, 100)
r = 1

# 単位円のxとy座標
x_circle = r * np.cos(theta)
y_circle = r * np.sin(theta)

# 多角形の頂点を計算（6 * 2**n 角形）
polygon_vertices = [(np.cos(2 * np.pi / (6 * 2**n) * i), np.sin(2 * np.pi / (6 * 2**n) * i)) for i in range(6 * 2**n)]

# 多角形のxとy座標（最初の頂点を最後にも追加して閉じる）
x_polygon = [v[0] for v in polygon_vertices] + [polygon_vertices[0][0]]
y_polygon = [v[1] for v in polygon_vertices] + [polygon_vertices[0][1]]

# プロットの設定
plt.figure(figsize=(6, 6))
plt.plot(x_circle, y_circle, label='Unit Circle')  # 単位円
plt.plot(x_polygon, y_polygon, label=f'{6 * 2**n}-sided Polygon')  # 多角形

# 軸の設定
plt.xlim(-1.5, 1.5)
plt.ylim(-1.5, 1.5)
plt.gca().set_aspect('equal', adjustable='box')

# 凡例の追加
plt.legend()

# グラフを表示
plt.show()
