import threading
import time

from momlib import MoMClient, Pusher, Subscriber


def main():
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_client.create_queue("qa")
    mom_info1, channel1 = mom_client.create_channel("qa")
    mom_info2, channel2 = mom_client.create_channel("qa")

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

    mom_client.list_queues()
    mom_client.list_channels()

    while True:
        time.sleep(10)

    # pusher1.close()


if __name__ == "__main__":
    main()
