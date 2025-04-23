# Dioxus System Explorer Integrated with a Terminal Emulator

This project aims to replicate the standard functionality of a file explorer, programmed in rust, whilst also providing custom utilities that one could leverage in a system administration environment to enhance workflow.

An example is the integrated terminal emulator which is fully synced with the program itself to show live updates, permissions and commands.

### Notes

Run this to start the app

```bash
dx serve
```

To run for a different platform (which is untested and not sure what will happen), use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

