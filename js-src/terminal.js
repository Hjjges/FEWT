import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

let terminal;
let currentCommand = '';
const fitAddon = new FitAddon();
window.fitAddon = fitAddon;


window.initTerminal = function(containerId) {

    if (!window.dioxusBridge) {
        console.error("Dioxus bridge not found. Retrying in 500ms...");
        setTimeout(initializeTerminal, 500);
        return;
    }

    console.log("Initializing terminal with Dioxus bridge");

    terminal = new Terminal({
        cursorBlink: true,
        theme: {
            background: '#181818',
            foreground: '#f0f0f0'
        },
        fontSize: 12
    });

    terminal.loadAddon(fitAddon);

    terminal.open(document.getElementById(containerId));
    fitAddon.fit();

    window.addEventListener('resize', () => fitAddon.fit());

    let string = `
    \x1b[38;2;0;120;213m██████╗\x1b[38;2;0;120;213mm██╗ \x1b[38;2;0;120;213m██████╗ \x1b[38;2;0;120;213m██╗  ██╗\x1b[0m\x1b[38;2;0;120;213m██╗   ██╗\x1b[38;2;0;120;213m███████╗\x1b[0m\r\n
    \x1b[38;2;0;120;213m██╔══██╗\x1b[38;2;0;120;213m██║\x1b[38;2;0;120;213m██╔═══██╗\x1b[38;2;0;120;213m╚██╗██╔╝\x1b[0m\x1b[38;2;0;120;213m██║   ██║\x1b[38;2;0;120;213m██╔════╝\x1b[0m\r\n
    \x1b[38;2;0;120;213m██║  ██║\x1b[38;2;0;120;213m██║\x1b[38;2;0;120;213m██║   ██║\x1b[38;2;0;120;213m ╚███╔╝\x1b[0m\x1b[38;2;0;120;213m ██║   ██║\x1b[38;2;0;120;213m███████╗\x1b[0m\r\n
    \x1b[38;2;0;120;213m██║  ██║\x1b[38;2;0;120;213m██║\x1b[38;2;0;120;213m██║   ██║\x1b[38;2;0;120;213m ██╔██╗\x1b[0m\x1b[38;2;0;120;213m ██║   ██║\x1b[38;2;0;120;213m╚════██║\x1b[0m\r\n
    \x1b[38;2;0;120;213m██████╔╝\x1b[38;2;0;120;213m██║\x1b[38;2;0;120;213m╚██████╔╝\x1b[38;2;0;120;213m██╔╝ ██╗\x1b[0m\x1b[38;2;0;120;213m╚██████╔╝\x1b[38;2;0;120;213m███████║\x1b[0m\r\n
    \x1b[38;2;0;120;213m╚═════╝ \x1b[38;2;0;120;213m╚═╝ \x1b[38;2;0;120;213m╚═════╝ \x1b[38;2;0;120;213m╚═╝  ╚═╝\x1b[0m\x1b[38;2;0;120;213m╚═════╝ \x1b[38;2;0;120;213m╚══════╝\x1b[0m\r\n
    `;

    terminal.write(string);
    terminal.write('Dioxus Terminal Emulator - v0.1\r\n');
    terminal.write('$ ');

    terminal.onData(data => {

        // pressing enter needs to be async because 
        if (data === '\r') {
            window.dioxusBridge.sendToRust(currentCommand);
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

    (async function() {{
        while (true) {{
            const output = await window.dioxusBridge.receiveFromRust();
            terminal.write(output + '\r\n');
            terminal.write('==> ');
        }}
    }})();
}

window.displayTerminalOutput = function(output) {
    terminal.write(output + '\r\n');
    terminal.write('$ ');
}

window.fitTerminal = function() {
    fitAddon.fit();
}

window.hideTerminal = function(containerId) {
    document.getElementById(containerId).classList.remove('show');
}

window.showTerminal = function(containerId) {
    document.getElementById(containerId).classList.add('show');
}