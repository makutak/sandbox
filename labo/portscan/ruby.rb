require 'socket'

def make_socket()
  soc = Socket::open(Socket::AF_INET, Socket::SOCK_RAW, Socket::IPPROTO_TCP)
  soc
end



def main
  soc = make_socket
  puts "#" * 30
  puts soc
  puts "#" * 30
end

main
