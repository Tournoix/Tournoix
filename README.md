# Tournoix

Tournoix est une application web de gestion de tournoi. 

## Spécifications

### Requirements fonctionnels

#### Authentification utilisateur
* Il est possible de créer un compte avec un nom d'utilisateur, une adresse mail et un mot de passe.
* L'utilisateur peut se connecter, s'il ne l'est pas déjà et dans ce cas, il peut se déconnecter.

#### Gestion de tournoi
* Un utilisateur connecté peut créer un nouveau tournoi. Il va devoir remplir des paramètres demandés sur la page de création du tournoi. Ces paramètres sont :
  - le nom du tournoi
  - une description du tournoi
  - la date du tournoi
  - le lieu du tournoi
  - le type de tournoi
  - le nombre d'équipe dans un groupe
  - les équipes
* un propriétaire de tournoi peut modifier ces paramètres tant que la phase de qualification n'a pas commencée. Une fois le tournoi demarré, il n'est plus possible de modifier les équipes inscrites ainsi que le nombre d'équipes dans un groupe.
* un propriétaire peut aussi supprimer un tournoi
* une fois un tournoi créé, il est possible d'obtenir un code et une URL pour inviter des gens à visionner un tournoi

#### Visualiser un tournoi
* un spéctateur peut voir les groupes du tournoi. Il peut aussi voir les qualifications et la fourchette sans pouvoir modifier les resultats.

#### Gestion des groupes
* les groupes se remplissent la première fois de manière aléatoire.
* un propriétaire peut modifier les équipes se trouvant dans un groupe en les glissant dans un autre groupe
* une fois satisfait, il peut appuyer sur le bouton pour démarrer le tournoi ce qui bloquera la modification des groupes.

#### Gestion des qualifications
* dans la partie des qualification, il est possible d'indiquer le score d'un match entre deux équipes

#### Gestion de la phase éliminatoire
* l'emplacement des équipes sur l'arbre est fait de manière semi altéatoire, les premiers de groupe ne tombent pas les uns contre les autre lors du premier match.
* avant le début de la phase éliminatoire, il est possible de modifier l'emplacement des équipes dans l'arbre.
* une fois la phase démarée, il faut rentrer le résultat pour que l'équipe gagnante continue son chemin dans l'arbre.

#### Les paris sur un tournoi avec des noix
* il est possible de miser sur le vainqueur d'un match avant qu'il soit démarré
* la monnaie sont des noix. On commence chaque tournoi avec un certain nombre de noix dont le nombre augmente ou diminue en fonction de nos pronostics.

#### Affichage des resultats
* une fois le tournoi terminé, un classement est affiché
* il y a un deuxième classement avec les personnes ayant terminé le tournoi avec le plus de noix.


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

## Lancer l'application

**Pré-requis**: 
- ...
