"""
This is a clientsocket program.
"""

import socket
import logging
from typing import List


logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)


def client(address: List[str], file_path: str) -> None:
    """
    address (List[str]): clientsocket address to synchronize.
    file_path (str): file path to synchronize.
    """
    port = 4455
    code = "utf-8"
    size = 1024
    addres = (address, port)
    clientsocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    clientsocket.connect(addres)
    with open(file_path, "r") as file:
        data = file.read()
        clientsocket.send(data.encode(code))
        msg = clientsocket.recv(size).decode(code)
        _LOG.info("message: %s", msg)

    clientsocket.close()
