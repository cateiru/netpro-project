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
    port = 4455
    format = "utf-8"
    size = 1024
    _LOG.info('Starting')
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind(address, port)
    server.listen(5)

    conn, addr = server.accept()
    filename = conn.recv(size).decode(format)
    _LOG.info("filename: %s",filename)
    with open(file_path, "w") as file:
        conn.send("Filename received.".encode(format))
        
        data = conn.recv(size).decode(format)
        _LOG.info("Receiving the file data.")
        file.write(data)
        conn.send("File data received".encode(format))
    conn.close()
    _LOG.info('disconnected')
