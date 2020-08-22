"""ソケットを使って ADD プロトコルを実装したクライアントのスクリプト"""

import socket
import struct


def send_msg(sock, msg):
    """ソケットに指定したバイト列を書き込む関数"""
    # これまでに送信できたバイト数
    total_sent_len = 0
    # 送信したいバイト数
    total_msg_len = len(msg)
    # まだ送信したいデータが残っているか判定する
    while total_sent_len < total_msg_len:
        # ソケットにバイト列を書き込んで、書き込めたバイト数を得る
        sent_len = sock.send(msg[total_sent_len:])
        # まったく書き込めなかったらソケットの接続が終了している
        if sent_len == 0:
            raise RuntimeError('socket conection broken')
        # 書き込めた分を加算する
        total_sent_len += sent_len


def recv_msg(sock, total_msg_size):
    """ソケットから特定のバイト数を読み込む関数"""
    # これまでに受信できたバイト数
    total_recv_size = 0
    # 指定したバイト数を受信できたか判定する
    while total_recv_size < total_msg_size:
        # 残りのバイト列を受信する
        received_chunk = sock.recv(total_msg_size - total_recv_size)
        # 1バイトも読めなかったときはソケットの接続が終了している
        if len(received_chunk) == 0:
            raise RuntimeError('socket connection broken')
        # 受信したバイト数を返す
        yield received_chunk
        # 受信できたバイト数を加算する
        total_recv_size += len(received_chunk)
