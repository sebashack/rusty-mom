from datetime import datetime
import sys
import threading
import time

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber


def main():
    retry_delay = 3
    max_attempts = 3
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_client.create_queue("news-queue", retry_delay, max_attempts)

    _, mom_info = mom_client.get_queue_info("news-queue", retry_delay, max_attempts)

    def push_news():
        pusher = Pusher(mom_info)
        while True:
            attempts = 0
            while attempts < max_attempts:
                now = str(datetime.now())
                msg = f"News with time {now}".encode("utf-8")
                try:
                    pusher.push(msg, "news-queue", topic="news")
                    attempts = 0
                except:
                    if attempts == max_attempts-1:
                        print("Max retries reached")
                        exit()
                    time.sleep(retry_delay)
                    attempts += 1
                    _, info = mom_client.get_queue_info("news-queue", retry_delay, max_attempts)
                    pusher = Pusher(info)
                time.sleep(5)

    threading.Thread(target=push_news).start()

    while True:
        time.sleep(10)


if __name__ == "__main__":
    main()
