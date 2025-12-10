# ☁️ Déploiement sur Google Cloud Run

Ce guide explique comment déployer le serveur de Poker (Rust + UI) sur Google Cloud Run.

## Prérequis

- Un projet Google Cloud Platform actif
- [Google Cloud SDK (gcloud)](https://cloud.google.com/sdk/docs/install) installé et configuré
- Docker (optionnel pour build local)

## 1. Initialisation (Si c'est la première fois)

Connectez-vous à votre compte Google Cloud :

```bash
gcloud auth login
gcloud config set project [VOTRE_PROJET_ID]
```

Activez les services nécessaires :

```bash
gcloud services enable cloudbuild.googleapis.com run.googleapis.com containerregistry.googleapis.com
```

## 2. Déploiement Robuste (Build puis Deploy)

La méthode automatique `--source` peut parfois échouer avec des erreurs de nommage d'image. Voici la méthode recommandée :

### Étape 2.1 : Construire l'image

Nous allons construire l'image Docker explicitement via Google Cloud Build et la stocker dans le registre.

```bash
gcloud builds submit --tag gcr.io/poker-480809/4sh-poker .
```

### Étape 2.2 : Déployer le service

Une fois l'image construite, nous la déployons sur Cloud Run.

```bash
gcloud run deploy poker-server \
  --image gcr.io/poker-480809/4sh-poker \
  --platform managed \
  --region europe-west1 \
  --allow-unauthenticated
```
