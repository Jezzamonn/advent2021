const fs = require('fs');
const path = require('path');

function solveProblem() {
    const filename = path.join(__dirname, 'input.txt');
    const groups = fs.readFileSync(filename, 'utf8').trim().split('\n\n');
    console.log(groups);

    const numbers = groups[0].split(',').map(x => parseInt(x));
    console.log(numbers);

    const boards = groups.slice(1).map(board => board.trim().split(/\s+/).map(x => parseInt(x)));

    console.log(findBingo(boards, numbers));
}

function findBingo(boards, numbers) {
    const boardMatches = boards.map(board => board.slice().fill(false));
    const boardColSums = boards.map(_ => Array(5).fill(0));
    const boardRowSums = boards.map(_ => Array(5).fill(0));

    for (const num of numbers) {
        for (let i = 0; i < boards.length; i++) {
            const board = boards[i];
            const pos = board.indexOf(num);
            if (pos === -1) continue;

            const col = pos % 5;
            const row = Math.floor(pos / 5);
            boardMatches[i][pos] = true;
            boardColSums[i][col]++;
            boardRowSums[i][row]++;
        }

        for (let i = 0; i < boards.length; i++) {
            const board = boards[i];
            let hasMatch = false;
            for (const colSum of boardColSums[i]) {
                if (colSum === 5) {
                    hasMatch = true;
                    break;
                }
            }
            for (const rowSum of boardRowSums[i]) {
                if (rowSum === 5) {
                    hasMatch = true;
                    break;
                }
            }
            if (hasMatch) {
                return getScore(board, boardMatches[i], num);
            }
        }
    }
}

function getScore(board, boardMatches, lastNum) {
    let score = 0;
    for (let i = 0; i < board.length; i++) {
        if (!boardMatches[i]) {
            score += board[i];
        }
    }
    return score * lastNum;
}



solveProblem();