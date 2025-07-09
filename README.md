# Fast Search

Une application de recherche rapide de fichiers construite avec Tauri, Vue.js 3 et Rust.

## Fonctionnalités

- 🔍 **Recherche avancée** : Recherche de fichiers et dossiers avec filtres par type
- 📊 **Statistiques en temps réel** : Vue d'ensemble des fichiers indexés
- 🎯 **Interface moderne** : UI élégante avec Naive UI et Tailwind CSS
- ⚡ **Performance optimisée** : Backend Rust pour des performances maximales
- 💾 **Base de données SQLite** : Persistance des données d'indexation
- 🔄 **Synchronisation** : Indexation automatique des dossiers sélectionnés
- 📁 **Navigation** : Ouverture des fichiers dans l'explorateur système
- 🎨 **Design responsive** : Interface adaptée à tous les écrans

## Technologies utilisées

### Frontend
- **Vue.js 3** - Framework JavaScript progressif
- **TypeScript** - Typage statique
- **Naive UI** - Composants UI modernes
- **Tailwind CSS** - Framework CSS utilitaire
- **Pinia** - Gestion d'état
- **Vue Router** - Routage côté client
- **Vite** - Build tool rapide

### Backend
- **Rust** - Langage système performant
- **Tauri** - Framework pour applications desktop
- **SQLite** - Base de données légère
- **Rusqlite** - Driver SQLite pour Rust
- **Walkdir** - Parcours de répertoires
- **Chrono** - Gestion des dates

## Installation

### Prérequis

- **Node.js** (version 18 ou supérieure)
- **Rust** (dernière version stable)
- **Tauri CLI** : `npm install -g @tauri-apps/cli`

### Installation des dépendances

```bash
# Cloner le repository
git clone <repository-url>
cd fast-search

# Installer les dépendances frontend
npm install

# Installer les dépendances Rust
cd src-tauri
cargo build
cd ..
```

## Utilisation

### Développement

```bash
# Lancer l'application en mode développement
npm run tauri dev
```

### Production

```bash
# Construire l'application
npm run tauri build
```

## Guide d'utilisation

### 1. Synchronisation des dossiers

1. Cliquez sur "Synchroniser" dans l'interface
2. Sélectionnez le dossier à indexer
3. L'application scanne récursivement tous les fichiers et dossiers
4. Les données sont stockées dans la base SQLite locale

### 2. Recherche de fichiers

1. Utilisez la barre de recherche pour trouver des fichiers
2. Filtrez par type de fichier (optionnel)
3. Choisissez de rechercher dans les dossiers ou fichiers
4. Les résultats s'affichent en temps réel

### 3. Navigation

- **Fichiers** : Double-cliquez pour ouvrir dans l'application par défaut
- **Dossiers** : Double-cliquez pour ouvrir dans l'explorateur
- **Statistiques** : Consultez les métriques d'indexation

## Architecture

### Frontend (Vue.js 3)

```
src/
├── components/          # Composants réutilisables
│   ├── CardFile.vue    # Carte d'affichage des fichiers
│   ├── CardFolder.vue  # Carte d'affichage des dossiers
│   ├── Filter.vue      # Composant de filtrage
│   ├── Header.vue      # En-tête de l'application
│   ├── Search.vue      # Barre de recherche
│   └── Setting.vue     # Paramètres
├── views/              # Pages de l'application
│   ├── Home.vue        # Page principale
│   └── Statistique.vue # Page des statistiques
├── shared/             # Code partagé
│   ├── store/          # Stores Pinia
│   │   └── search.ts   # Store de recherche
│   ├── pathFormat.ts   # Utilitaires de formatage
│   └── sieFormat.ts    # Formatage des tailles
├── types/              # Types TypeScript
│   ├── file.ts         # Interface File
│   ├── stat.ts         # Interface Stat
│   └── setting.ts      # Interface Setting
└── route.ts            # Configuration du routage
```

### Backend (Rust + Tauri)

```
src-tauri/src/
├── lib.rs              # Point d'entrée et commandes Tauri
├── entities/           # Modèles de données
│   ├── file.rs         # Entité File
│   └── stat.rs         # Entité Stat
├── adapters/           # Couche d'accès aux données
│   └── repository/     # Implémentations des repositories
│       └── sqlite.rs   # Repository SQLite
├── services/           # Logique métier
│   └── file_service.rs # Service de gestion des fichiers
├── ports/              # Interfaces (traits)
│   └── repository.rs   # Interface Repository
└── utils/              # Utilitaires
    ├── collect.rs      # Collecte de fichiers
    └── generator.rs    # Génération des services
```

## API Backend

### Commandes Tauri disponibles

- `get_stat()` - Récupère les statistiques globales
- `sync_files_and_folders(path: String)` - Synchronise un dossier
- `search_files(search, types, is_dir, folders)` - Recherche de fichiers
- `get_type_files()` - Liste des types de fichiers
- `open_file_in_explorer(path: String)` - Ouvre un fichier
- `reset_data()` - Réinitialise la base de données
- `get_all_folders()` - Récupère tous les dossiers

## Structure de la base de données

```sql
-- Table des fichiers
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    file_type TEXT,
    size INTEGER,
    last_modified TEXT,
    created_at TEXT
);

-- Table des dossiers
CREATE TABLE folders (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL
);
```

## Résolution des problèmes

### Problèmes courants

1. **Chemin invalide**
   - Vérifiez que le chemin existe et est accessible
   - Utilisez des chemins absolus (ex: `C:\Users\Documents`)

2. **Permissions insuffisantes**
   - L'application peut ne pas avoir accès aux dossiers système
   - Évitez les dossiers protégés comme `C:\Windows`

3. **Base de données verrouillée**
   - Fermez l'application et relancez-la
   - Supprimez le fichier `db.sqlite` pour repartir de zéro

4. **Erreurs de compilation Rust**
   - Vérifiez que Rust est à jour : `rustup update`
   - Nettoyez le cache : `cargo clean`

### Logs et débogage

- **Développement** : Console du navigateur (F12)
- **Production** : Console de l'application
- **Rust** : Logs dans la console de développement

## Développement

### Ajouter une nouvelle fonctionnalité

1. **Backend** : Ajoutez la commande dans `src-tauri/src/lib.rs`
2. **Frontend** : Créez l'interface dans `src/views/`
3. **Types** : Définissez les types dans `src/types/`
4. **Store** : Ajoutez la logique dans `src/shared/store/`

### Tests

```bash
# Tests Rust
cd src-tauri
cargo test

# Tests frontend (si configurés)
npm run test
```

### Scripts disponibles

```bash
npm run dev          # Développement frontend uniquement
npm run build        # Build frontend
npm run tauri dev    # Développement complet
npm run tauri build  # Build production
```

## Contribution

1. Fork le projet
2. Créez une branche pour votre fonctionnalité
3. Committez vos changements
4. Poussez vers la branche
5. Ouvrez une Pull Request

## TODO

- Dans les settings possibilité d'avoir plusieurs sources
- Ajout de filtre :
   - Poids
   - ...
- ...

## Licence

MIT License - voir le fichier LICENSE pour plus de détails.
