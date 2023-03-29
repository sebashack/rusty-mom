# import asyncio
# import logging

# import grpc
# import messages_pb2
# import messages_pb2_grpc

from momlib import MoMClient


# async def run() -> None:
#    async with grpc.aio.insecure_channel("localhost:50051") as channel:
#        stub = messages_pb2_grpc.MessageStreamStub(channel)
#
#        message_stream = stub.SubscribeToChannel(
#            messages_pb2.SubscriptionRequest(
#                channel_id="fb584a4e-7a6c-4e2b-b49a-f341209da716"
#            )
#        )
#        while True:
#            res = await message_stream.read()
#            if res == grpc.aio.EOF:
#                break
#            content = res.content.decode("utf-8")
#            print(f"{(res.id, res.topic)}: {content}")
#


def main():
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_client.create_queue("qa")
    chan1Info = mom_client.create_channel("qa")
    pusher1 = MoMClient.get_pusher(chan1Info)

    pusher1.push(b"Hello!", "qa")
    pusher1.push(b"Hello!", "qa")
    pusher1.push(b"Hello!", "qa")
    pusher1.push(b"Hello!", "qa")
    pusher1.push(b"Hello!", "qa")

    pusher1.close()

    # mom_client.delete_queue("qa")
    # mom_client.create_queue("qa")
    # mom_client.create_channel("qa")
    # mom_client.create_channel("qa", "t1")


if __name__ == "__main__":
    # logging.basicConfig()
    # asyncio.run(run())
    main()
