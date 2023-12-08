#!/usr/bin/env python3
# dpw@tiburon.local
# 2023-12-08 18:18:58

import socket

UDP_IP = "127.0.0.1"
UDP_PORT = 22200
MESSAGE = b"/ping"

count = 0
while count < 3_000:
    count = count + 1

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.settimeout(5)  # Set timeout to 5 seconds
    sock.sendto(MESSAGE, (UDP_IP, UDP_PORT))

    try:
        data, addr = sock.recvfrom(1024)
        print("rcvd :", count, data.decode())
    except socket.timeout:
        print("Timed out while waiting for response")
    finally:
        sock.close()

