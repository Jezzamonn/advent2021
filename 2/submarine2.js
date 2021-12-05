const fs = require('fs');
const path = require('path');

const filename = path.join(__dirname, 'input.txt');

const lines = fs.readFileSync(filename, 'utf8').split('\n');

let horizontalPosition = 0;
let aim = 0;
let depth = 0;
for (const line of lines) {
    const [command, distStr] = line.split(' ');
    const distance = parseInt(distStr);
    switch (command) {
        case 'forward':
            horizontalPosition += distance;
            depth += aim * distance;
            break;
        case 'up':
            aim -= distance;
            break;
        case 'down':
            aim += distance;
            break;
    }
}

console.log(depth * horizontalPosition);