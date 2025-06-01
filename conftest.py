import socket
import typing

import pytest

CACHE_HOST = '127.0.0.1'
CACHE_PORT = 5555


@pytest.fixture
def tcp_client() -> typing.Generator[socket.socket, None, None]:
    _socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        _socket.connect((CACHE_HOST, CACHE_PORT))
        yield _socket
    finally:
        _socket.close()
