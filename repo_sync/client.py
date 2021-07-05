"""
This is a clientsocket program.
"""

import logging
import socket
from multiprocessing import Process
from pathlib import Path

from .abstract_connector import AbstractConnect
from .core_op import fop

logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)


class Client(AbstractConnect):
    """
    Client.
    """

    def connect(self, file_path: Path, cache_dir: Path):
        """
        Client run.

        Args:
            file_path (Path): Target file path.
            cache_dir (Path): Directory of cache.
        """
        cache_file = cache_dir.joinpath('cache_file')
        cache_is_update = cache_dir.joinpath('is_update')
        self._socket.connect((self._address, self._port))

        process_first = Process(
            target=self.client_process, args=(self._socket, cache_file, self._size, cache_is_update))
        process_first.start()

        fop(str(file_path), str(cache_dir))

    @staticmethod
    def client_process(server_socket: socket.socket, cache_file_path: Path, size: int, is_update_path: Path):
        """
        Connect to server.

        Args:
            server_socket (socket.socket): Server socket contractor.
            cache_file_path (Path): read cache file path.
            size (int): send size.
            is_update_path (Path): update check file path.
        """
        is_send = False
        while True:
            if cache_file_path.exists():
                with open(str(is_update_path), mode="r") as file:
                    data = file.read()
                is_update = data == 'true'

                if is_update == is_send:
                    server_socket.send('None'.encode('UTF-8'))
                else:
                    with open(str(cache_file_path), mode='r') as file:
                        data = file.read()

                    _LOG.info('Send!')
                    if data == '':
                        server_socket.send("[[empty]]".encode('UTF-8'))
                    else:
                        server_socket.send(data.encode('UTF-8'))
                    is_send = is_update
                try:
                    msg = server_socket.recv(size).decode('UTF-8')
                    if msg != 'None':
                        if msg == "[[empty]]":
                            msg = ""

                        _LOG.info('Update file!')
                        with open(str(cache_file_path), mode='w') as file:
                            file.write(msg)
                except socket.timeout:
                    _LOG.error("Recv timeouted.")
