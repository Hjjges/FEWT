let startY;
let bottomHeight;
let appHeight;

function initResize(e) {
    e.preventDefault();
    startY = e.clientY;
    appHeight =  document.querySelector(".app-container").offsetHeight;
    bottomHeight = document.querySelector(".bottom-component").offsetHeight;
    document.addEventListener('mousemove', resize);
    document.addEventListener('mouseup', stopResize);
}

function resize(e) {
    const diffY = startY - e.clientY;
    const bottomHeightPercent = ((bottomHeight + diffY) / appHeight) * 100;
    const constrainedHeight = Math.max(10, Math.min(80, bottomHeightPercent));
    document.querySelector(".app-container").style.gridTemplateRows = `[first-row] 10% [second-row] auto [buttons] 60px [resizer] ${constrainedHeight}%`;
}

function stopResize() {
    // On stopping resize, update the terminal size to parent div size using fitAddon
    window.fitAddon.fit();
    window.fitTerminal();
    document.removeEventListener('mousemove', resize);
    document.removeEventListener('mouseup', stopResize);
}

document.querySelector(".resizer-bottom").addEventListener('mousedown', initResize);