- Voici une question pour vous montrer le format d'une réponse:
		Voici une réponse correctement indentée pour être replier avec la question.
		Merci d'ajouter vos réponses aux questions ci-dessous en respectant ce format.
		Répondez avec concision, mais avec le niveau de détail que vous donneriez à l'oral.
- Selon la page d'introduction du chapitre, combien de concepts y sont abordés ?
		Selon la page d'instruction cinq concepts y sont abordés
- Qu'est-ce qu'un _mot clé_ ?
		Un mot clés est un mot que le langage de programmation reconnaît et interprète d'une manière spéciale car ils ont un rôle précis tels que fn, let, mut, etc...
- Tous les mots clés sont-ils associés à des fonctionnalités ?
		Tous les mots clés ne sont pas associés à des fonctionnalités car il y a des fonctionnalités pas encore développer mais qui sont des mots clés pour prévenir les conflits dans les projets des développeurs dans les mis à jour futures.
- Où peut-on trouver la liste de tous les mots clés ?
		On peut trouver la liste des tous les mots clés dans l'annexe A du rust-book.
- Vrai, faux, ou autres ? "Une variable correspond à toujours une "mémoire" pouvant prendre différentes valeurs durant une seule et même exécution."
		Cette déclaration est en partie fausse car les variables en rust sont par défaut immutables mais elles peuvent être muttables si on utilise "mut" elle devient donc vrai cette déclaration. Elle est donc fausse la majeur partie du temps mais peut devenir vrai lorsqu'une variable est muttables.
- Comment déclarer une variable correspond à une "mémoire" pouvant prendre différentes valeurs durant une seule et même exécution ?
		En utilisant "mut" 

			let mut x = 1;
			x=10;

- Vrai, faux, ou autres ? "Si un variable ne prends qu'une seule valeur au cours de l'exécution d'un programme, alors il est toujours possible de la déclarer comme une constante."
		Vrai et cela est recommandé
- Vrai, faux, ou autres ? "Comme en Java, deux variables Rust différentes ont forcément des noms différents au sein d'un seul et même bloc de portée."
		Vrai
- Vrai, faux, ou autres ? "Au sein d'un bloc de portée, un _nom_ de variable correspond à un unique et même type tout au long du bloc de portée."
		Vrai
- Donner la liste des principaux types scalaires de Rust.
		Entier, nombres à virgules flottantes, les booléens et  les caractères.
- Combien y a-t-il de types de nombres entiers en Rust ?
		Il y a 12 types de nombres entiers en Rust, 6 en types signés (i8,etc...) et 6 en types non signés (u8, etc...)
- Combien de types de nombres entiers ont un nombre de bit indépendant de l'architecture vers laquelle le programme est compilé ?
		Il y a en a 10 (i8, ... , i128 et u8, ... , u128)
- Quelle est le type de nombres entiers par défaut ?
		Le type de nombres entiers par défaut est i32
- Quelles types de nombres entiers utilise-t-on pour indexer un tableau en Rust ?
		isize et usize
- Quelle est la représentation binaire machine de la valeur -3 de type i16 ?
		1111 1111 1111 1101
- Donner le littéral correspond à -3 de type i16.
		-3i16
- Comment écrire le littéral correspond à "cent millions" en Rust de sorte à rendre facilement lisible que le nombre de zéro est le bon ?
		100_000_000
- Vrai, faux, ou autres ? "En Rust, si on incrémente une variable de type u8 dans une boucle infini, alors toutes les valeurs de 0 à 255 s'affiche en boucle et le programme ne s'arrête jamais."
		Vrai, il va afficher des valeurs entre 0 et 255 à l'infini
- Donner la liste des types à virgule flottante.
		f32 et f64
- Vrai, faux, ou autres ? "Rust est un langage bas niveau, donc une variable à virgule flottante a une représentation différente en fonction de l'architecture vers laquelle on compile notre programme."
		Faux car représenté par la norme IEEE 754
- Vrai, faux, ou autres ? "Le type flottant par défaut a une taille de 32 bits."
		Faux car par défaut la taille est de 64 bits
- Où peut-on consulter la liste de tous les opérateurs numériques ?
		Annexe B du rust-book
- Vrai, faux, ou autres ? "Rust profite d'une panoplie de près de 200 opérateurs numériques."
		Faux
- Vrai, faux, ou autres ? "En Rust, un test qui échoue (comme 10 > 30) renvoie 0 alors qu'un test qui réussi comme (10 < 30) renvoie 1."
		Faux renvoie des booléen
- Vrai, faux, ou autres ? "Le type char représente 8 octets et représente un caractère ASCII."
		Faux représente caractère Unicode
- Vrai, faux, ou autres ? "Il est possible d'ajouter des éléments dans un tableau Rust, modifiant ainsi sa taille, au cours de l'exécution d'un programme."
		Faux sa taille est fixe
- Si on a écrit un programme avec une variable x de type `(i32, i32, i32)`, décrire informellement comment le réécrire pour que son type soit maintenant `[i32; 3]` ? Le programme résultat est-il globalement plus court ? plus long ? de même longueur ?
		let x: [i32; 3] = [5, 8, 10]; le programme est légèrement plus court
- Si on a écrit un programme avec une variable x de type `[i32; 3]`, décrire informellement comment le réécrire pour que son type soit maintenant  `(i32, i32, i32)` ? Le programme résultat est-il globalement plus court ? plus long ? de même longueur ?
	let x: (i32, i32, i32) = (x[0], x[1], x[2]); le programme est plus long
- Que se passe-t-il en Rust si on tente d'accéder à la case 10 d'un tableau de taille 5 ? En quoi cela diffère de ce qui se passe en C ?
  En rust si on essaye d'accéder hors limite il sera plus stricte et renvoiera un message d'erreur. Or en C il n'y aura pas de message d'erreur mais probablement un comportement bizzare tels que lire des valeurs aléatoires en mémoire
- Existe-t-il d'autres types Rust fonctionnant comme les tableaux ?
		Les HashMap, BTreeMap Vec(Vecteur), Slices ou LinkedList
- Vrai, faux, ou autres ? "Comme en Java, Rust a besoin d'allouer de la mémoire et d'utiliser des pointeurs en interne pour faire fonctionner ces types composés."
		Vrai mais d'une manière différente car il utilise le ownership et l'allocation de mémoire se fait sans 'garbage collector'
- Proposer une expression à mettre à la place des points d'intérrogation afin que le programme suivant compile et affiche "Yes" à l'exécution.
  ```rust
	fn main() {
	    let x = (2, 1, [(true, 1), (true, 2), (true, 3)]);
		if x.2[x.1].0 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
	```
- Transformer le code précédent en une fonction recevant en paramètre la valeur de x.
  ```rust
	fn main() {
		let x = (2, 1, [(true, 1), (true, 2), (true, 3)]);
		verif_x(x);
	}

	fn verif_x(x: (i32, usize, [(bool, i32); 3])){
		if x.2[x.1].0 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
	```
- Modifier la fonction pour qu'elle prenne un paramètre la valeur de x et retourne un booléen au lieu d'afficher un message. Faites le bien "à la Rust", en suivant le style indiqué dans le chapitre.
    ```rust
	fn main() {
		let x = (2, 1, [(true, 1), (true, 2), (true, 3)]);
		let result = verif_x(x);
		println!("{}", result);
	}

	fn verif_x(x: (i32, usize, [(bool, i32); 3])) -> bool{
		x.2[x.1].0
	}
	```
- Quelle convention utilise-t-on en Rust pour nommer les variables et les fonctions ?
		snake_case
- Quel est normalement la différence technique entre "paramètre" et "argument" ?
		Un paramètre sont des variables appartenant à la définition de la fonction et un argument est une variable que l'on fournie lors de l'appel de la fonction
- Le bout de code `let x = 10` est une expression ou une instruction ?
		c'est une instruction
- Le bout de code `let x = 10` contient il une expression ?
		Oui car la valeur 10 est assigné à x
- Vrai, faux, ou autres ? "Lorsqu'on déclare une variable, il est optionnel d'indiquer le type."
		Vrai
- Vrai, faux, ou autres ? "Lorsqu'on déclare un paramètre, il est optionnel d'indiquer le type."
		Faux cela est obligatoire
- Vrai, faux, ou autres ? "En Rust, il n'existe qu'une seule façon de faire des commentaires."
		Vrai
- Dans le code suivant, que faut-il mettre à la place des parenthèses pour que le code de la fonction compile (on ignorera les warnings):
	```rust
	fn blabla(x: i32) -> i32 {
		x + x;
	}
	```
- Les `if/else` sont ils des instructions ou des expressions ?
		expressions
- Les `if` sans `else` sont ils des instructions ou des expressions ?
		expressions
- Remanier le code suivant afin de n'avoir qu'un seul appel à `println!` et aucune variable supplémentaire.
    ```rust
	fn main() {
	    let x = (2, 1, [(true, 1), (true, 2), (true, 3)]);
		println!("{}", if x.2[x.1].0 { "Yes" } else { "No" });
	}
	```
- A quoi servent les étiquettes de boucle ?
		Gérer les boucles avec précision et sécurité et rendre le code plus lisible
- Une boucle `loop` est une expression. Quels types peuvent être retournés par un loop ?
		les types scalaires, les types composés et des objets
- Une boucle `while` est une expression. Quels types peuvent être retournés par un while ?
		Elle ne retourne pas de valeur car c'est une instruction
- Les boucles `for` de Rust ressemble au boucle `for` de quels languages que vous connaissez ?
		Java, JavaScript, Python et C#
- Comme suggéré à la fin du chapitre, réaliser les programmes suivants:
	- Convertir des températures entre les degrés Fahrenheit et Celsius.
		```rust
		se std::io;

		fn main() {
			let mut x = String::new();

			println!("Entrez une températur ex( 20°C ou 65F) :");
			io::stdin().read_line(&mut x).expect("Erreur");

			let x  = x.trim();
			let degree = x.chars().last().unwrap();
			let v : f64 = x[..x.len()-1].parse().expect("NO DEGREE");


			match degree {
				'C' => {
					let result = celsius_to_fahrenheit(v);
					println!("{}°C donne {} F", v, result);
				},
				'F' => {
					let result = fahrenheit_to_celsius(v);
					println!("{}°F donne {} C", v, result);
				}
				_ =>{
					println!("Veuillez entrez l'unité a côté du chiffre tel que : 32C ou 65F");
					main();
				}
			}
		}

		fn fahrenheit_to_celsius(x: f64) -> f64{
			(x - 32.0) * 5.0 /9.0
		}
		
		fn celsius_to_fahrenheit(x: f64) -> f64{
			(x*9.0 / 5.0) + 32.0
		}

		```
	- Générer le _n_-ième nombre de Fibonacci.
		```rust
		use std::io;

		fn main() {
			let mut x = String::new();
			
			println!("Entrez un nombre pour obtenir le n-ième nombre de Fibonacci :");
			io::stdin().read_line(&mut x).expect("Error");

			let n: u32 = x.trim().parse().expect("Error");

			let result = fibonacci(n);
			println!("Le {}-ième nombre de Fibonacci est : {}", n, result);
		}

		fn fibonacci(x: u32) -> u32 {
			if x == 0 {
				return 0;
			} else if x == 1 {
				return 1;
			}

			let mut a = 0; 
			let mut b = 1; 
			
			for _ in 2..= x {
				let result = a + b;
				a = b;
				b = result;
			}
			b 
		}

		```
	- Afficher les paroles de la chanson de Noël _The Twelve Days of Christmas_ en profitant de l'aspect répétitif de la chanson (https://www.lyricsforchristmas.com/christmas-carols/the-twelve-days-of-christmas/).
		```rust
			fn main() {
				let gifts = [
					"a partridge in a pear tree",
					"Two turtle doves",
					"Three French Hens",
					"Four calling birds",
					"Five golden rings",
					"Six geese a-laying",
					"Seven swans a-swimming",
					"Eight maids a-milking",
					"Nine ladies dancing",
					"Ten lords a-leaping",
					"Eleven pipers piping",
					"Twelve drummers drumming",
				];

				let days = [
					"first", "second", "third", "fourth", "fifth", "sixth", 
					"seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
				];

				for day in 0..12 {
					println!("On the {} day of Christmas, my true love sent to me", days[day]);
					
					for gift in (0..=day).rev() {
						if gift == day {
							println!("{}", gifts[gift]);
						} else if gift == 0 && day != 0 {
							println!("and {}", gifts[gift]);
						} else {
							println!("{},", gifts[gift]);
						}
					}
					println!(); 
				}
		}
		```