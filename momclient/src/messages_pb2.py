# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: messages.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0emessages.proto\x12\x08messages\")\n\x13SubscriptionRequest\x12\x12\n\nchannel_id\x18\x01 \x01(\t\"5\n\x07Message\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0f\n\x07\x63ontent\x18\x02 \x01(\x0c\x12\r\n\x05topic\x18\x03 \x01(\t\";\n\x04Push\x12\x0f\n\x07\x63ontent\x18\x01 \x01(\x0c\x12\r\n\x05topic\x18\x02 \x01(\t\x12\x13\n\x0bqueue_label\x18\x03 \x01(\t\"\x10\n\x0ePushOkResponse\")\n\x12\x43reateQueueRequest\x12\x13\n\x0bqueue_label\x18\x01 \x01(\t\"\x17\n\x15\x43reateQueueOkResponse\")\n\x12\x44\x65leteQueueRequest\x12\x13\n\x0bqueue_label\x18\x01 \x01(\t\"\x17\n\x15\x44\x65leteQueueOkResponse\":\n\x14\x43reateChannelRequest\x12\x13\n\x0bqueue_label\x18\x01 \x01(\t\x12\r\n\x05topic\x18\x02 \x01(\t\"+\n\x15\x43reateChannelResponse\x12\x12\n\nchannel_id\x18\x01 \x01(\t\"*\n\x14\x44\x65leteChannelRequest\x12\x12\n\nchannel_id\x18\x01 \x01(\t\"\x19\n\x17\x44\x65leteChannelOkResponse2\xe0\x03\n\rMessageStream\x12J\n\x12SubscribeToChannel\x12\x1d.messages.SubscriptionRequest\x1a\x11.messages.Message\"\x00\x30\x01\x12\x39\n\x0bPushToQueue\x12\x0e.messages.Push\x1a\x18.messages.PushOkResponse\"\x00\x12N\n\x0b\x43reateQueue\x12\x1c.messages.CreateQueueRequest\x1a\x1f.messages.CreateQueueOkResponse\"\x00\x12N\n\x0b\x44\x65leteQueue\x12\x1c.messages.DeleteQueueRequest\x1a\x1f.messages.DeleteQueueOkResponse\"\x00\x12R\n\rCreateChannel\x12\x1e.messages.CreateChannelRequest\x1a\x1f.messages.CreateChannelResponse\"\x00\x12T\n\rDeleteChannel\x12\x1e.messages.DeleteChannelRequest\x1a!.messages.DeleteChannelOkResponse\"\x00\x42\x06\xa2\x02\x03MSGb\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'messages_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  DESCRIPTOR._serialized_options = b'\242\002\003MSG'
  _SUBSCRIPTIONREQUEST._serialized_start=28
  _SUBSCRIPTIONREQUEST._serialized_end=69
  _MESSAGE._serialized_start=71
  _MESSAGE._serialized_end=124
  _PUSH._serialized_start=126
  _PUSH._serialized_end=185
  _PUSHOKRESPONSE._serialized_start=187
  _PUSHOKRESPONSE._serialized_end=203
  _CREATEQUEUEREQUEST._serialized_start=205
  _CREATEQUEUEREQUEST._serialized_end=246
  _CREATEQUEUEOKRESPONSE._serialized_start=248
  _CREATEQUEUEOKRESPONSE._serialized_end=271
  _DELETEQUEUEREQUEST._serialized_start=273
  _DELETEQUEUEREQUEST._serialized_end=314
  _DELETEQUEUEOKRESPONSE._serialized_start=316
  _DELETEQUEUEOKRESPONSE._serialized_end=339
  _CREATECHANNELREQUEST._serialized_start=341
  _CREATECHANNELREQUEST._serialized_end=399
  _CREATECHANNELRESPONSE._serialized_start=401
  _CREATECHANNELRESPONSE._serialized_end=444
  _DELETECHANNELREQUEST._serialized_start=446
  _DELETECHANNELREQUEST._serialized_end=488
  _DELETECHANNELOKRESPONSE._serialized_start=490
  _DELETECHANNELOKRESPONSE._serialized_end=515
  _MESSAGESTREAM._serialized_start=518
  _MESSAGESTREAM._serialized_end=998
# @@protoc_insertion_point(module_scope)
