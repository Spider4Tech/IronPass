# IronPass


1. Architecture Globale
   
## Frontend (Web)

Framework : Yew (Rust) 
Stockage local : IndexedDB pour les données chiffrées
Communication : WebSocket ou REST API chiffrée (TLS 1.3)

## Backend (Rust)

Framework : Actix-web 
Base de données : PostgreSQL (pour les métadonnées, logs, coffres partagés)
Authentification : JWT + Argon2id pour le hachage des mots de passe maîtres

## Chiffrement

Horizon  : Chiffrement des mots de passe stockés
Argon2id : Dérivation de clé à partir du mot de passe maître
Zero Knowledge : Le serveur ne connaît jamais les mots de passe en clair


2. Système de Chiffrement
Flux de chiffrement/déchiffrement

 - Mot de passe maître → Argon2id → Clé principale (64 octets)
 - Clé principale → Horizon → Chiffrement des mots de passe
 - Stockage : Seuls les données chiffrées sont envoyées au serveur


détection leak mp via ihavebeen pwned
