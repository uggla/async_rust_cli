# Déployer le projet avec DevPod

## Qu'est-ce que DevPod ?

DevPod est une plateforme open-source qui permet de créer et de gérer des environnements de développement reproductibles et portables. Il simplifie le partage d'environnements de développement en conteneurisant votre setup complet (dépendances, outils, configurations) et le rend accessible depuis n'importe quelle machine. Il implémente le standard DevContainer de Microsoft, ce qui le rend compatible avec de nombreux IDEs populaires.

DevPod automatise :
- La création d'environnements de développement isolés
- L'installation des dépendances requises
- La configuration de l'IDE (VS Code, JetBrains, etc.)
- La synchronisation des fichiers avec votre machine locale

## Ouvrir ce projet avec DevPod

### Installation de DevPod

Installez DevPod en suivant la [documentation officielle](https://devpod.sh/docs/getting-started/install).

### Déployer le projet

1. **Ouvrir le projet directement depuis DevPod** :
   ```bash
   devpod up --ide codium --git-clone-recursive-submodules https://github.com/uggla/asyncrustcli
   ```

   Remplacez `codium` par votre IDE préféré si nécessaire (`jetbrains-gateway`, `ssh`, etc.).

2. **Ou cloner le projet localement puis l'ouvrir** :
   ```bash
   git clone --recurse-submodules https://github.com/uggla/asyncrustcli
   cd asyncrustcli
   devpod up --ide codium .
   ```

3. Codium s'ouvrira automatiquement connecté à votre environnement de développement.

## Environnement inclus

L'environnement DevPod de ce projet inclut :
- **Python 3** : Pour la génération des présentations
- **Node.js & npm** : Pour reveal.js
- **Rust** : Pour le projet CLI
- **Git** : Pour la gestion du versioning

## Après le déploiement

Une fois connecté à l'environnement DevPod :

1. Installer les dépendances npm pour reveal.js :
   ```bash
   cd reveal.js
   npm install
   ```

2. Lancer le serveur local :
   ```bash
   ./server.py
   ```

3. Accéder à la présentation via [http://localhost:8000](http://localhost:8000)