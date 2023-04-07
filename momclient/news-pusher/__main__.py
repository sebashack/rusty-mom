from datetime import datetime
import sys
import threading
import time

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber


def main():
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_client.create_queue("news-queue")

    _, mom_info = mom_client.get_queue_info("news-queue")

    def push_sports():
        pusher = Pusher(mom_info)
        while True:
            now = str(datetime.now())
            msg = f"Sports news with time {now}".encode("utf-8")
            pusher.push(msg, "news-queue", topic="sports")
            time.sleep(5)

    threading.Thread(target=push_sports).start()

    def push_politics():
        pusher = Pusher(mom_info)
        while True:
            now = str(datetime.now())
            msg = f"Politics news with time {now}".encode("utf-8")
            pusher.push(msg, "news-queue", topic="politics")
            time.sleep(5)

    threading.Thread(target=push_politics).start()

    def push_fashion():
        pusher = Pusher(mom_info)
        while True:
            now = str(datetime.now())
            msg = f"Fashion news with time {now}".encode("utf-8")
            pusher.push(msg, "news-queue", topic="fashion")
            time.sleep(5)

    threading.Thread(target=push_fashion).start()

    while True:
        time.sleep(10)


if __name__ == "__main__":
    main()
