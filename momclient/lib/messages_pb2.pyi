from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class CreateChannelRequest(_message.Message):
    __slots__ = ["queue_label", "topic"]
    QUEUE_LABEL_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    queue_label: str
    topic: str
    def __init__(self, queue_label: _Optional[str] = ..., topic: _Optional[str] = ...) -> None: ...

class CreateChannelResponse(_message.Message):
    __slots__ = ["channel_id"]
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    def __init__(self, channel_id: _Optional[str] = ...) -> None: ...

class CreateQueueOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class CreateQueueRequest(_message.Message):
    __slots__ = ["queue_label"]
    QUEUE_LABEL_FIELD_NUMBER: _ClassVar[int]
    queue_label: str
    def __init__(self, queue_label: _Optional[str] = ...) -> None: ...

class DeleteChannelOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class DeleteChannelRequest(_message.Message):
    __slots__ = ["channel_id"]
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    def __init__(self, channel_id: _Optional[str] = ...) -> None: ...

class DeleteQueueOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class DeleteQueueRequest(_message.Message):
    __slots__ = ["queue_label"]
    QUEUE_LABEL_FIELD_NUMBER: _ClassVar[int]
    queue_label: str
    def __init__(self, queue_label: _Optional[str] = ...) -> None: ...

class HeartbeatOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class HeartbeatRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ListChannelsRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ListChannelsResponse(_message.Message):
    __slots__ = ["channels"]
    CHANNELS_FIELD_NUMBER: _ClassVar[int]
    channels: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, channels: _Optional[_Iterable[str]] = ...) -> None: ...

class ListQueuesRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ListQueuesResponse(_message.Message):
    __slots__ = ["queues"]
    QUEUES_FIELD_NUMBER: _ClassVar[int]
    queues: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, queues: _Optional[_Iterable[str]] = ...) -> None: ...

class Message(_message.Message):
    __slots__ = ["content", "id", "topic"]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    content: bytes
    id: str
    topic: str
    def __init__(self, id: _Optional[str] = ..., content: _Optional[bytes] = ..., topic: _Optional[str] = ...) -> None: ...

class Push(_message.Message):
    __slots__ = ["content", "queue_label", "topic"]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    QUEUE_LABEL_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    content: bytes
    queue_label: str
    topic: str
    def __init__(self, content: _Optional[bytes] = ..., topic: _Optional[str] = ..., queue_label: _Optional[str] = ...) -> None: ...

class PushOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class RebuildQueueOkResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class RebuildQueueRequest(_message.Message):
    __slots__ = ["queue_label"]
    QUEUE_LABEL_FIELD_NUMBER: _ClassVar[int]
    queue_label: str
    def __init__(self, queue_label: _Optional[str] = ...) -> None: ...

class SubscriptionRequest(_message.Message):
    __slots__ = ["channel_id"]
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    def __init__(self, channel_id: _Optional[str] = ...) -> None: ...
