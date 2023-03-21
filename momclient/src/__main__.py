import asyncio
import logging

import grpc
import messages_pb2
import messages_pb2_grpc


async def run() -> None:
    async with grpc.aio.insecure_channel("localhost:50051") as channel:
        stub = messages_pb2_grpc.MessageStreamStub(channel)

        message_stream = stub.SubscribeToChannel(messages_pb2.SubscriptionRequest(channel_id="fb584a4e-7a6c-4e2b-b49a-f341209da716"))
        while True:
            res = await message_stream.read()
            if res == grpc.aio.EOF:
                break
            content = res.content.decode('utf-8')
            print(f"{(res.id, res.topic)}: {content}")


if __name__ == "__main__":
    logging.basicConfig()
    asyncio.run(run())
