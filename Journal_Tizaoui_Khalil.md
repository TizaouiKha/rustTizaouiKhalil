# Journal de TIZAOUI Khalil

<details>
<summary> Questions Pages d'Acceuil </summary>
<br>
Quels sont les raisons pour les quels Rust est rapide et n'utilise pas beaucoup de mémoire?
<br><br>
Qu'est ce que le modèle d'ownership?
<br><br>
Si Rust est autant productif, pourquoi n'est il pas plus utilisé / populaire ? 
<br><br>
On en vient donc à se poser la question: Quels sont les avantages et les désavantages de Rust? Dans quels domaines ce language est-il "fort" et dans quels domaines est-il "moins-fort"?
    
</details>

### Les premières pages du livre
<details>
<summary>Choses Utiles à retenir</summary>
rustup docs --book : cette commande ouvre le livre rust format anglais en local sur la machine
</details>
<br>
<details>
<summary>Avant-propos</summary>
Rust permet de facilité la programmation bas-niveau en déjouant les failles de sécurité, les plantages de système, qui traditionnelement demande beaucoup d'expertise et de prudence, et en guidant naturellement vers un code fiable et efficace. 
<br>
Rust est un language qui facilite la programmation bas niveau mais qui est également utilisable pour les serveurs-web, applications en ligne de commande, etc...<br>
Question examen: Qu'est ce qui est facilité avec Rust?
</details>
<br>
<details>
<summary>Introduction</summary>
Rust permet de contrôler l'utilisation de la mémoire.
<br><br>
Pour les équipes de dev:
Outils de développement Rust: <br>
- Cargo: gestion de dépendance et compilation<br>
- Rustfmt: style de codage<br>
- Rust Language Server: complétion de code et messages d'erreur dans l'IDE<br>
<br>
Pour les étudiants: Rendre accessible la compréhension des notions de système avec le livre

Pour les entreprises : Rust est utilisé pour différentes missions ( IOT, machine learning, services web, FIREFOX, etc...) 

Ouvert aux dev de logiciel libre.

Rust fait en sorte que le code soit sur  et rapide sans se soucier du controle de code instable fait auparavant qui pouvait etre fait auparavant.

Question: Rust se vend comme un language simple qui facilite la vie du développeur et qui est accessible à tous les profils. Est-ce vrai ?

Question examen: Quels sont les outils de dev Rust? A qui Rust est-il destiné?
</details>


## Chapitre 1: Prise en main

### 1.1 Installation

Après avoir fait la commande "rustup update" ma version de rustc est 1.81

Question examen: Comment mettre à jour Rust ?

### 1.2 Hello, wolrd!

"println!()" est un appel a une macro rust car il y a un point d'interogation

"rustc" permet de compiler le fichier rust et créer un éexecutable et un fichier de débogagge avec l'extension .pdb

Rust est un language a compilation anticipée ce qui permet de compiler le programme et de le partager avec n'importe qui sans avoir Rust d'installé

Question examen: Comment appeller une macro? Comment afficher une chaine de caractères? Qu'est ce que la compilation anticipée?


### 1.3 Hello, Cargo!



