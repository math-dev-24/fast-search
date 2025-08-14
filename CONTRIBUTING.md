# 🤝 Guide de contribution - Fast Search

Merci de votre intérêt pour contribuer à Fast Search ! Ce guide vous aidera à démarrer.

## 🌟 Types de contributions

Nous accueillons tous types de contributions :

### 🐛 Signalement de bugs
- Utilisez les [Issues GitHub](https://github.com/math-dev-24/fast-search/issues)
- Utilisez le template de bug report
- Incluez les étapes de reproduction
- Précisez votre environnement (OS, version)

### 💡 Suggestions de fonctionnalités
- Ouvrez une [Discussion](https://github.com/math-dev-24/fast-search/discussions)
- Décrivez le cas d'usage
- Expliquez pourquoi cette fonctionnalité serait utile

### 📝 Documentation
- Améliorez le README
- Ajoutez des exemples d'utilisation
- Corrigez les fautes de frappe
- Traduisez en d'autres langues

### 🔧 Contributions de code
- Corrections de bugs
- Nouvelles fonctionnalités
- Optimisations de performance
- Tests unitaires

## 🚀 Processus de développement

### 1. Préparation
```bash
# Fork le projet sur GitHub
# Cloner votre fork
git clone https://github.com/VOTRE-USERNAME/fast-search.git
cd fast-search

# Installer les dépendances
npm install
cd src-tauri && cargo build && cd ..

# Créer une branche pour votre contribution
git checkout -b feature/ma-nouvelle-fonctionnalite
```

### 2. Développement
```bash
# Lancer en mode développement
npm run tauri dev

# Tests Rust
cd src-tauri && cargo test

# Tests frontend (si configurés)
npm run test

# Linter et formatage
npm run lint
cd src-tauri && cargo fmt && cargo clippy
```

### 3. Soumission
```bash
# Commit vos changements
git add .
git commit -m "feat: ajouter recherche par taille de fichier"

# Push vers votre fork
git push origin feature/ma-nouvelle-fonctionnalite

# Ouvrir une Pull Request sur GitHub
```

## 📋 Standards de code

### Frontend (Vue.js + TypeScript)
- Utilisez la Composition API
- Typez tout avec TypeScript
- Suivez les conventions de nommage Vue.js
- Utilisez Prettier pour le formatage

```typescript
// ✅ Bon
const searchQuery = ref<string>('')
const isLoading = computed(() => store.loading)

// ❌ Éviter
let query = ''
const loading = store.loading
```

### Backend (Rust)
- Utilisez `cargo fmt` pour le formatage
- Corrigez tous les warnings de `cargo clippy`
- Ajoutez des tests pour les nouvelles fonctions
- Documentez les fonctions publiques

```rust
// ✅ Bon
/// Recherche des fichiers selon les critères fournis
pub async fn search_files(query: SearchQuery) -> Result<Vec<File>, Error> {
    // implémentation
}

// ❌ Éviter
pub async fn search(q: SearchQuery) -> Vec<File> {
    // implémentation
}
```

## 🎯 Zones de contribution prioritaires

### 🔥 Haute priorité
- Optimisation des performances de recherche
- Support de nouveaux formats de fichiers
- Tests automatisés
- Documentation API

### 🚀 Moyenne priorité
- Interface utilisateur améliorée
- Raccourcis clavier
- Internationalisation
- Détection automatique de changements

### 💡 Idées bienvenues
- Intégrations avec d'autres outils
- Fonctionnalités d'IA avancées
- Thèmes personnalisés
- Plugins et extensions

## 🧪 Tests

### Tests Rust
```bash
cd src-tauri
cargo test

# Tests avec coverage
cargo tarpaulin --out Html
```

### Tests Frontend
```bash
# Tests unitaires
npm run test

# Tests e2e (si configurés)
npm run test:e2e
```

### Tests d'intégration
- Testez sur différents OS (Windows, macOS, Linux)
- Vérifiez avec de gros volumes de fichiers
- Testez les cas d'erreur

## 📝 Conventions de commit

Utilisez les [Conventional Commits](https://www.conventionalcommits.org/) :

```
type(scope): description

[corps optionnel]

[footer optionnel]
```

### Types
- `feat`: nouvelle fonctionnalité
- `fix`: correction de bug
- `docs`: documentation
- `style`: formatage, espaces
- `refactor`: refactoring de code
- `test`: ajout de tests
- `chore`: tâches de maintenance

### Exemples
```
feat(search): ajouter recherche par regex
fix(ui): corriger l'affichage des icônes sur Windows
docs(readme): mettre à jour les instructions d'installation
refactor(backend): simplifier la logique de scan
```

## 🔍 Review de code

### Checklist pour les PR
- [ ] Les tests passent
- [ ] Le code est formaté correctement
- [ ] La documentation est à jour
- [ ] Les changements sont testés
- [ ] Les commits suivent les conventions
- [ ] La PR a une description claire

### Processus de review
1. **Automatique** : Tests CI/CD
2. **Technique** : Review du code par les mainteneurs
3. **Fonctionnelle** : Tests manuels si nécessaire
4. **Merge** : Squash and merge ou rebase

## 🏗️ Architecture du projet

### Structure frontend
```
src/
├── components/     # Composants réutilisables
├── views/         # Pages de l'application
├── composables/   # Logique réutilisable
├── stores/        # Gestion d'état Pinia
├── types/         # Définitions TypeScript
└── utils/         # Fonctions utilitaires
```

### Structure backend
```
src-tauri/src/
├── entities/      # Modèles de données
├── services/      # Logique métier
├── adapters/      # Couche d'accès aux données
├── ports/         # Interfaces (traits)
└── utils/         # Utilitaires
```

## 🚨 Que faire avant de commencer

1. **Vérifiez les issues existantes** pour éviter les doublons
2. **Discutez des grosses fonctionnalités** avant de commencer
3. **Lisez le code existant** pour comprendre l'architecture
4. **Configurez votre environnement** selon les instructions
5. **Testez localement** avant de soumettre

## 💬 Communication

### Où poser des questions
- [Discussions GitHub](https://github.com/math-dev-24/fast-search/discussions) : Questions générales
- [Issues](https://github.com/math-dev-24/fast-search/issues) : Bugs et fonctionnalités
- [Pull Requests](https://github.com/math-dev-24/fast-search/pulls) : Review de code

### Règles de communication
- Soyez respectueux et constructif
- Aidez les nouveaux contributeurs
- Documentez vos décisions techniques
- Partagez vos retours d'expérience

## 🎉 Reconnaissance

Tous les contributeurs sont ajoutés automatiquement au fichier CONTRIBUTORS.md et apparaissent sur la page GitHub du projet.

### Types de reconnaissance
- **Code** : Votre nom dans les commits et releases
- **Documentation** : Crédit dans le README
- **Tests** : Mention dans les notes de release
- **Support** : Badge "Helpful" sur les discussions

## 📞 Contact des mainteneurs

Pour les questions urgentes ou sensibles :
- **Issues** : Questions publiques
- **Email** : [À définir]
- **Twitter** : [À définir]

---

**Merci de contribuer à Fast Search ! 🚀**

*Ensemble, nous construisons le meilleur outil de recherche de fichiers.*