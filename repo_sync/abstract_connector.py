"""
Server and Client abstract class.

Copyright (C) 2021 Netpro Project RepoSync
"""
import socket
from abc import ABC


class AbstractConnect(ABC):
    """
    Server and Client abstract class.
    """

    def __init__(self, port: int, address: str):
        self._port = port
        self._address = address
        self._size = 2048
        self._socket = self._create_socket()

    def _create_socket(self) -> socket.socket:
        """
        Create socket instance.

        Returns:
            socket.socket: instance of socket.socket
        """
        return socket.socket(socket.AF_INET, socket.SOCK_STREAM)
