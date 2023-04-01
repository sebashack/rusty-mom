from collections import namedtuple
import json
import requests
import asyncio

import grpc
import messages_pb2
import messages_pb2_grpc


MoMInfo = namedtuple("MoMInfo", ["host", "port"])
Channel = namedtuple("Channel", ["id"])


class MoMClient:
    _http_host = None
    _http_port = None

    def __init__(self, http_host, http_port):
        self._http_host = http_host
        self._http_port = http_port

    def create_queue(self, queue_label):
        response = requests.post(f"{self.root()}/queues/{queue_label}")
        print(f"Response Status: {response.status_code}")
        print(f"Response body: {response.text}")

    def delete_queue(self, queue_label):
        response = requests.delete(f"{self.root()}/queues/{queue_label}")
        print(f"Response Status: {response.status_code}")
        print(f"Response body: {response.text}")

    def create_channel(self, queue_label, topic="__none__"):
        headers = {"Content-type": "application/json"}
        response = requests.put(
            f"{self.root()}/queues/{queue_label}/channels/{topic}", headers=headers
        )
        data = response.json()

        print(f"Response Status: {response.status_code}")
        print(f"Response body: {data}")
        return (MoMInfo(data["host"], data["port"]), Channel(data["id"]))

    def delete_channel(self, channel_id):
        response = requests.delete(f"{self.root()}/channels/{channel_id}")
        print(f"Response Status: {response.status_code}")
        print(f"Response body: {response.text}")

    def list_queues(self):
        response = requests.get(f"{self.root()}/queues")
        data = response.json()

        print(f"Response Status: {response.status_code}")
        print(f"Response body: {data}")

        return data

    def list_channels(self):
        response = requests.get(f"{self.root()}/channels")
        data = response.json()

        print(f"Response Status: {response.status_code}")
        print(f"Response body: {data}")

        return data

    # Helpers
    def root(self):
        return f"http://{self._http_host}:{self._http_port}"


class Pusher:
    _pusher = None
    _grpc_channel = None

    def __init__(self, mom_info):
        self._grpc_channel = grpc.insecure_channel(f"{mom_info.host}:{mom_info.port}")
        self._pusher = messages_pb2_grpc.MessageStreamStub(self._grpc_channel)

    def push(self, content, queue_label, topic="__none__"):
        self._pusher.PushToQueue(
            messages_pb2.Push(content=content, topic=topic, queue_label=queue_label)
        )
        print("Message pushed...")

    def close(self):
        self._grpc_channel.close()


class Subscriber:
    _mom_info = None
    _channel_id = None

    def __init__(self, mom_info, channel):
        self._mom_info = mom_info
        self._channel_id = channel.id

    def consume(self):
        async def run():
            async with grpc.aio.insecure_channel(
                f"{self._mom_info.host}:{self._mom_info.port}"
            ) as channel:
                stub = messages_pb2_grpc.MessageStreamStub(channel)

                message_stream = stub.SubscribeToChannel(
                    messages_pb2.SubscriptionRequest(channel_id=self._channel_id)
                )
                while True:
                    res = await message_stream.read()
                    if res == grpc.aio.EOF:
                        break
                    content = res.content.decode("utf-8")
                    print(f"{(res.id, res.topic)}: {content}")

        asyncio.run(run())
