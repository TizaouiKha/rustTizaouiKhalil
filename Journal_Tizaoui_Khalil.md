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

"cargo new": créer un projet

"cargo build": compile le projet

"cargo run": compile et lance l'éxécutable

"cargo check": compile le projet sans transformation en binaire pour vérifier les erreurs

"cargo build --release": créer un dossier release avec un éxecutable si on veut livrer une version stable

Question examen : Quels sont les différentes commandes cargo?

## Chapitre 2: Programmer le jeu du plus ou du moins

Les variables sont immuables par défaut en Rust, mettre "let mut x" pour la rendre muttables.

"io::stdin().read_line(&mut supposition).expect("Échec de la lecture de l'entrée utilisateur");" 

stdin() va nous permettre de nous renvoyer vers l'entrée du terminal 

.read_line()va nous permettre de récupérer la saisie de cette entrée du terminal

"&" l'argument est une référence

les références sont immuables par défaut

.read_line() retourne une valeur de type "io::Result"

Les types Result sont des enums(énumérations) qui sont souvent utilisés avec des match, Result sert à récupérer des informations pour la gestion d'erreurs (Ok ou Err)

"println!("Votre nombre : {}", supposition);" {} est un espace réservé pour la variable. S'il y'avait "{} {}, x, y" x irait la première et y dans la deuxième, système qui s'incrémente.

lorsqu'on rajoute par ex "rand = "0.8.3" dans les dépendances et qu'on build ça télécharge la crate rand et celles dont elle à besoin pour fonctionner

le fichier cargo lock permet a cargo de connaître les versions des dépendances du projet et de ne pas toujours allez chercher les versions jusqu'ç qu'on le change nous même

"cargo update": permet de mettre à jour les dépendences comme ceci 0.8.X
il ne montera pas en version 0.9.0 pour avoir la version 0.9.0 il faut modifier cargo.toml

"let nombre_secret = rand::thread_rng().gen_range(1..101)" rand::thread_rng() nous permet d'appeller le générateur de nb aléatoires qui est dans le crate rand et .gen_range nous permet de génerer un intervalle de nombre pour le générateur aléatoires.

Less (inférieur), Greater (supérieur) et Equal (égal)

"match" permet de comparer une valeur avec d'autres valeurs et d'ensuite faire une action, avec des branches qui sont parcourus à tour de rôle si la valeur est la même que le motif on rentre dans cette branche

En rust on peut masquer une valeur en la réaffectant (shadowing)

"u32" = chaines constitué que de chiffre

.trim() : enleve les whitespace et les saut de ligne \n

.parse() : parse la chaîne de caractères dans le format spécifier ex: " let supposition: u32 = supposition.trim().parse()" va parser en u32. Elle ne  peut parser que sur des nombres donc il faut gérer les erreurs car part facilement en erreur

"break" permet de quitter la boucle

"continue" permet de continuer le programme même si ce qu'on a rentrer n'est pas au bon format

Question: Qu'est ce qu'une variable immuable et une variable mutables ? Qu'est ce qu'une référence ? "

Question examen: Comment récupérer l'entrée d'un utilisateur ? Qu'est ce que io::stdin().read_line() ?

A Quoi servent les références ? A quoi sert cargo lock? A quoi sert match? A quoi Sert trim() ? Qu'est ce qu'u32 ? Qu'est ce que parse ? A quoi sert break ?


## Chapitre 3: Les concepts courants de programmation

### 3.1 Les variables et la mutabilité 

variable immuable : valeur liée au nom et qui ne peut pas être changé

valeur muttable : valeur que l'on peut changer lorsqu'on le souhaite suffit de rajouter "mut" avant le nom de la variable lors de sa création

constante : déclaré par "const" ce sont des variables immuables qui sont toujours immuables, qui peuvent être définis par une expression constante et non par le résultat d'une valeur calculé qu'a l'exécution



