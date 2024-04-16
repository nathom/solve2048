import GameManager from './game_manager';
import HTMLActuator from './html_actuator';
import KeyboardInputManager from './keyboard_input_manager';
import LocalStorageManager from './local_storage_manager';
import init, * as wasm from 'solve2048';

function init_game(wasm_path)
{
    init(wasm_path).then(() => {
        let weights_promise =
            fetch(
                'https://huggingface.co/nathom/ntuple-2048/resolve/main/tuplenet_4M_lr.bin')
                .then(r => r.arrayBuffer())
                .then(b => new Uint8Array(b));

        // Wait till the browser is ready to render the game (avoids glitches)
        window.requestAnimationFrame(function() {
            new GameManager(
                4, KeyboardInputManager, HTMLActuator, LocalStorageManager,
                weights_promise, wasm);
        });
    });
}

if (wasm_path === undefined) {
    alert('Please specify the path to the wasm file');
} else {
    init_game(wasm_path);
}
