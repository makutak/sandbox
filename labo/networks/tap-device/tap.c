#include <errno.h>
#include <fcntl.h>
#include <linux/if_tun.h>
#include <net/if.h>
#include <stdio.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <unistd.h>

#define CLONE_DEVICE "/dev/net/tun"

int main(int argc, char *argv[]) {
  int fd;                // ファイルディスクリプタ
  struct ifreq ifr = {}; // インターフェース設定用構造体
  char buf[2046];        // 受信パケットを一時的に格納するバッファ(MTU + 余裕)
  char *ifname;
  ssize_t n; // システムコールが読み込んだバイト数を格納する

  // 実行時にインターフェース名が渡されなければエラー
  if (argc != 2) {
    fprintf(stderr, "usage: %s ifname\n", argv[0]);
    return -1;
  }

  // /dev/net/tunを読み書きモードで開く
  fd = open(CLONE_DEVICE, O_RDWR);
  if (fd == -1) {
    perror("open");
    return -1;
  }

  // 仮想デバイスの生成

  // コマンドライン引数で指定された名前を設定用構造体の名前フィールドにコピーする
  strncpy(ifr.ifr_name, argv[1], sizeof(ifr.ifr_name) - 1);
  // 動作モードの指定
  // - IFF_TAP: L2(イーサネット)モード。イーサネットヘッダを含むフレーム全体を扱う
  // - IFF_NO_PI: No Packet Information 4バイトのパケット情報ヘッダ(Flags/Proto)を付けない設定
  ifr.ifr_flags = IFF_TAP | IFF_NO_PI;
  // fdに対してTAPインターフェースを割り当てる
  if (ioctl(fd, TUNSETIFF, &ifr) == -1) {
    perror("ioctl [TUNSETIFF]");
    close(fd);
    return -1;
  }

  ifname = ifr.ifr_name;
  // カーネルによって割り当てられた正式なインターフェース名を表示する
  fprintf(stderr, "ifname: %s\n", ifname);
  while (1) {
    n = read(fd, buf, sizeof(buf)); // TAPデバイスにパケットが到達するまで待機
    if (n == -1) {
      if (errno == EINTR) {
        continue;
      }
      perror("read");
      close(fd);
      return -1;
    }
    // 受信に成功したら受信したバイト数とインターフェース名を表示する
    printf("recv: %zd via %s\n", n, ifname);
  }
  close(fd);
  return 0;
}
