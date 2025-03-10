import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

let terminal;
let currentCommand = '';

console.log("SCREAMING FOR HELP RAGH");

window.initTerminal = function(containerId) {
    terminal = new Terminal({
        cursorBlink: true,
        theme: {
            background: 'black',
            foreground: '#f0f0f0'
        }
    });

    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);

    terminal.open(document.getElementById(containerId));
    fitAddon.fit();

    window.addEventListener('resize', () => fitAddon.fit());
    terminal.write('Dioxus Terminal Emulator - v0.1\r\n');
    terminal.write('$ ');

    terminal.onData(data => {
        if (data === '\r') {
            terminal.write('\r\n');
            terminal.write('$ ');
            currentCommand = '';
        } else if (data === '\x7f' && currentCommand.length > 0) {
            currentCommand = currentCommand.slice(0, -1);
            terminal.write('\b \b');
        } else if (data >= ' ' && data <= '~') {
        currentCommand += data;
        terminal.write(data);
        }
    })

    // implement writing mechanics later
}

window.displayTerminalOutput = function(output) {
    terminal.write(output + '\r\n');
    terminal.write('$ ');
}