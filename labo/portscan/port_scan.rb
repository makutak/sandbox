require 'socket'

MAX_PORT_NUM = 65535

def make_socket()
  Socket::open(Socket::AF_INET, Socket::SOCK_RAW, Socket::IPPROTO_TCP)
end

#
# target_ip: 宛先のIP
# target_port: 宛先のport番号
# my_ip: 自分のport番号
# my_port 自分のIP
def make_tcp(target_ip: '127.0.0.1', target_port: 80, my_ip: '127.0.0.1', my_port: 9999)
  puts({ target_ip: target_ip, target_port: target_port, my_ip: my_ip, my_port: my_port })
end

def main
  # ソケット生成
  soc = make_socket
  puts "#" * 30
  puts soc
  puts "#" * 30

  # スキャン開始

  (1..MAX_PORT_NUM).each do |i|
    # TCPヘッダの作成
    puts i
    packet = make_tcp(target_ip: '93.184.216.34', target_port: i)
  end
end

main
