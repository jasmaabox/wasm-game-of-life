import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Drawing constants
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Create universe
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Get DOM
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const gliderBut = document.getElementById('glider-but');
gliderBut.onclick = () => {
    universe.generate_glider();
}

/**
 * Get index from grid row and column
 * @param {*} row 
 * @param {*} column 
 */
const getIndex = (row, column) => {
    return row * width + column;
};

/**
 * Draws grid on canvas
 */
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

/**
 * Draw cells on canvas
 */
const drawCells = () => {
    const cellsPtr = universe.cells(); // Pointer to where cells are in memory
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height); // Load cells from wasm linear memory

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

/**
 * Main game loop
 */
const renderLoop = () => {

    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);