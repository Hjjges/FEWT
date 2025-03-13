let startX;
let sideWidth;
let appWidth;

function initResizeA(e) {
    e.preventDefault();
    startX = e.clientX;
    appWidth =  document.querySelector(".app-container").offsetWidth;
    sideWidth = document.querySelector(".side-bar").offsetWidth;
    document.addEventListener('mousemove', resizeA);
    document.addEventListener('mouseup', stopResizeA);
}

function resizeA(e) {
    const diffX = e.clientX - startX;
    const newWidth = (sideWidth + diffX)
    const maxWidth = window.innerWidth * 0.5;
    const constrainedWidth = Math.max(180, Math.min(maxWidth, newWidth));
    document.querySelector(".side-bar").style.width = `${constrainedWidth}px`
    document.querySelector(".app-container").style.gridTemplateColumns = `${constrainedWidth}px`;

}

function stopResizeA() {
    document.removeEventListener('mousemove', resizeA);
    document.removeEventListener('mouseup', stopResizeA);
}

document.querySelector(".resizer-side").addEventListener('mousedown', initResizeA);