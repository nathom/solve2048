'use strict';

var _documentCurrentScript = typeof document !== 'undefined' ? document.currentScript : null;
let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); }
let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {Int32Array} arr
* @returns {number}
*/
function monte_carlo(arr) {
    const ptr0 = passArray32ToWasm0(arr, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.monte_carlo(ptr0, len0);
    return ret;
}

/**
* @param {Int32Array} arr
* @returns {number}
*/
function expectimax(arr) {
    const ptr0 = passArray32ToWasm0(arr, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.expectimax(ptr0, len0);
    return ret;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {Uint8Array} weights
* @returns {NTuple}
*/
function build_ntuple(weights) {
    const ptr0 = passArray8ToWasm0(weights, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.build_ntuple(ptr0, len0);
    return NTuple.__wrap(ret);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {NTuple} net
* @param {Int32Array} arr
* @returns {number}
*/
function ntuple(net, arr) {
    _assertClass(net, NTuple);
    const ptr0 = passArray32ToWasm0(arr, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ntuple(net.__wbg_ptr, ptr0, len0);
    return ret;
}

/**
* @param {Int32Array} arr
* @returns {number}
*/
function random_available_move(arr) {
    const ptr0 = passArray32ToWasm0(arr, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.random_available_move(ptr0, len0);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

const NTupleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ntuple_free(ptr >>> 0));
/**
*/
class NTuple {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(NTuple.prototype);
        obj.__wbg_ptr = ptr;
        NTupleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NTupleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ntuple_free(ptr);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('solve2048_bg.wasm', (typeof document === 'undefined' ? require('u' + 'rl').pathToFileURL(__filename).href : (_documentCurrentScript && _documentCurrentScript.src || new URL('index.js', document.baseURI).href)));
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

// convert to ES6 class syntax:

class Tile {
    constructor(position, value)
    {
        this.x = position.x;
        this.y = position.y;
        this.value = value || 2;

        this.previousPosition = null;
        this.mergedFrom = null;  // Tracks tiles that merged together
    }

    savePosition()
    {
        this.previousPosition = {x: this.x, y: this.y};
    }

    updatePosition(position)
    {
        this.x = position.x;
        this.y = position.y;
    }

    serialize()
    {
        return {position: {x: this.x, y: this.y}, value: this.value};
    }
}

// convert the below code to ES6 class syntax:

class Grid {
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

// convert ALL the code to ES6 class syntax:
class HTMLActuator {
    constructor()
    {
        this.tileContainer = document.querySelector('.tile-container');
        this.scoreContainer = document.querySelector('.score-container');
        this.bestContainer = document.querySelector('.best-container');
        this.messageContainer = document.querySelector('.game-message');
        this.score = 0;
    }
    actuate(grid, metadata)
    {
        const self = this;
        window.requestAnimationFrame(() => {
            self.clearContainer(self.tileContainer);
            grid.cells.forEach(column => {
                column.forEach(cell => {
                    if (cell) {
                        self.addTile(cell);
                    }
                });
            });
            self.updateScore(metadata.score);
            self.updateBestScore(metadata.bestScore);
            if (metadata.terminated) {
                if (metadata.over) {
                    self.message(false);
                } else if (metadata.won) {
                    self.message(true);
                }
            }
        });
    }
    continueGame()
    {
        this.clearMessage();
    }
    clearContainer(container)
    {
        while (container.firstChild) {
            container.removeChild(container.firstChild);
        }
    }
    addTile(tile)
    {
        const self = this;
        const wrapper = document.createElement('div');
        const inner = document.createElement('div');
        const position = tile.previousPosition || {x: tile.x, y: tile.y};
        const positionClass = this.positionClass(position);
        const classes = ['tile', `tile-${tile.value}`, positionClass];
        if (tile.value > 2048) {
            classes.push('tile-super');
        }
        this.applyClasses(wrapper, classes);
        inner.classList.add('tile-inner');
        inner.textContent = tile.value;
        if (tile.previousPosition) {
            window.requestAnimationFrame(() => {
                classes[2] = self.positionClass({x: tile.x, y: tile.y});
                self.applyClasses(wrapper, classes);
            });
        } else if (tile.mergedFrom) {
            classes.push('tile-merged');
            this.applyClasses(wrapper, classes);
            tile.mergedFrom.forEach(merged => {
                self.addTile(merged);
            });
        } else {
            classes.push('tile-new');
            this.applyClasses(wrapper, classes);
        }
        wrapper.appendChild(inner);
        this.tileContainer.appendChild(wrapper);
    }
    applyClasses(element, classes)
    {
        element.setAttribute('class', classes.join(' '));
    }
    normalizePosition(position)
    {
        return {x: position.x + 1, y: position.y + 1};
    }
    positionClass(position)
    {
        position = this.normalizePosition(position);
        return `tile-position-${position.x}-${position.y}`;
    }
    updateScore(score)
    {
        this.clearContainer(this.scoreContainer);
        const difference = score - this.score;
        this.score = score;
        this.scoreContainer.textContent = this.score;
        if (difference > 0) {
            const addition = document.createElement('div');
            addition.classList.add('score-addition');
            addition.textContent = `+${difference}`;
            this.scoreContainer.appendChild(addition);
        }
    }
    updateBestScore(bestScore)
    {
        this.bestContainer.textContent = bestScore;
    }
    message(won)
    {
        const type = won ? 'game-won' : 'game-over';
        const message = won ? 'You win!' : 'Game over!';
        this.messageContainer.classList.add(type);
        this.messageContainer.getElementsByTagName('p')[0].textContent =
            message;
    }
    clearMessage()
    {
        this.messageContainer.classList.remove('game-won');
        this.messageContainer.classList.remove('game-over');
    }
}

async function downloadFile(url, progressCallback)
{
    if (!url) {
        alert('No URL provided');
        return;
    }
    const response = await fetch(url);
    const contentLength = parseInt(
        response.headers.get('content-length'),
        10);  // Parse content-length as integer
    const reader = response.body.getReader();
    let receivedLength = 0;  // Track the total number of bytes downloaded
    let chunks = [];         // Array to store downloaded chunks

    while (true) {
        const {done, value} = await reader.read();

        if (done) {
            break;
        }

        chunks.push(value);
        receivedLength += value.length;
        progressCallback(receivedLength, contentLength);
    }

    // Concatenate all the downloaded chunks into a single Uint8Array
    const totalLength = chunks.reduce((acc, chunk) => acc + chunk.length, 0);
    let offset = 0;
    const uint8Array = new Uint8Array(totalLength);

    for (const chunk of chunks) {
        uint8Array.set(chunk, offset);
        offset += chunk.length;
    }

    return uint8Array;
}

class KeyboardInputManager {
    constructor()
    {
        this.events = {};

        if (window.navigator.msPointerEnabled) {
            // Internet Explorer 10 style
            this.eventTouchstart = 'MSPointerDown';
            this.eventTouchmove = 'MSPointerMove';
            this.eventTouchend = 'MSPointerUp';
        } else {
            this.eventTouchstart = 'touchstart';
            this.eventTouchmove = 'touchmove';
            this.eventTouchend = 'touchend';
        }

        this.delay = 100;
        this.weightsUrl = null;

        const progressContainer = document.querySelector('.progress-container');
        this.msPerMove = document.getElementById('ms-per-move');
        this.showNTupleFooter = () => {
            progressContainer.style.display = 'flex';
        };
        this.hideNTupleFooter = () => {
            progressContainer.style.display = 'none';
        };

        this.sliderHandler();
        this.listen();
    }

    on(event, callback)
    {
        if (!this.events[event]) {
            this.events[event] = [];
        }
        this.events[event].push(callback);
    }

    emit(event, data)
    {
        const callbacks = this.events[event];
        if (callbacks) {
            callbacks.forEach(callback => callback(data));
        }
    }

    listen()
    {
        const map = {
            38: 0,  // Up
            39: 1,  // Right
            40: 2,  // Down
            37: 3,  // Left
            75: 0,  // Vim up
            76: 1,  // Vim right
            74: 2,  // Vim down
            72: 3,  // Vim left
            87: 0,  // W
            68: 1,  // D
            83: 2,  // S
            65: 3   // A
        };

        // Respond to direction keys
        document.addEventListener('keydown', event => {
            const modifiers = event.altKey || event.ctrlKey || event.metaKey ||
                event.shiftKey;
            const mapped = map[event.which];

            if (!modifiers) {
                if (mapped !== undefined) {
                    event.preventDefault();
                    this.emit('move', mapped);
                }
            }

            // R key restarts the game
            if (!modifiers && event.which === 82) {
                this.restart.call(this, event);
            }
        });

        // Respond to button presses
        this.bindButtonPress('.retry-button', this.restart);
        this.bindButtonPress('.restart-button', this.restart);
        this.bindButtonPress('.keep-playing-button', this.keepPlaying);
        this.bindButtonPress('.random-move-button', this.randomMove);

        // Respond to swipe events
        const gameContainer =
            document.getElementsByClassName('game-container')[0];
        // continue converting to ES6 classes

        gameContainer.addEventListener(this.eventTouchstart, event => {
            if ((!window.navigator.msPointerEnabled &&
                 event.touches.length > 1) ||
                event.targetTouches.length > 1) {
                return;  // Ignore if touching with more than 1 finger
            }

            if (window.navigator.msPointerEnabled) {
                touchStartClientX = event.pageX;
                touchStartClientY = event.pageY;
            } else {
                touchStartClientX = event.touches[0].clientX;
                touchStartClientY = event.touches[0].clientY;
            }

            event.preventDefault();
        });

        gameContainer.addEventListener(this.eventTouchmove, event => {
            event.preventDefault();
        });

        gameContainer.addEventListener(this.eventTouchend, event => {
            if ((!window.navigator.msPointerEnabled &&
                 event.touches.length > 0) ||
                event.targetTouches.length > 0) {
                return;  // Ignore if still touching with one or more fingers
            }

            let touchEndClientX, touchEndClientY;

            if (window.navigator.msPointerEnabled) {
                touchEndClientX = event.pageX;
                touchEndClientY = event.pageY;
            } else {
                touchEndClientX = event.changedTouches[0].clientX;
                touchEndClientY = event.changedTouches[0].clientY;
            }

            const dx = touchEndClientX - touchStartClientX;
            const absDx = Math.abs(dx);

            const dy = touchEndClientY - touchStartClientY;
            const absDy = Math.abs(dy);

            if (Math.max(absDx, absDy) > 10) {
                // (right : left) : (down : up)
                this.emit(
                    'move',
                    absDx > absDy ? (dx > 0 ? 1 : 3) : (dy > 0 ? 2 : 0));
            }
        });
    };
    restart(event)
    {
        event.preventDefault();
        this.emit('restart');
    };
    keepPlaying(event)
    {
        event.preventDefault();
        this.emit('keepPlaying');
    };

    randomMove(event)
    {
        event.preventDefault();
        this.emit('randomMove');
    };

    bindButtonPress(selector, fn)
    {
        const button = document.querySelector(selector);
        button.addEventListener('click', fn.bind(this));
        button.addEventListener(this.eventTouchend, fn.bind(this));
    };

    handleDropdownEvent(callback)
    {
        const dropdownContent = document.querySelector('.dropdown-content');
        const dropdownBtn = document.querySelector('.dropbtn');

        dropdownBtn.addEventListener('mouseenter', function() {
            dropdownContent.style.display = 'block';
        });
        dropdownBtn.addEventListener('click', function() {
            dropdownContent.style.display = 'block';
        });

        document.addEventListener('click', function(event) {
            if (!dropdownContent.contains(event.target) &&
                !dropdownBtn.contains(event.target)) {
                dropdownContent.style.display = 'none';
            }
        });

        dropdownContent.addEventListener('click', event => {
            // Check if the clicked element is a dropdown item (an <a> tag)
            if (event.target.tagName === 'A') {
                // Prevent the default action (e.g., navigating to a URL)
                event.preventDefault();

                // Get the id of the selected item
                const selectedItem = event.target.id;
                console.log('selected ', selectedItem);
                callback(selectedItem);

                // dropdownContent.classList.remove('show');
                dropdownContent.style.display = 'none';
            }
        });
    };


    setWeightsUrl(url)
    {
        this.weightsUrl = url;
    }

    shakeAgentsButton()
    {
        // Get the button element
        let button = document.querySelector('.dropbtn');

        // Add event listener to trigger the shaking effect
        // Add the CSS class to apply the shake animation
        button.classList.add('shake-animation');

        // After a short delay, remove the CSS class to stop the animation
        setTimeout(function() {
            button.classList.remove('shake-animation');
        }, 500);  // Duration of the shake animation (0.5s)
    }

    shakeActivateButton()
    {
        // Get the button element
        let button = document.querySelector('.random-move-button');

        // Add event listener to trigger the shaking effect
        // Add the CSS class to apply the shake animation
        button.classList.add('shake-animation');

        // After a short delay, remove the CSS class to stop the animation
        setTimeout(function() {
            button.classList.remove('shake-animation');
        }, 500);  // Duration of the shake animation (0.5s)
    }


    shakeProgressBar()
    {
        let bar = document.querySelector('.progress-container');

        bar.classList.add('shake-animation');

        setTimeout(function() {
            bar.classList.remove('shake-animation');
        }, 500);  // Duration of the shake animation (0.5s)
    }

    setSelectedMode(mode)
    {
        // Get the dropdown button element
        var dropdownButton = document.querySelector('.dropbtn');

        // Set the text of the dropdown button to the selected mode but
        // capitalize the first letter

        let text = 'Select';
        switch (mode) {
        case 'montecarlo':
            text = 'Monte Carlo';
            break;
        case 'expectimax':
            text = 'Expectimax';
            break;
        case 'ntuple':
            text = 'N-Tuple';
            if (this.weightsUrl === null) {
                console.error('Weights URL is not set');
                this.shakeAgentsButton();
            }
            break;
        case 'random':
            text = 'Random';
            break;
        }

        dropdownButton.textContent = text;
    }

    activationButtonOn()
    {
        // Get the activation button element
        var button = document.querySelector('.random-move-button');

        // Set the button text to 'Deactivate'
        button.textContent = 'Deactivate';
    }

    activationButtonOff()
    {
        // Get the activation button element
        var button = document.querySelector('.random-move-button');

        // Set the button text to 'Deactivate'
        button.textContent = 'Activate';
    }

    // toggleActivationButton()
    // {
    //     // Get the activation button element
    //     var button = document.querySelector('.random-move-button');
    //
    //     // Toggle the button text between 'Activate' and 'Deactivate'
    //     if (button.textContent === 'Activate') {
    //         button.textContent = 'Deactivate';
    //     } else {
    //         button.textContent = 'Activate';
    //     }
    // }



    getSelectedDelay()
    {
        return this.delay;
    }

    sliderHandler()
    {
        // Get the range input element
        let delayRange = document.getElementById('delay-range');
        // Get the label element
        let delayLabel = document.getElementById('delay-label');

        // Add event listener to the range input
        delayRange.addEventListener('input', () => {
            // Update the label text content with the new value of the range
            // pad value with spaces to the left
            delayLabel.textContent = delayRange.value;
            this.delay = delayRange.value;
        });
    }

    downloadWeights()
    {
        let progressBar = document.getElementById('download-bar');
        let progressText = document.getElementById('download-progress-label');
        return downloadFile(
            this.weightsUrl, (receivedLength, contentLength) => {
                this.setDownloadProgress(
                    receivedLength, contentLength, progressBar, progressText);
            });
    }

    setDownloadProgress(
        receivedLength, contentLength, progressBar, progressText)
    {
        const receivedLengthMB = Math.round(receivedLength / 1024 / 1024);
        const contentLengthMB = Math.round(contentLength / 1024 / 1024);

        progressBar.style.width =
            (100.0 * receivedLength / contentLength) + '%';
        progressText.textContent =
            `Downloading Weights (${receivedLengthMB}/${contentLengthMB} MB): `;

        if (receivedLength == contentLength) {
            let progressColor = document.querySelector('.color');
            progressColor.style.backgroundColor = 'green';
        }
    }

    startBuildingNetwork()
    {
        let progressText = document.getElementById('download-progress-label');
        progressText.textContent = `Building network... `;
    }

    doneBuildingNetwork()
    {
        let progressText = document.getElementById('download-progress-label');
        progressText.textContent = `Done! `;
    }

    /**
     * setMsPerMove.
     *
     * @param {number} time
     */
    setMsPerMove(time)
    {
        let rtime;
        if (time < 1) {
            rtime = time.toFixed(2);
        } else {
            rtime = Math.round(time);
        }
        this.msPerMove.textContent = rtime;
    }
}

window.fakeStorage = {
    _data: {},

    setItem: function(id, val) {
        return this._data[id] = String(val);
    },

    getItem: function(id) {
        return this._data.hasOwnProperty(id) ? this._data[id] : undefined;
    },

    removeItem: function(id) {
        return delete this._data[id];
    },

    clear: function() {
        return this._data = {};
    }
};

// convert the below code to ES6 class syntax:
class LocalStorageManager {
    constructor()
    {
        this.bestScoreKey = 'bestScore';
        this.gameStateKey = 'gameState';
        const supported = this.localStorageSupported();
        this.storage = supported ? window.localStorage : window.fakeStorage;
    }
    localStorageSupported()
    {
        const testKey = 'test';
        try {
            const storage = window.localStorage;
            storage.setItem(testKey, '1');
            storage.removeItem(testKey);
            return true;
        } catch (error) {
            return false;
        }
    }
    getBestScore()
    {
        return this.storage.getItem(this.bestScoreKey) || 0;
    }
    setBestScore(score)
    {
        this.storage.setItem(this.bestScoreKey, score);
    }
    getGameState()
    {
        const stateJSON = this.storage.getItem(this.gameStateKey);
        return stateJSON ? JSON.parse(stateJSON) : null;
    }
    setGameState(gameState)
    {
        this.storage.setItem(this.gameStateKey, JSON.stringify(gameState));
    }
    clearGameState()
    {
        this.storage.removeItem(this.gameStateKey);
    }
}

// convert the below code to ES6 class syntax:
/**
 * GameManager.
 */
class GameManager {
    constructor(size, weightsUrl)
    {
        this.size = size;
        this.inputManager = new KeyboardInputManager;
        this.storageManager = new LocalStorageManager;
        this.actuator = new HTMLActuator;
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
        await this.playGame(arr => monte_carlo(arr));
    }

    async playExpectimax()
    {
        await this.playGame(arr => expectimax(arr));
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
                this.tupleNetwork = build_ntuple(weights);
                this.inputManager.doneBuildingNetwork();
            }
        }
        // another coroutine is awaiting the weights and building network
        if (this.mode == 'ntuple' && this.tupleNetwork === null) {
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
            _ => random_available_move(this.boardAsArray()));
    }

    async playNtuple()
    {
        if (this.tupleNetwork === null) return;
        await this.playGame(arr => ntuple(this.tupleNetwork, arr));
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

function init_game(wasm_path)
{
    const weights_url =
        'https://huggingface.co/nathom/ntuple-2048/resolve/main/tuplenet_4M_lr.bin';
    __wbg_init(wasm_path).then(() => {
        // let weights_promise = downloadFile(weights_url);

        // Wait till the browser is ready to render the game (avoids glitches)
        window.requestAnimationFrame(function() {
            new GameManager(4, weights_url);
        });
    });
}

if (wasm_path === undefined) {
    alert('Please specify the path to the wasm file');
} else {
    init_game(wasm_path);
}
