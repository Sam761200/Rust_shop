# RustShop

RustShop est une application de commerce électronique basée sur Rust, conçue pour permettre aux utilisateurs de parcourir les produits, d'en ajouter à un panier, et de les acheter. Cette version initiale offre des fonctionnalités de base via une API RESTful.

## Fonctionnalités

### Gestion des Produits:

L'API permet de récupérer les produits disponibles en se connectant à une base de données de démonstration. Des routes pour ajouter, mettre à jour, et supprimer des produits peuvent être ajoutées ultérieurement.
API RESTful: L'API permet des interactions faciles et standardisées pour les clients web.

## Installation

### Prérequis:

Assurez-vous d'avoir installé Rust et Cargo.
Une base de données SQLite est également nécessaire pour la gestion des produits.

## Clone du dépôt:

Clonez le dépôt et naviguez dans le répertoire:

```bash
git clone https://github.com/votre_utilisateur/rust_shop.git
cd rust_shop
```

### Configuration de la base de données:

Créez un fichier .env à la racine du projet, contenant la ligne suivante:

```bash
DATABASE_URL=rustshop.db
```

Cela permettra à l'application de se connecter à la base de données SQLite.

### Lancement de l'application:

Compilez et exécutez l'application:

```bash
cargo run
```

## Utilisation

Produits disponibles: Accédez à http://127.0.0.1:3030/products pour voir les produits disponibles.

Gestion de la base de données: Actuellement, la base de données est initialisée avec des produits de démonstration, mais elle peut être étendue avec des fonctions CRUD complètes.

## Structure du Projet

#### main.rs:

Contient la logique principale pour lancer le serveur et initialiser les produits de démonstration.

#### routes/:

mod.rs: Définit les routes principales pour la gestion des produits.

#### Développement Futur :

- Gestion CRUD Complète: Étendre les routes pour permettre une gestion complète des produits.

- Gestion des Utilisateurs: Ajouter la gestion des utilisateurs et des fonctionnalités d'authentification.

- Panier d'Achat: Permettre aux utilisateurs d'ajouter des produits à un panier et de passer commande.
