from datetime import datetime
import sys
import time

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber


def main():
    retry_delay = 3
    max_attempts = 3
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_info, channel_info = mom_client.create_channel("news-queue", retry_delay, max_attempts, "news")

    # mom_client.create_queue("news-comment-queue")

    subscriber = Subscriber(mom_info, channel_info)
    subscriber.consume()


if __name__ == "__main__":
    main()
