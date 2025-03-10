import { init, Wasmer } from "https://unpkg.com/@wasmer/sdk@latest/dist/index.mjs";

    async function runPython() {
        await init();

        const packageName = "python/python";
        const pkg = await Wasmer.fromRegistry(packageName);
        const instance = await pkg.entrypoint.run({
            args: ["-c", "print('Hello, World!')"],
        });

        const { code, stdout } = await instance.wait();

        console.log(`Python exited with ${code}: ${stdout}`);
    }

runPython();