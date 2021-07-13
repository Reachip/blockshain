import os
import socket

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sockets = [f"/Users/rachedmejri/sockets/{filename}" for filename in os.listdir("/Users/rachedmejri/sockets")
           if filename.endswith(".sock")]


for s in sockets:
    try:
        print("For :", s)
        sock.connect(s)

        for i in range(10):
            sock.sendall(b"from python sock")
            print("=> Get", sock.recv(1024))

    except Exception as why:
        print("=>", why)
