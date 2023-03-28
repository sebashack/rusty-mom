# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import messages_pb2 as messages__pb2


class MessageStreamStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.SubscribeToChannel = channel.unary_stream(
            "/messages.MessageStream/SubscribeToChannel",
            request_serializer=messages__pb2.SubscriptionRequest.SerializeToString,
            response_deserializer=messages__pb2.Message.FromString,
        )
        self.PushToChannel = channel.unary_unary(
            "/messages.MessageStream/PushToChannel",
            request_serializer=messages__pb2.Push.SerializeToString,
            response_deserializer=messages__pb2.PushOkResponse.FromString,
        )


class MessageStreamServicer(object):
    """Missing associated documentation comment in .proto file."""

    def SubscribeToChannel(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def PushToChannel(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")


def add_MessageStreamServicer_to_server(servicer, server):
    rpc_method_handlers = {
        "SubscribeToChannel": grpc.unary_stream_rpc_method_handler(
            servicer.SubscribeToChannel,
            request_deserializer=messages__pb2.SubscriptionRequest.FromString,
            response_serializer=messages__pb2.Message.SerializeToString,
        ),
        "PushToChannel": grpc.unary_unary_rpc_method_handler(
            servicer.PushToChannel,
            request_deserializer=messages__pb2.Push.FromString,
            response_serializer=messages__pb2.PushOkResponse.SerializeToString,
        ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
        "messages.MessageStream", rpc_method_handlers
    )
    server.add_generic_rpc_handlers((generic_handler,))


# This class is part of an EXPERIMENTAL API.
class MessageStream(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def SubscribeToChannel(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_stream(
            request,
            target,
            "/messages.MessageStream/SubscribeToChannel",
            messages__pb2.SubscriptionRequest.SerializeToString,
            messages__pb2.Message.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )

    @staticmethod
    def PushToChannel(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/messages.MessageStream/PushToChannel",
            messages__pb2.Push.SerializeToString,
            messages__pb2.PushOkResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )
