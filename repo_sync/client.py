import socket
from typing import List

PORT = 4455
FORMAT = "utf-8"
SIZE = 1024

def client(address: List[str], file_path: str) -> None:
    """
    address (List[str]): client address to synchronize.
    file_path (str): file path to synchronize.
    """
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect(address, PORT)

    file = open(file_path, "r")
    data = file.read()

    client.send(data.encode(FORMAT))
    msg = client.recv(SIZE).decode(FORMAT)
    print(f"[SERVER]: {msg}")

    """ Sending the file data to the server. """
    client.send(data.encode(FORMAT))
    msg = client.recv(SIZE).decode(FORMAT)
    print(f"[SERVER]: {msg}")

    file.close()

    client.close()