window.dioxusBridge = {
    toRustQueue: [],
    fromRustQueue: [],
    waitCallbacks: [],

    sendToRust: function(message) {
        this.toRustQueue.push(message);
    },

    receiveFromRust: function() {
        return new Promise((resolve) => {
            if (this.fromRustQueue.length > 0) {
                resolve(this.fromRustQueue.shift());
            } else {
                this.waitCallbacks.push(resolve);
            }
        });
    }
}

console.log("Rust Macro - Dioxus Bridge Initialized");