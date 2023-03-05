#!/bin/env python3
# Stolen from:
# https://github.com/tomKPZ/pykak/blob/main/kak_socket.py
# https://github.com/caksoylar/kakoune-smooth-scroll

import os
import socket
import sys

def init(session):
    global socket_path
    socket_path = get_socket_path(session)

def send_cmd(cmd: str) -> bool:
    bytes_cmd = cmd.encode('utf-8')
    sock = socket.socket(socket.AF_UNIX)
    sock.connect(socket_path)
    bytes_content = encode_length(len(bytes_cmd)) + bytes_cmd
    bytes_header = b'\x02' + encode_length(len(bytes_content) + 5)
    bytes_message = bytes_header + bytes_content
    return sock.send(bytes_message) == len(bytes_message)


def encode_length(str_length: int) -> bytes:
    return str_length.to_bytes(4, byteorder=sys.byteorder)

def get_socket_path(session: str) -> str:
    xdg_runtime_dir = os.environ.get('XDG_RUNTIME_DIR')
    return os.path.join(xdg_runtime_dir, 'kakoune', session)


session = os.environ['kak_session']
client = os.environ['kak_client']
socket_path = get_socket_path(session)

print(session)
print(client)
print(socket_path)
