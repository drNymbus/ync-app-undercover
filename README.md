
# Undercover — Un jeu de société en Rust avec Iced

Un petit projet Rust pour apprendre à utiliser la crate [`iced`](https://github.com/iced-rs/iced), en développant une version numérique du célèbre jeu de société *Undercover*.

## Règles du jeu

Chaque joueur reçoit un mot :
- Les **civils** partagent tous **le même mot**.
- Les **Undercover(s)** ont un mot **similaire mais différent**.
- Optionnellement, un joueur peut être le **Mr White**, qui ne connaît **aucun mot** et doit deviner ceux des autres (À partir de 4 joueurs).

Au fil des tours, chaque joueur donne un indice verbal lié à son mot. Le but :
- Pour les **civils** : identifier les infiltrés.
- Pour les **Undercover(s)** : se fondre dans la masse.
- Pour le **Mr White** : ne pas se faire repérer! S'il vient à être demasquer, il aura l'occasion de gagner en devinant le mot secret des **civils**.

## Objectif du projet

Ce projet est conçu comme un **bac à sable pédagogique** pour :
- Explorer la programmation UI avec la crate [`iced`](https://github.com/iced-rs/iced)
- Comprendre les concepts de base de **Rust** (ownership, enums, structs, etc.)
- Structurer une application avec une architecture simple et modulaire

## Lancer le projet

### 1. Prérequis

- [Rust installé](https://www.rust-lang.org/tools/install)
- `cargo` disponible

### 2. Cloner et lancer

```bash
git clone https://github.com/ton-pseudo/undercover-rust.git
cd undercover-rust
cargo run
```
