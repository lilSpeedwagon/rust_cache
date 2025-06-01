import dataclasses
import socket
import struct


@dataclasses.dataclass
class RequestHeader:
    proto_version: int
    ops_count: int
    checksum: int
    body_size: int

    def to_bytes(self) -> bytes:
        return struct.pack(
            ">BBHIII",
            self.proto_version,
            0,  # reserved1
            self.ops_count,
            self.checksum,
            self.body_size,
            0,  # reserved2
        )


@dataclasses.dataclass
class RequestOperationHeader:
    op_code: int
    body_size: int

    def to_bytes(self) -> bytes:
        return struct.pack(
            ">BBHI",
            self.op_code,
            0,  # reserved1
            0,  # reserved2
            self.body_size,
        )


@dataclasses.dataclass
class RequestOperation:
    op_code: int
    body: bytes

    def to_bytes(self) -> bytes:
        header = RequestOperationHeader(
            op_code=self.op_code,
            body_size=len(self.body),
        )
        return header.to_bytes() + self.body
    

@dataclasses.dataclass
class Request:
    operations: list[RequestOperation]

    def to_bytes(self) -> bytes:
        body = b"".join(op.to_bytes() for op in self.operations)
        header = RequestHeader(
            proto_version=1,
            ops_count=len(self.operations),
            checksum=0,  # Placeholder
            body_size=len(body),
        )
        return header.to_bytes() + body


def test__access(tcp_client: socket.socket) -> None:
    """
    Run a request with dummy operations to test the connectivity.
    """

    request = Request(
        operations=[
            RequestOperation(op_code=1, body=b"Dummy operation 1"),
            RequestOperation(op_code=2, body=b"Dummy operation 2"),
        ]
    )
    tcp_client.sendall(request.to_bytes())

    _buffer_size = 4096
    response = tcp_client.recv(_buffer_size)
    assert response == b"OK"
