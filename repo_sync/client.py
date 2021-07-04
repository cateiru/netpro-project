import socket
import logging
from typing import List


logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)

def client(address: List[str], file_path: str) -> None:
    """
    address (List[str]): client address to synchronize.
    file_path (str): file path to synchronize.
    """

    PORT = 4455
    FORMAT = "utf-8"
    SIZE = 1024
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect(address, PORT)

    with open(file_path, "r") as file:
        data = file.read()

        client.send(data.encode(FORMAT))
        msg = client.recv(SIZE).decode(FORMAT)
        _LOG.info(f"[SERVER]: {msg}")

    client.close()
