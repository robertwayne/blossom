# Tools

To run the tools, you'll need to have a modern version of Python3 installed. To
run, use the command `python <tool>.py`. You can view a tools help menu with
`python <tool>.py --help`.

<!-- markdownlint-disable -->
| Name      | File                                | Description                                |
|-----------|-------------------------------------|------------------------------------------- |
| swarm     | [swarm.py](/tools/swarm.py)         | Load testing and command 'fuzzing' script. |
| make_room | [make_room.py](/tools/make_room.py) | Generates a formatted room file.           |
<!-- markdownlint-enable -->

## Using Swarm

In order to use swarm, you must first run `python swarm.py create n`, where `n`
is how many players it will create. They will NOT be logged in when finished.

After that, you can run `python swarm.py run n`, where `n` is the same number
before - or the total amount of players you have created as a whole, to start
the swarm. It will take a bit to log them all in, depending on how many players
you are running.

All the players you create are stored in `swarm_names.json` in the `tools`
directory. This must be cleared if you wipe the database, otherwise the script
will try logging in with players that no longer exist.
