# Dining philosophers problem

https://en.wikipedia.org/wiki/Dining_philosophers_problem

![Alt text](image.png)

## Problem statement

Five philosophers dine together at the same table. Each philosopher has their own plate at the table. There is a fork between each plate. The dish served is a kind of spaghetti which has to be eaten with two forks. Each philosopher can only alternately think and eat. Moreover, a philosopher can only eat their spaghetti when they have both a left and right fork. Thus two forks will only be available when their two nearest neighbors are thinking, not eating. After an individual philosopher finishes eating, they will put down both forks. The problem is how to design a regimen (a concurrent algorithm) such that no philosopher will starve; i.e., each can forever continue to alternate between eating and thinking, assuming that no philosopher can know when others may want to eat or think (an issue of incomplete information).

The problem is that they each will think for an undetermined amount of time and may end up holding a left fork thinking, staring at the right side of the plate, unable to eat because there is no right fork, until they starve.

### Instructions of each philosopher

- think unless the left fork is available; when it is, pick it up;
- think unless the right fork is available; when it is, pick it up;
- when both forks are held, eat for a fixed amount of time;
- put the left fork down;
- put the right fork down;
- repeat from the beginning.

## Models that solve the problem

### Semaphores

```python	
# Define the number of philosophers and forks
num_philosophers = 5
num_forks = num_philosophers
 
# Define semaphores for the forks and the mutex
forks = [threading.Semaphore(1) for i in range(num_forks)]
mutex = threading.Semaphore(1)
 
# Define the philosopher thread function
def philosopher(index):
    while True:
        print(f"Philosopher {index} is thinking...")
        time.sleep(random.randint(1, 5))
         
        mutex.acquire()
         
        left_fork_index = index
        right_fork_index = (index + 1) % num_forks
         
        forks[left_fork_index].acquire()
        forks[right_fork_index].acquire()
         
        mutex.release()
         
        print(f"Philosopher {index} is eating...")
        time.sleep(random.randint(1, 5))
         
        forks[left_fork_index].release()
        forks[right_fork_index].release()
 
# Create a thread for each philosopher
philosopher_threads = []
for i in range(num_philosophers):
    philosopher_threads.append(threading.Thread(target=philosopher, args=(i,)))
     
# Start the philosopher threads
for thread in philosopher_threads:
    thread.start()
     
# Wait for the philosopher threads to complete
for thread in philosopher_threads:
    thread.join()
```
source: [geeksforgeeks](https://www.geeksforgeeks.org/dining-philosopher-problem-using-semaphores/)