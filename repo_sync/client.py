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

    port = 4455
    code = "utf-8"
    size = 1024
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect(address, port)

    with open(file_path, "r") as file:
        data = file.read()

        client.send(data.encode(code))
        msg = client.recv(size).decode(code)
        _LOG.info("message: %s", msg)

    client.close()