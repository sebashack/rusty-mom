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

    def push_news():
        pusher = Pusher(mom_info)
        while True:
            now = str(datetime.now())
            msg = f"News with time {now}".encode("utf-8")
            try:
                pusher.push(msg, "news-queue", topic="news")
            except:
                time.sleep(3)
                _, info = mom_client.get_queue_info("news-queue")
                pusher = Pusher(info)
            time.sleep(5)

    threading.Thread(target=push_news).start()

    while True:
        time.sleep(10)


if __name__ == "__main__":
    main()
