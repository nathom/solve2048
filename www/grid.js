// convert the below code to ES6 class syntax:
import Tile from './tile';

export default class Grid {
    constructor(size, previousState)
    {
        this.size = size;
        this.cells =
            previousState ? this.fromState(previousState) : this.empty();
    }

    empty()
    {
        const cells = [];

        for (let x = 0; x < this.size; x++) {
            const row = cells[x] = [];

            for (let y = 0; y < this.size; y++) {
                row.push(null);
            }
        }

        return cells;
    }

    fromState(state)
    {
        const cells = [];

        for (let x = 0; x < this.size; x++) {
            const row = cells[x] = [];

            for (let y = 0; y < this.size; y++) {
                const tile = state[x][y];
                row.push(tile ? new Tile(tile.position, tile.value) : null);
            }
        }

        return cells;
    }

    randomAvailableCell()
    {
        const cells = this.availableCells();

        if (cells.length) {
            return cells[Math.floor(Math.random() * cells.length)];
        }
    }

    availableCells()
    {
        const cells = [];

        this.eachCell((x, y, tile) => {
            if (!tile) {
                cells.push({x: x, y: y});
            }
        });

        return cells;
    }

    eachCell(callback)
    {
        for (let x = 0; x < this.size; x++) {
            for (let y = 0; y < this.size; y++) {
                callback(x, y, this.cells[x][y]);
            }
        }
    }

    cellsAvailable()
    {
        return !!this.availableCells().length;
    }

    cellAvailable(cell)
    {
        return !this.cellOccupied(cell);
    }

    cellOccupied(cell)
    {
        return !!this.cellContent(cell);
    }

    cellContent(cell)
    {
        if (this.withinBounds(cell)) {
            return this.cells[cell.x][cell.y];
        } else {
            return null;
        }
    }

    insertTile(tile)
    {
        this.cells[tile.x][tile.y] = tile;
    }

    removeTile(tile)
    {
        this.cells[tile.x][tile.y] = null;
    }

    withinBounds(position)
    {
        return position.x >= 0 && position.x < this.size && position.y >= 0 &&
            position.y < this.size;
    }
    serialize()
    {
        const cellState = [];

        for (let x = 0; x < this.size; x++) {
            const row = cellState[x] = [];

            for (let y = 0; y < this.size; y++) {
                row.push(
                    this.cells[x][y] ? this.cells[x][y].serialize() : null);
            }
        }

        return {size: this.size, cells: cellState};
    }
}
