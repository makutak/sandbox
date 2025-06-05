# 再度必要なモジュールをインポート
import matplotlib.pyplot as plt
import numpy as np

# 単位円の定義
theta = np.linspace(0, 2 * np.pi, 100)
r = 1
x_circle = r * np.cos(theta)
y_circle = r * np.sin(theta)

# 正六角形の頂点の計算
hexagon_vertices = [(np.cos(2 * np.pi / 6 * i), np.sin(2 * np.pi / 6 * i)) for i in range(6)]
x_hexagon = [v[0] for v in hexagon_vertices] + [hexagon_vertices[0][0]]
y_hexagon = [v[1] for v in hexagon_vertices] + [hexagon_vertices[0][1]]

# 正六角形の一辺の中点とその中点を単位円の周上まで伸ばす点の計算
midpoint = ((hexagon_vertices[0][0] + hexagon_vertices[1][0]) / 2,
            (hexagon_vertices[0][1] + hexagon_vertices[1][1]) / 2)
vector_to_midpoint = np.array(midpoint)
normalized_vector = vector_to_midpoint / np.linalg.norm(vector_to_midpoint)
extended_point = normalized_vector * r

# 左上の頂点の取得
left_top_vertex = hexagon_vertices[1]

# 描画
plt.figure(figsize=(6, 6))
plt.plot(x_circle, y_circle, label='Unit Circle')  # 単位円
plt.plot(x_hexagon, y_hexagon, label='Hexagon')    # 正六角形

# 正六角形の対角線
for i in range(6):
    plt.plot([hexagon_vertices[i][0], hexagon_vertices[(i+3) % 6][0]],
             [hexagon_vertices[i][1], hexagon_vertices[(i+3) % 6][1]],
             color='red')

# 正六角形の中心から円の周上の点まで線を引く
plt.plot([0, extended_point[0]], [0, extended_point[1]], color='green')

# 新たな三角形の線
plt.plot([left_top_vertex[0], extended_point[0]], [left_top_vertex[1], extended_point[1]], color='blue')

# 軸と凡例の設定
plt.xlim(-1.5, 1.5)
plt.ylim(-1.5, 1.5)
plt.gca().set_aspect('equal', adjustable='box')
plt.legend()

# グラフを表示
plt.show()
