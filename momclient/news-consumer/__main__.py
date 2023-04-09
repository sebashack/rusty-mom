from datetime import datetime
import sys
import time
import threading

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber


message_list = []
mutex = threading.Lock()


def on_message(message):
    with mutex:
        message_list.append(message)

def main():
    retry_delay = 2
    max_attempts = 5
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_info, channel_info = mom_client.create_channel("news-queue", retry_delay, max_attempts, "news")

    subscriber = Subscriber(mom_info, channel_info)
    subscriber.consume(on_message)


if __name__ == "__main__":
    main()
