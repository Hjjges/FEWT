async function processToRustQueue() {
    while (true) {
        if (window.dioxusBridge && window.dioxusBridge.toRustQueue.length > 0) {
            const message = window.dioxusBridge.toRustQueue.shift();
            console.log("Sending to Rust:", message);
            dioxus.send(message); // utilising dioxus rust macro context
        }
        await new Promise(resolve => setTimeout(resolve, 100));
    }
}
processToRustQueue();