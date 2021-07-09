"""
This is a serversocket program.
"""
import json
import logging
import socket
import time
from multiprocessing import Process, Value
from pathlib import Path
from typing import Any

from .abstract_connector import AbstractConnect

logging.basicConfig()
_LOG = logging.getLogger(__name__)
_LOG.setLevel(logging.INFO)


class Server(AbstractConnect):
    """
    Server.
    """

    def connect(self):
        """
        Server run.
        """
        self._socket.bind((self._address, self._port))
        self._socket.listen(5)

        thread_jobs = []
        is_update = Value('i', 0)
        cache_file = Path('cache')

        # create cache file
        with open(str(cache_file), mode="w"):
            pass

        while True:
            client_socket, address = self._socket.accept()
            _LOG.info('Connect client: %s', address)
            thread_jobs.append(
                Process(target=self.server_process, args=(client_socket, self._size, is_update, cache_file)))
            thread_jobs[-1].start()

    @staticmethod
    def server_process(client_socket: socket.socket, size: int, is_update: Any, cache_file: Path):
        """
        Connection server to client process.

        Args:
            client_socket (socket.socket): Socket of client connection.
            size (int): send buffer size.
            is_update (any): update flag.
            cache_file (Path): cache file path.
        """
        is_send = is_update.value

        while True:
            msg = json.loads(client_socket.recv(size).decode('UTF-8'))

            if msg['status'] == 1:
                _LOG.info('Update cache!')
                with open(str(cache_file), mode='w') as file:
                    file.write(msg['data'])
                if is_update.value == 1:
                    is_update.value = 0
                    is_send = 0
                else:
                    is_update.value = 1
                    is_send = 1

                client_socket.send(json.dumps({'status': 0, 'data': ''}).encode('UTF-8'))
            elif msg['status'] == 2:
                _LOG.info('Upload cache!')
                with open(str(cache_file), mode='r') as file:
                    data = file.read()
                client_socket.send(json.dumps({'status': 1, 'data': data}).encode('UTF-8'))
            else:
                if is_update.value != is_send:
                    with open(str(cache_file), mode='r') as file:
                        data = file.read()
                    _LOG.info('Send client')
                    client_socket.send(json.dumps({'status': 1, 'data': data}).encode('UTF-8'))
                    is_send = is_update.value
                else:
                    client_socket.send(json.dumps({'status': 0, 'data': ''}).encode('UTF-8'))

            time.sleep(0.5)
