import time
from collections import deque
import heapq


class Scheduler:
    def __init__(self):
        self.ready = deque()
        self.sleeping = []
        self.current = None
        self.sequence = 0

    async def sleep(self, delay):
        """
        The current "coroutine" wants to sleep. How?
        """
        deadline = time.time() + delay
        self.sequence += 1
        heapq.heappush(self.sleeping, (deadline, self.sequence, self.current))
        self.current = None
        await switch()  # Switch tasks

    def new_task(self, coro):
        self.ready.append(coro)

    def run(self):
        while self.ready or self.sleeping:
            if not self.ready:
                deadline, _, coro = heapq.heappop(self.sleeping)
                delta = deadline - time.time()
                if delta > 0:
                    time.sleep(delta)
                self.ready.append(coro)

            self.current = self.ready.popleft()
            # Drive as a generator
            try:
                self.current.send(None)  # send to a co-routine
                if self.current:
                    self.ready.append(self.current)
            except StopIteration:
                pass


class Awaitable:
    def __await__(self):
        yield


def switch():
    return Awaitable()


async def count_up(y):
    x = 0
    while x < y + 1:
        print(x)
        await sched.sleep(1)
        x += 1


async def count_down(x):
    while x >= 0:
        print(x)
        await sched.sleep(1)
        x -= 1


async def main(sched):
    sched.new_task(count_down(10))
    sched.new_task(count_up(10))


if __name__ == "__main__":
    sched = Scheduler()  # Background scheduler object
    sched.new_task(main(sched))
    sched.run()
