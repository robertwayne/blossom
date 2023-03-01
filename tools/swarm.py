import pathlib
import random
import string
import sys
import time
from multiprocessing import Process
from telnetlib import ECHO, IAC, WILL, WONT, Telnet

PROCESS_COUNT = 6
WAIT_TIME = 0.5

names = []
commands = ['n', 'e', 's', 'w', 'd', 'u', 'say hello', 'global hello', 'who', 'afk', 'brief', 'look']

def random_string():
    name = ''.join(random.choice(string.ascii_letters) for _ in range(6))

    while name in names:
        name = random_string()

    with open('tools/swarm_names.txt', 'a') as f:
        f.write(name + '\n')

    return name

def create_swarm(n):
    print(f'Creating {n} players.')

    # Creates a pool of n connections.
    connections = [Telnet('localhost', 5000) for _ in range(int(n))]

    # For each connection, create a new player with the name `Swarm_n` and the
    # password `password`.
    for i, tn in enumerate(connections):
        random_name = random_string()

        name = f'{random_name}'
        time.sleep(1)
        tn.write(name.encode('ascii') + b'\r\n')

        time.sleep(1)
        response = 'y'
        tn.write(response.encode('ascii') + b'\r\n')

        time.sleep(1)
        tn.get_socket().send(IAC + WONT + ECHO)

        time.sleep(1)
        password = 'password'
        tn.write(password.encode('ascii') + b'\r\n')

        time.sleep(1)
        tn.get_socket().send(IAC + WILL + ECHO)

        tn.write('quit'.encode('ascii') + b'\r\n')
        time.sleep(1)

def run_swarm(chunk):
    print(f'Swarming the server with {len(chunk)} players.')

    # Creates a pool of n connections.
    connections = [Telnet('localhost', 5000) for _ in range(len(chunk))]

    # For each connection, logs in with the name `Swarm_n` and the password
    # `password`. Note that you need to have run `python swarm.py create n`
    # before running this.
    for i, tn in enumerate(connections):
        time.sleep(1)
        name = names[i]
        tn.write(name.encode('ascii') + b'\r\n')

        print(f'Logging in as {name}.')

        time.sleep(1)
        tn.get_socket().send(IAC + WONT + ECHO)

        time.sleep(1)
        password = 'password'
        tn.write(password.encode('ascii') + b'\r\n')

        print(f'Logged in as {name}.')

        time.sleep(1)
        tn.get_socket().send(IAC + WILL + ECHO)

    print('All players logged in.')

    # For each connection, randomly sends a commands (from the list at the top
    # of this file).
    while True:
        for c in connections:
            dir = random.choice(commands)
            c.write(dir.encode('ascii') + b'\r\n')
            time.sleep(WAIT_TIME)
        
def print_missing_args_error(name):
    print(f'Error: Missing argument. \
    \nUsage: python swarm.py {name} <n> \
    \n\nSee python swarm.py --help for more information.')

    sys.exit(1)


if __name__ == '__main__':
    if pathlib.Path('tools/swarm_names.txt').is_file():
        with open('tools/swarm_names.txt', 'r') as f:
            names = f.read().splitlines()

    if sys.argv[1:] == [] or sys.argv[1] == '--help' or sys.argv[1] == '-h':
        print('Blossom Swarm - Load testing for the Blossom MUD engine. \
        \n\nUsage: \
        \n  python swarm.py <option> \
        \n \
        \nOptions: \
        \n  create <n> - spawns n connections that send random commands \
        \n  run <n> - spawn n connections and creates a new player for each one \
        \n')
        sys.exit(0)
    
    elif sys.argv[1] == 'create':
        if len(sys.argv) != 3:
            print_missing_args_error('create')

        chunk_size = int(sys.argv[2]) // PROCESS_COUNT

        for _ in range(PROCESS_COUNT):
            Process(target=create_swarm, args=(chunk_size,)).start()

        sys.exit(0)
        
    elif sys.argv[1] == 'run':
        if len(sys.argv) != 3:
            print_missing_args_error('run')

        names = names[:int(sys.argv[2])]
        chunk_size = len(names) // PROCESS_COUNT
        chunks = [names[i:i + chunk_size] for i in range(0, len(names), chunk_size)]

        if len(chunks) > PROCESS_COUNT:
            last_chunk = chunks.pop()
            chunks[-1].append(last_chunk)


        for chunk in chunks:
            Process(target=run_swarm, args=(chunk,)).start()
            
        sys.exit(0)
