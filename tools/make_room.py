import json
import os
import os.path
import pathlib
import sys

template = (
"""let description = `
`;

return #{
    name: "$$NAME",
    description: description,
    position: #{
        x: 0,
        y: 0,
        z: 0
    },
    exits: []
}"""
)

def make_room(data):
    with open(pathlib.Path('game/rooms') / f'{data[0]}.rhai', 'w') as f:

        out = template.replace('$$NAME', data[0].replace('_', ' ').title())

        f.write(out)

if __name__ == '__main__':
    make_room(sys.argv[1:])
