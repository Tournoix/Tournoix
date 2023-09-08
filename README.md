# Tournoix

Tournoix est une application web de gestion de tournoi. Elle permet également aux utilisateur de miser des noix, une monnaie virtuelle du site. A la fin du tournoi, on regarde qui a gagné ainsi que qui a su faire les paris les plus judicieux et avoir le plus de noix.

## Spécifications

### Requirements fonctionnels

#### Authentification utilisateur

- Il est possible de créer un compte avec un nom d'utilisateur, une adresse mail et un mot de passe.
- L'utilisateur peut se connecter, s'il ne l'est pas déjà et dans ce cas, il peut se déconnecter.

#### Gestion de tournoi

- Un utilisateur connecté peut créer un nouveau tournoi. Il va devoir remplir des paramètres demandés sur la page de création du tournoi. Ces paramètres sont :
  - le nom du tournoi
  - une description du tournoi
  - la date du tournoi
  - le lieu du tournoi
  - le type de tournoi
  - le nombre d'équipe dans un groupe
  - les équipes
- un propriétaire de tournoi peut modifier ces paramètres tant que la phase de qualification n'a pas commencée. Une fois le tournoi demarré, il n'est plus possible de modifier les équipes inscrites ainsi que le nombre d'équipes dans un groupe.
- un propriétaire peut aussi supprimer un tournoi
- une fois un tournoi créé, il est possible d'obtenir un code et une URL pour inviter des gens à visionner un tournoi

#### Visualiser un tournoi

- un spéctateur peut voir les groupes du tournoi. Il peut aussi voir les qualifications et la fourchette sans pouvoir modifier les resultats.

#### Gestion des groupes

- les groupes se remplissent la première fois de manière aléatoire.
- un propriétaire peut modifier les équipes se trouvant dans un groupe en les glissant dans un autre groupe
- une fois satisfait, il peut appuyer sur le bouton pour démarrer le tournoi ce qui bloquera la modification des groupes.

#### Gestion des qualifications

- dans la partie des qualification, il est possible d'indiquer le score d'un match entre deux équipes

#### Gestion de la phase éliminatoire

- l'emplacement des équipes sur l'arbre est fait de manière semi altéatoire, les premiers de groupe ne tombent pas les uns contre les autre lors du premier match.
- avant le début de la phase éliminatoire, il est possible de modifier l'emplacement des équipes dans l'arbre.
- une fois la phase démarée, il faut rentrer le résultat pour que l'équipe gagnante continue son chemin dans l'arbre.

#### Les paris sur un tournoi avec des noix

- il est possible de miser sur le vainqueur d'un match avant qu'il soit démarré
- la monnaie sont des noix. On commence chaque tournoi avec un certain nombre de noix dont le nombre augmente ou diminue en fonction de nos pronostics.

#### Affichage des resultats

- une fois le tournoi terminé, un classement est affiché
- il y a un deuxième classement avec les personnes ayant terminé le tournoi avec le plus de noix.

### Requirements non-fonctionnels

#### Performances

Le site doit répondre dans un temps acceptable aux différentes actions utilisateur.

#### Disponibilité

Le site doit avoir une disponibilité de 99.9%. Les maintenances ne doivent pas engranger des downtime de plus de 2 heures.

#### Sécurité

Toutes les données sensibles sont stockées de manière conforme aux lois et au respect des utilisateurs.

#### Compatibilité

Le site doit être compatible sur toutes les plateformes. Cela comprenant différent format d'écran PC et mobile.

### Structure de l'application

## Méthodologie de travail agile

Pour ce projet nous allons travailler avec la méthode SCRUM.

Afin de gérer les différentes stories et tâches, nous allons utiliser un tableau Kanban qui nous permettra de représenter le backlog, le sprint actuel, les tâches en cours et les tâches terminées.

Ce tableau Kanban contient 4 colonnes **Backlog**, **Ready**, **In Progress** et **Done**.

- La colonne **Backlog** contient le backlog du projet, c’est à dire toutes les tâches en attente d’être réalisées.
- La colonne **Ready** contient les tâches du sprint actuel. Ces tâches sont déplacées dans cette colonne en début de sprint durant le premier meeting scrum du sprint. Les tâches non terminées restent dans cette colonne pour le prochain sprint.
- La colonne **In Progress** contient les tâches en cours de réalisation. Les développeurs déplacent eux-mêmes les tâches qu’ils souhaitent réaliser ou qui leur ont été assignées dans la colonne **Ready**.
- Une tâche est mise dans la colonne **Done** quand une tâche est terminée, qu'elle a été push dans le repo github et que tous les tests sont passés.

### Scrum

Un meeting scrum aura lieu chaque matin et chaque début d'après midi afin de mettre à jour toute l’équipe sur les tâches en cours, terminées et les prochaines priorités ainsi que les problèmes rencontrés. Cela nous permet de se recoordonner, regarder son avancement et de se rendre compte d’un possible ralentissement de l’avancée des taches.

### Scrum Master

Le scrum master aura la responsabilité que l’équipe applique correctement la méthode scrum.

## Méthodologie GIT

Dnas notre repertoire git, nous utilisons les branches de trois façon distinctes :

- la branche **main**, elle contient le code en production. Cette branche a un CI/CD permettant de lancer les tests unitaires définis dans le code et de déployer l'application en production.
- la branche **develop**, elle contient toutes les features que l'on a implémenté mais il n'y a pas encore assez de features pour faire une release. Cette branche a un CI qui lance les tests unitaires définis dans le code.
- les branches **feature/...**, elles sont utilisées pour coder les différentes features. L'objectif est d'avoir environ une branche par issue.

## Structure de l'application

![tournoix_structure drawio](https://github.com/Tournoix/Tournoix/assets/30533851/9f160057-f37e-42c5-ba31-2a0790b3d757)

Notre application est composée de deux applications distinctes:

- Un backend, qui est une application web fournissant l'application frontend et une API REST
- Un frontend, qui est une application WebAssembly tournant dans le navigateur

## Mise en place

### Installation:

1. Installer rust (>= v.1.71) et cargo (>= v.1.71) https://rust-lang.org
2. Ajouter la target wasm32

```bash
rustup target add wasm32-unknown-unknown
```

3. Installer Trunk

```bash
cargo install --locked trunk
```

(Optionel) Installer cargo-watch pour le hot reload du backend

``bash
cargo install --locked cargo-watch
```

4. Installer du moteur de base de donnée mariaDB via docker (recommandé)

4.1. Installation de docker

Pour installer via Windows:

Pour installer via Linux, lancer ces commandes:

```bash
curl -sSL https://get.docker.com/ | sh
sudo systemctl start docker
sudo gpasswd -a "${USER}" docker
```

Nécessite un logout/login pour que l'utilisateur courrant ajouté au groupe docker ait les droits de lancer des commandes docker.

4.2. Création du container de la base de donnée

```bash
docker create --name tournoix_db -p 3306:3306 \
--env MYSQL_DATABASE=tournoix_db \
--env MYSQL_USER=tournoix_user \
--env MYSQL_PASSWORD=sample_password \
--env MYSQL_ROOT_PASSWORD=super_secret \
mariadb
```

4.3. Création des fichiers de config
Copier le fichier `.env.example` et le renommer `.env`
Copier le fichier `Rocket.toml.example` et le renommer `Rocket.toml`
Modifier ces fichiers selon la config (par exemple le mot de passe ou le nom de la db)

5. Diesel CLI
5.1. Installation
```bash
cargo install diesel_cli --no-default-features --features postgres
```

Note: l'installation de Diesel nécessite les librairies dev de mysql (libmysql-dev), l'installation de ces librairies dépends du système d'exploitation.

5.2. Lancer les migrations pour créer la base de donnée

```bash
diesel migration run
```

### Démarrage:

- Démarrer la base de données

```bash
docker start tournoix_db
```

- Démarrer le backend (API)

```bash
cd ./backend
cargo run
```

Pour le hot reaload, on doit utiliser cette commande à la place:
```bash
cd ./backend
cargo watch -x run
```

- Démarrer le frontend en mode développement

```bash
cd ./ui
npm run serve
```

Pour le hot reaload, on doit avoir cette commande qui tourne à côté:
```bash
cd ./ui
npx npx tailwindcss --watch -i ./style.scss -o ./output.scss
```

### Déploiement:

- Build en mode release

```bash
npm run build
cd ./backend
cargo run --release
```

### Autre:

- Reset la base de données
```bash
diesel database reset
```
