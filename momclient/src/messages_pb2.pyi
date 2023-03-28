from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class Message(_message.Message):
    __slots__ = ["content", "id", "topic"]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    content: bytes
    id: str
    topic: str
    def __init__(
        self,
        id: _Optional[str] = ...,
        content: _Optional[bytes] = ...,
        topic: _Optional[str] = ...,
    ) -> None: ...

class Push(_message.Message):
    __slots__ = ["content", "queue_label", "topic"]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    QUEUE_LABEL_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    content: bytes
    queue_label: str
    topic: str
    def __init__(
        self,
        content: _Optional[bytes] = ...,
        topic: _Optional[str] = ...,
        queue_label: _Optional[str] = ...,
    ) -> None: ...

class PushOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class SubscriptionRequest(_message.Message):
    __slots__ = ["channel_id"]
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    def __init__(self, channel_id: _Optional[str] = ...) -> None: ...
