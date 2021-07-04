import socket
import threading
from typing import List

PORT = 4455
FORMAT = "utf-8"
SIZE = 1024

def server(address: List[str], file_path: str)-> None:
    """
    address (List[str]): client address to synchronize.
    file_path (str): file path to synchronize.
    """
    print('Starting')
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind(address, PORT)
    server.listen(5)

    while True:
        conn, addr = server.accept()
        filename = conn.recv(SIZE).decode(FORMAT)
        print(filename)
        with open(file_path, "w") as file:
            conn.send("Filename received.".encode(FORMAT))
            
            data = conn.recv(SIZE).decode(FORMAT)
            print(f"[RECV] Receiving the file data.")
            file.write(data)
            conn.send("File data received".encode(FORMAT))
        conn.close()
        print('disconnected')

        break
