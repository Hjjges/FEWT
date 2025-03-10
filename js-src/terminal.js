import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

let terminal;
let currentCommand = '';

console.log("SCREAMING FOR HELP RAGH");

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
            background: 'black',
            foreground: '#f0f0f0'
        },
        fontSize: 12
    });

    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);

    terminal.open(document.getElementById(containerId));
    fitAddon.fit();

    window.addEventListener('resize', () => fitAddon.fit());

    let string = `
    \x1b[31m██████╗ \x1b[31m██╗ \x1b[31m██████╗ \x1b[31m██╗  ██╗\x1b[0m\x1b[31m██╗   ██╗\x1b[31m███████╗\x1b[0m\r\n
    \x1b[31m██╔══██╗\x1b[31m██║\x1b[31m██╔═══██╗\x1b[31m╚██╗██╔╝\x1b[0m\x1b[31m██║   ██║\x1b[31m██╔════╝\x1b[0m\r\n
    \x1b[31m██║  ██║\x1b[31m██║\x1b[31m██║   ██║\x1b[31m ╚███╔╝\x1b[0m\x1b[31m ██║   ██║\x1b[31m███████╗\x1b[0m\r\n
    \x1b[31m██║  ██║\x1b[31m██║\x1b[31m██║   ██║\x1b[31m ██╔██╗\x1b[0m\x1b[31m ██║   ██║\x1b[31m╚════██║\x1b[0m\r\n
    \x1b[31m██████╔╝\x1b[31m██║\x1b[31m╚██████╔╝\x1b[31m██╔╝ ██╗\x1b[0m\x1b[31m╚██████╔╝\x1b[31m███████║\x1b[0m\r\n
    \x1b[31m╚═════╝ \x1b[31m╚═╝ \x1b[31m╚═════╝ \x1b[31m╚═╝  ╚═╝\x1b[0m\x1b[31m╚═════╝ \x1b[31m╚══════╝\x1b[0m\r\n
    `;

    terminal.write(string);
    terminal.write('Dioxus Terminal Emulator - v0.1\r\n');
    terminal.write('==> ');

    terminal.onData(data => {

        console.log("attempting to send new message");
        window.dioxusBridge.sendToRust("Hi from JS!");
        terminal.write('\r\n');
        terminal.write('==> ');
        currentCommand = '';


        // if (data === '\r') {
        //     terminal.write('\r\n');
        //     terminal.write('$ ');
        //     currentCommand = '';
        // } else if (data === '\x7f' && currentCommand.length > 0) {
        //     currentCommand = currentCommand.slice(0, -1);
        //     terminal.write('\b \b');
        // } else if (data >= ' ' && data <= '~') {
        //     currentCommand += data;
        //     terminal.write(data);
        // }
    })

    // implement writing mechanics later
    // (async function() {{
    //     while (true) {{
    //         const output = await dioxus.recv();
    //         terminal.write(output + '\r\n');
    //         terminal.write('==> ');
    //     }}
    // }})();
}

window.displayTerminalOutput = function(output) {
    terminal.write(output + '\r\n');
    terminal.write('==> ');
}