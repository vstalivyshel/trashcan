
eval %sh{ python -c "
import sys
import socket
import os
rtp = os.environ.get('XDG_RUNTIME_DIR')
socket_path = os.path.join(rtp, 'kakoune', 'kak')

def send_cmd(cmd: str) -> bool:
    b_cmd = cmd.encode('utf-8')
    sock = socket.socket(socket.AF_UNIX)
    sock.connect(socket_path)
    b_content = encode_length(len(b_cmd)) + b_cmd
    b_header = b'\x02' + encode_length(len(b_content) + 5)
    b_message = b_header + b_content
    print('eval'b_message)
    return sock.send(b_message) == len(b_message)


def encode_length(str_length: int) -> bytes:
    return str_length.to_bytes(4, byteorder=sys.byteorder)

send_cmd('eval -client client0 %{ echo hello }')
"}

