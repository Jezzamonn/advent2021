const fs = require('fs');
const path = require('path');

const filename = path.join(__dirname, 'input.txt');

const lines = fs.readFileSync(filename, 'utf8').trim().split('\n');
const numBits = lines[0].length;

function mostCommonBitAtPosition(binaryStrings, position) {
    let numOnes = 0;
    for (const line of binaryStrings) {
        if (line[position] === '1') {
            numOnes++;
        }
    }
    const mostCommon = 2 * numOnes >= binaryStrings.length ? '1' : '0';
    const count = mostCommon === '1' ? numOnes : binaryStrings.length - numOnes;
    return {mostCommon, count};
}

let oxygenRatingLines = lines.slice();
for (let b = 0; b < numBits; b++) {
    const {mostCommon, count} = mostCommonBitAtPosition(oxygenRatingLines, b);
    if (count === oxygenRatingLines.length) {
        continue;
    }
    oxygenRatingLines = oxygenRatingLines.filter(line => line[b] === mostCommon);

    if (oxygenRatingLines.length === 1) {
        break;
    }
}
console.log(oxygenRatingLines);
const oxygenRating = parseInt(oxygenRatingLines[0], 2);

let co2RatingLines = lines.slice();
for (let b = 0; b < numBits; b++) {
    const {mostCommon, count} = mostCommonBitAtPosition(co2RatingLines, b);
    co2RatingLines = co2RatingLines.filter(line => line[b] !== mostCommon);

    if (co2RatingLines.length === 1) {
        break;
    }
}
console.log(co2RatingLines);
const co2Rating = parseInt(co2RatingLines[0], 2);

console.log(oxygenRating * co2Rating);