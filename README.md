# PDG-Tournoix

Tournoix est une application web de gestion de tournoi. 

## Spécifications

### Requirements fonctionnels

### Requirements non-fonctionnels

### Structure de l'application

## Lancer l'application

**Pré-requis**: 
- rustc >= 1.71
- cargo >= 1.71

1. Ajouter la target wasm32

```rustup target add wasm32-unknown-unknown```

2. Installer Trunk

```cargo install --locked trunk```



3. Lancer en développement

3a. Lancer le backend (API)

```
cd ./backend
cargo run
```

3b. Lancer le frontend en dev
```
cd ./ui
trunk serve
```

4. Build en release

```
npm run build
cd ./backend
cargo run --release
```