import socket
import logging
from typing import List


logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)

def server(address: List[str], file_path: str)-> None:
    """
    address (List[str]): client address to synchronize.
    file_path (str): file path to synchronize.
    """
    PORT = 4455
    FORMAT = "utf-8"
    SIZE = 1024
    _LOG.info('Starting')
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind(address, PORT)
    server.listen(5)

    conn, addr = server.accept()
    filename = conn.recv(SIZE).decode(FORMAT)
    _LOG.info(filename)
    with open(file_path, "w") as file:
        conn.send("Filename received.".encode(FORMAT))
        
        data = conn.recv(SIZE).decode(FORMAT)
        _LOG.info(f"[RECV] Receiving the file data.")
        file.write(data)
        conn.send("File data received".encode(FORMAT))
    conn.close()
    _LOG.info('disconnected')
