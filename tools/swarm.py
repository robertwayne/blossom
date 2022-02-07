import pathlib
import random
import string
import sys
import time
from multiprocessing import Process
from telnetlib import ECHO, IAC, WILL, WONT, Telnet

PROCESS_COUNT = 6

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

    # For each connection, create a new player with the name `Swarm_n` and the password `password`.
    for i, tn in enumerate(connections):
        random_name = random_string()

        tn.read_until(b'What is your name? If you are new, enter the name you wish to use.')
        name = f'{random_name}'
        tn.write(name.encode('ascii') + b'\r\n')

        time.sleep(0.1)
        tn.read_until(b'Character not found. Create a new character with this name? [Y/n]')
        response = 'y'
        tn.write(response.encode('ascii') + b'\r\n')

        time.sleep(0.1)
        tn.get_socket().send(IAC + WONT + ECHO)

        time.sleep(0.1)
        tn.read_until(b'What will your password be?')
        password = 'password'
        tn.write(password.encode('ascii') + b'\r\n')

        time.sleep(0.1)
        tn.get_socket().send(IAC + WILL + ECHO)

        tn.write('quit'.encode('ascii') + b'\r\n')
        time.sleep(0.1)

def run_swarm(chunk):
    print(f'Swarming the server with {len(chunk)} players.')

    # Creates a pool of n connections.
    connections = [Telnet('localhost', 5000) for _ in range(len(chunk))]

    # For each connection, logs in with the name `Swarm_n` and the password `password`. Note that
    # you need to have run `python swarm.py create n` before running this.
    for i, tn in enumerate(connections):
        tn.read_until(b'What is your name? If you are new, enter the name you wish to use.')
        name = names[i]
        tn.write(name.encode('ascii') + b'\r\n')

        time.sleep(0.1)
        tn.get_socket().send(IAC + WONT + ECHO)

        time.sleep(0.1)
        tn.read_until(b'What is your password?')
        password = 'password'
        tn.write(password.encode('ascii') + b'\r\n')

        time.sleep(0.1)
        tn.get_socket().send(IAC + WILL + ECHO)

    # For each connection, randomly sends a commands (from the list at the top of this file).
    while True:
        for c in connections:
            dir = random.choice(commands)
            c.write(dir.encode('ascii') + b'\r\n')
            time.sleep(0.1)
        
def print_missing_args_error(name):
    print(f'Error: Missing argument. \
    \nUsage: python swarm.py {name} <n> \
    \n\nSee python swarm.py --help for more information.')


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
