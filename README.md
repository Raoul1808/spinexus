# Spinexus
A custom SpinShare client to download custom charts and mods, written in Rust with Dioxus and Tailwind.

**This project is currently a work in progress.** The current focus is to get a functional application before working on the UI (kind of, I get sidetracked a lot :P), so it is preferable to stick with the official SpinShare client.

This project is meant to be a simple program that uses the SpinShare API to deliver custom songs. It is by no means created to compete against or replace the official client.

## Building
Prerequisites:
- Latest stable Rust version
- Cargo (duh)
- Platform-specific packages installed. Visit [Dioxus' website](https://dioxuslabs.com/learn/0.4/getting_started/desktop) for more info.
- Tailwind CSS CLI (any form, I personally use the standalone CLI program)

Steps:
1. Clone this repository (`git clone https://github.com/Raoul1808/spinexus.git`)
2. cd into the directory (`cd spinexus`)
3. Run the file `tailwind.sh` (or the command inside if you can't run a shell file for some reason)
4. Build the project (`cargo build`)
5. Run the executable (or type `cargo run`)

## License

This project is licensed under the MIT License.
