import * as wasm from 'solve2048';

import Grid from './grid';
import Actuator from './html_actuator';
import InputManager from './keyboard_input_manager';
import StorageManager from './local_storage_manager';

// convert the below code to ES6 class syntax:
/**
 * GameManager.
 */
export default class GameManager {
    constructor(size, weightsUrl)
    {
        this.size = size;
        this.inputManager = new InputManager;
        this.storageManager = new StorageManager;
        this.actuator = new Actuator;
        this.startTiles = 2;
        this.botPlaying = false;
        this.delay = 100;
        this.inputManager.on('move', this.move.bind(this));
        this.inputManager.on('restart', this.restart.bind(this));
        this.inputManager.on('keepPlaying', this.keepPlaying.bind(this));
        this.inputManager.on('randomMove', this.toggleAgent.bind(this));
        this.inputManager.handleDropdownEvent(
            this.handleDropdownEvent.bind(this));
        this.inputManager.setWeightsUrl(weightsUrl);
        this.mode = 'random';

        this.cancelRequest = false;
        // get weights as blob of bytes from
        // https://huggingface.co/nathom/ntuple-2048/resolve/main/tuplenet_4M_lr.bin
        this.weightsUrl = weightsUrl;
        this.weightsPromise = null;
        this.downloadingWeights = false;
        this.tupleNetwork = null;
        this.awaitedWeights = false;

        this.perMoveEMA = 0;

        this.setup();
    }

    handleDropdownEvent(mode)
    {
        if (this.botPlaying) {
            this.inputManager.shakeAgentsButton();
            console.warn('Cannot change mode while bot is playing');
            return;
        }
        let prev_mode = this.mode;
        switch (mode) {
        case 'random-item':
            this.mode = 'random';
            break;
        case 'expectimax-item':
            this.mode = 'expectimax';
            break;
        case 'monte-carlo-item':
            this.mode = 'montecarlo';
            break;
        case 'ntuple-item':
            this.mode = 'ntuple';
            this.inputManager.showNTupleFooter();
            if (this.weightsPromise === null) {
                this.weightsPromise = this.inputManager.downloadWeights();
            }
            break;
        default:
            this.mode = 'default';
        }
        if (this.mode !== prev_mode) {
            this.perMoveEMA = 0;
        }
        this.inputManager.setSelectedMode(this.mode);
    }

    restart()
    {
        if (this.botPlaying) {
            this.inputManager.shakeActivateButton();
            return;
        }
        this.storageManager.clearGameState();
        this.actuator.continueGame();
        this.perMoveEMA = 0;
        this.setup();
    }
    keepPlaying()
    {
        this.keepPlaying = true;
        this.actuator.continueGame();
    }

    isGameTerminated()
    {
        return this.over || (this.won && !this.keepPlaying);
    }
    setup()
    {
        const previousState = this.storageManager.getGameState();
        if (previousState) {
            this.grid =
                new Grid(previousState.grid.size, previousState.grid.cells);
            this.score = previousState.score;
            this.over = previousState.over;
            this.won = previousState.won;
            this.keepPlaying = previousState.keepPlaying;
        } else {
            this.grid = new Grid(this.size);
            this.score = 0;
            this.over = false;
            this.won = false;
            this.keepPlaying = false;
            this.addStartTiles();
        }
        this.actuate();
    }


    addStartTiles()
    {
        for (let i = 0; i < this.startTiles; i++) {
            this.addRandomTile();
        }
    }

    addRandomTile()
    {
        if (this.grid.cellsAvailable()) {
            const value = Math.random() < 0.9 ? 2 : 4;
            const tile = new Tile(this.grid.randomAvailableCell(), value);
            this.grid.insertTile(tile);
        }
    }

    actuate()
    {
        if (this.storageManager.getBestScore() < this.score) {
            this.storageManager.setBestScore(this.score);
        }
        if (this.over) {
            this.storageManager.clearGameState();
        } else {
            this.storageManager.setGameState(this.serialize());
        }
        this.actuator.actuate(this.grid, {
            score: this.score,
            over: this.over,
            won: this.won,
            bestScore: this.storageManager.getBestScore(),
            terminated: this.isGameTerminated()
        });
    }

    serialize()
    {
        return {
            grid: this.grid.serialize(),
            score: this.score,
            over: this.over,
            won: this.won,
            keepPlaying: this.keepPlaying
        };
    }

    prepareTiles()
    {
        this.grid.eachCell((_x, _y, tile) => {
            if (tile) {
                tile.mergedFrom = null;
                tile.savePosition();
            }
        });
    }
    moveTile(tile, cell)
    {
        this.grid.cells[tile.x][tile.y] = null;
        this.grid.cells[cell.x][cell.y] = tile;
        tile.updatePosition(cell);
    }

    move(direction)
    {
        if (this.isGameTerminated()) {
            return;
        }
        const self = this;
        const vector = this.getVector(direction);
        const traversals = this.buildTraversals(vector);
        let moved = false;
        this.prepareTiles();
        traversals.x.forEach(x => {
            traversals.y.forEach(y => {
                const cell = {x, y};
                const tile = self.grid.cellContent(cell);
                if (tile) {
                    const positions = self.findFarthestPosition(cell, vector);
                    const next = self.grid.cellContent(positions.next);
                    if (next && next.value === tile.value && !next.mergedFrom) {
                        const merged = new Tile(positions.next, tile.value * 2);
                        merged.mergedFrom = [tile, next];
                        self.grid.insertTile(merged);
                        self.grid.removeTile(tile);
                        tile.updatePosition(positions.next);
                        self.score += merged.value;
                        if (merged.value === 2048) {
                            self.won = true;
                        }
                    } else {
                        self.moveTile(tile, positions.farthest);
                    }
                    if (!self.positionsEqual(cell, tile)) {
                        moved = true;
                    }
                }
            });
        });
        if (moved) {
            this.addRandomTile();
            if (!this.movesAvailable()) {
                this.over = true;
            }
            this.actuate();
        }
    }

    async playGame(agent)
    {
        if (this.botPlaying) return;
        // play until we have lost the game
        // or the user has requested to stop
        // if the win screen shows, keep playing if the player chooses to
        while (!this.over) {
            this.botPlaying = true;
            const arr = this.boardAsArray();
            const start = performance.now();
            const move = agent(arr);
            const moveTime = performance.now() - start;
            this.perMoveEMA = moveTime * 0.05 + this.perMoveEMA * 0.95;
            this.inputManager.setMsPerMove(this.perMoveEMA);
            if (move === -1) {
                this.perMoveEMA = 0;
                break;
            }
            // time taken to make a move
            this.move(move);
            const timeTaken = performance.now() - start;

            console.log('Average time: ', this.perMoveEMA, 'ms');

            if (this.cancelRequest) {
                this.cancelRequest = false;
                break;
            }
            while (this.won && !this.keepPlaying) {
                await new Promise(r => setTimeout(r, 10));
            }
            await new Promise(r => setTimeout(r, this.getDelay(timeTaken)));
        }
        this.botPlaying = false;
    }

    async playMonteCarlo()
    {
        await this.playGame(arr => wasm.monte_carlo(arr));
    }

    async playExpectimax()
    {
        await this.playGame(arr => wasm.expectimax(arr));
    }

    getDelay(timeTaken)
    {
        const selectedDelay = this.inputManager.getSelectedDelay();
        return Math.max(selectedDelay - timeTaken, 0);
    }

    async toggleAgent()
    {
        if (this.botPlaying) {
            await this.cancelAgent();
        } else {
            await this.activateAgent();
        }
    }

    async cancelAgent()
    {
        if (!this.botPlaying) return;
        this.cancelRequest = true;
        while (this.botPlaying) {
            await new Promise(r => setTimeout(r, 10));
        }
    }

    async activateAgent()
    {
        if (this.botPlaying) return;
        if (this.mode === 'default') {
            this.inputManager.shakeAgentsButton();
            return;
        }
        if (this.mode == 'ntuple' && this.tupleNetwork === null) {
            // ensure only one copy of the weights are ever downloaded
            if (this.weightsPromise === null) {
                console.error('Weights promise is null, recovering');
                // This should never run since weightsPromise is set
                // when the mode is set to 'ntuple'
                // Leaving it just in case
                this.weightsPromise = this.inputManager.downloadWeights();
            }
            // only one copy of the network should be built
            if (!this.awaitedWeights) {
                this.awaitedWeights = true;
                let weights = await this.weightsPromise;
                this.inputManager.startBuildingNetwork();
                this.tupleNetwork = wasm.build_ntuple(weights);
                this.inputManager.doneBuildingNetwork();
            }
        }
        // another coroutine is awaiting the weights and building network
        if (this.tupleNetwork === null) {
            this.inputManager.shakeProgressBar();
            return;
        }

        this.inputManager.activationButtonOn();
        if (this.mode === 'expectimax') {
            await this.playExpectimax();
        } else if (this.mode === 'montecarlo') {
            await this.playMonteCarlo();
        } else if (this.mode === 'ntuple') {
            await this.playNtuple();
        } else if (this.mode === 'random') {
            await this.playRandom();
        }
        this.inputManager.activationButtonOff();
    }

    async playRandom()
    {
        await this.playGame(
            _ => wasm.random_available_move(this.boardAsArray()));
    }

    async playNtuple()
    {
        if (this.tupleNetwork === null) return;
        await this.playGame(arr => wasm.ntuple(this.tupleNetwork, arr));
    }

    boardAsArray()
    {
        let board = [];
        for (let i = 0; i < this.size; i++) {
            for (let j = 0; j < this.size; j++) {
                let tile = this.grid.cells[j][i];
                board.push(tile ? Math.log2(tile.value) : 0);
            }
        }
        return board;
    }

    getVector(direction)
    {
        const map = {
            0: {x: 0, y: -1},  // Up
            1: {x: 1, y: 0},   // Right
            2: {x: 0, y: 1},   // Down
            3: {x: -1, y: 0}   // Left
        };
        return map[direction];
    }

    buildTraversals(vector)
    {
        const traversals = {x: [], y: []};
        for (let pos = 0; pos < this.size; pos++) {
            traversals.x.push(pos);
            traversals.y.push(pos);
        }
        if (vector.x === 1) traversals.x = traversals.x.reverse();
        if (vector.y === 1) traversals.y = traversals.y.reverse();
        return traversals;
    }

    findFarthestPosition(cell, vector)
    {
        let previous;
        do {
            previous = cell;
            cell = {x: previous.x + vector.x, y: previous.y + vector.y};
        } while (this.grid.withinBounds(cell) && this.grid.cellAvailable(cell));
        return {farthest: previous, next: cell};
    }

    movesAvailable()
    {
        return this.grid.cellsAvailable() || this.tileMatchesAvailable();
    }

    tileMatchesAvailable()
    {
        let tile;
        for (let x = 0; x < this.size; x++) {
            for (let y = 0; y < this.size; y++) {
                tile = this.grid.cellContent({x, y});
                if (tile) {
                    for (let direction = 0; direction < 4; direction++) {
                        const vector = this.getVector(direction);
                        const cell = {x: x + vector.x, y: y + vector.y};
                        const other = this.grid.cellContent(cell);
                        if (other && other.value === tile.value) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    positionsEqual(first, second)
    {
        return first.x === second.x && first.y === second.y;
    }
}
import Tile from './tile';
