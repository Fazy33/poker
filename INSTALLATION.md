# Guide d'Installation de Rust

Ce guide vous aidera à installer Rust sur votre système macOS.

## Installation

### Méthode 1: Installation Standard (Recommandée)

Ouvrez votre terminal et exécutez :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Suivez les instructions à l'écran. L'installation par défaut est généralement la meilleure option.

### Après l'Installation

Rechargez votre configuration shell :

```bash
source $HOME/.cargo/env
```

Ou fermez et rouvrez votre terminal.

### Vérification

Vérifiez que Rust est correctement installé :

```bash
rustc --version
cargo --version
```

Vous devriez voir quelque chose comme :
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

## Utilisation du Projet

Une fois Rust installé, vous pouvez :

### Compiler le Projet

```bash
cd /Users/mmuller/dev/4sh/4sh-poker
cargo build --workspace
```

### Exécuter les Tests

```bash
cargo test --workspace
```

### Exécuter l'Exemple de Démonstration

```bash
cargo run --example demo
```

### Vérifier le Code

```bash
# Vérifier la syntaxe sans compiler
cargo check

# Formater le code
cargo fmt

# Linter
cargo clippy
```

## Prochaines Étapes

Après avoir installé Rust et vérifié que le projet compile :

1. Explorez le code dans `poker-engine/src/`
2. Exécutez les tests pour voir le moteur en action
3. Lancez l'exemple de démonstration
4. Commencez à développer le serveur web et l'API

## Ressources

- [Documentation Rust](https://doc.rust-lang.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
