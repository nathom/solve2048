import {nodeResolve} from '@rollup/plugin-node-resolve';
import {wasm} from '@rollup/plugin-wasm';

export default {
    input: 'index.js',
    output: {dir: 'output', format: 'cjs'},
    plugins: [nodeResolve(), wasm()]
};
