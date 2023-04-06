import threading
import time

from momlib import MoMClient, Pusher, Subscriber


def main():
    mom_client1 = MoMClient("127.0.0.1", 8082)
    mom_client2 = MoMClient("127.0.0.1", 8082)
    mom_client1.create_queue("cola1")
    mom_client2.create_queue("cola2")
    mom_info1, channel1 = mom_client1.create_channel("cola1", "david")
    mom_info2, channel2 = mom_client2.create_channel("cola2", "guayaba")

    def constant_push():
        pusher1 = Pusher(mom_info1)
        pusher2 = Pusher(mom_info2)
        while True:
            pusher1.push(b"Hello cola #1!", "cola1")
            pusher2.push(b"Hello cola #2!", "cola2")
            time.sleep(2)

    push_thread = threading.Thread(target=constant_push)
    push_thread.start()

    while True:
        time.sleep(10)

    """
    print("---- PRUEBA GET CHANNELS (1) ----")
    mom_client.list_channels()

    print("---- PRUEBA GET TOPICS (1) ----")
    mom_client.get_queue_topics("qa")

    print("---- PRUEBA DELETE CHANNEL ----")
    mom_client.delete_channel(channel1.id)
    
    print("---- PRUEBA GET CHANNELS (2) ----")
    mom_client.list_channels()

    print("---- PRUEBA GET CHANNEL INFO ----")
    mom_client.get_channel_info(channel2.id)

    print("---- PRUEBA GET QUEUE INFO ----")
    mom_client.get_queue_info("qa")

    print("---- PRUEBA GET TOPICS (2) ----")
    mom_client.get_queue_topics("qa")


    
    def constant_push():
        pusher1 = Pusher(mom_info1)
        while True:
            pusher1.push(b"Hello!", "qa")
            time.sleep(1)

    push_thread = threading.Thread(target=constant_push)
    push_thread.start()

    def consumer1():
        subscriber = Subscriber(mom_info1, channel1)
        subscriber.consume()

    consumer1_thread = threading.Thread(target=consumer1)
    consumer1_thread.start()

    queue_labels = mom_client.list_queues()
    channel_ids = mom_client.list_channels()

    for cid in channel_ids:
        mom_client.delete_channel(cid)

    channel_ids = mom_client.list_channels()

    while True:
        time.sleep(10)

    # pusher1.close()
    """

if __name__ == "__main__":
    main()
