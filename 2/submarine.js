const fs = require('fs');
const path = require('path');

const filename = path.join(__dirname, 'input.txt');

const lines = fs.readFileSync(filename, 'utf8').split('\n');

let depth = 0;
let horizontalPosition = 0;
for (const line of lines) {
    const [command, distStr] = line.split(' ');
    const distance = parseInt(distStr);
    switch (command) {
        case 'forward':
            horizontalPosition += distance;
            break;
        case 'up':
            depth -= distance;
            break;
        case 'down':
            depth += distance;
            break;
    }
}

console.log(depth * horizontalPosition);