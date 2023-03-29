import threading
import time

from momlib import MoMClient, Pusher, Subscriber


def main():
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_client.create_queue("qa")
    chan1_info = mom_client.create_channel("qa")
    chan2_info = mom_client.create_channel("qa")

    def constant_push():
        pusher1 = Pusher(chan1_info)
        while True:
            pusher1.push(b"Hello!", "qa")
            time.sleep(1)

    push_thread = threading.Thread(target=constant_push)
    push_thread.start()

    def consumer1():
        subscriber = Subscriber(chan1_info)
        subscriber.consume()

    consumer1_thread = threading.Thread(target=consumer1)
    consumer1_thread.start()

    while True:
        time.sleep(10)

    # pusher1.close()


if __name__ == "__main__":
    main()
