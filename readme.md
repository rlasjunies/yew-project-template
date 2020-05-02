# Yew project template

> [inspired by the github project `lenote-mirror`](https://github.com/snluu/lenote-mirror)

`cargo web` does not seems to be trendy, Rust webassembly community are moving toward `web-sys` and there is not yet CLI providing the convenience of `cargo web`.

I do not plan to bundle the solution made in Rust Webassembly via Yew.

So here is a basic template to simplify our life!  

This template come with:

- [x] it's own server project,
- [x] it's own static site with index.html, javascript to load the webassembly
- [x] structure to compile the file at the good place

## How to use

> Rust & wasm are dependencies for this project

- download the project

```bash
git clone https://github.com/rlasjunies/yew-project-template

```

- build the server project

```bash
cd server
cargo run
```

- build the spa project

In another command line

```bash
wasm-pack build --debug --no-typescript --out-name spa --target web
```

- open your webbrowser at the URL <http://127.0.0.1::8080>

Enjoy!

## What's next

- [ ] build both project in one command line
- [ ] file watching
- [ ] web portal auto refresh

## Reminder

If you are using vs-code with the `rls-vscode` extension, do not forget to add the folder in vscode `Workspace`

> `File > Add Folder to Workspace...` select the main folder of the template not `spa` neither `server`
