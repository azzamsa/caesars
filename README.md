<div align="center">
<h1>Caesars</h1>
<img src='docs/padlock.svg' width=80px/>

Encrypt and decrypt your secrets using ROT algorithm 🔐

</div>

![demo](https://github.com/user-attachments/assets/0603d140-1480-49ed-92dc-6d9eda3dc34b)

---

## Rationale

The project started to find a frontend framework that suit my need.
For backend, I found Axum is the most appealing currently.

After some research, now the result is live: [Is Rust Ready for Web Front-End?](https://azzamsa.com/n/rust-frontend/).

## Navigating the Code

All the features can be viewed in the [CHANGELOG](CHANGELOG.md) file tagged with `feat`.
The file only contains user-facing changes, so you won't get lost bisecting the features.

## Web

- 🦀 [Dioxus](frontend/dioxus)
- 🦀 [Leptos](frontend/leptos)
- 🎩 [Vue 3](frontend/vue)
- 🎩 [Svelte](frontend/svelte)

## Usage

Start the backend.

```sh
$ cd backend
$ just setup && just dev
```

Go to the playground `http://127.0.0.1:8000/playground` to see the schema.

After starting the backend. Choose the frontend framework you want to see.

```sh
$ cd frontend/favorite-frontend
$ just setup && just dev
```

## Credits

- [Noto Emoji](https://github.com/googlefonts/noto-emoji)
