- Quels sont les 4 thèmes abordés dans ce chapitre ?
		La possession, l'emprunt, les slices et la façon dont Rust agence les données en mémoire
- Quelles sont les deux approches traditionnelles de gestion de la mémoire ?
		gestion manuelle de la mémoire et garbage collector
- Est-il a priori rapide de se familiariser avec le concept de possession ?
		Non il n'est pas a priori rapide de se familiariser avec le concept de possession car il y a beaucoup de nouveaux aspects à prendre en compte.
- Quelle structure de donnée est utilisé dans le chapitre pour exemplifier la possession ?
		String
- Connaîssez-vous un autre langage qui demande la maitrise de la différence pile/tas ?
		Le langage C et C++
- (Hors Livre) En gros, quelles instructions assembleurs permettent de manipuler la pile, et comment la pile est-elle gérée ? et par quoi ? processeur/système d'exploitation/bibliothèque ?
		Les instructions assembleurs qui permettent de manipuler la pile sont push, pop, add, sub, call et ret. La gestion de la pile est faites grace aux instructions assembleurs, le système d'exploitation gère l'allocation et la sécurité et les bibliothéques fournissent  des abstractions pratiqiues pour les développeurs.
- (Hors Livre) En gros, quelles instructions assembleurs permettent de manipuler le tas, et comment le tas est-il géré ? et par quoi ? processeur/système d'exploitation/bibliothèque ?
		malloc, free. le tas nécessite une gestion explicite de la mémoire, où les développeurs doivent allouer et libérer manuellement la mémoire.
- (Hors Livre) Pourquoi il est mieux de consulter des données qui sont proches les unes des autres pour un processeur moderne ?
		Car cela améliore l'éficacité des processus en retournant des réponses beaucoup plus rapidement car les caches sont plus utilisés.
- Quel genre de données sont stockées sur la pile ?
		les données qui ont une taille fixe et une durée de vie limitée
- Quel est le but principale de la possession ?
		Gestion efficace et sécurisé de la mémoire
- Quand est-ce qu'une valeur est supprimée ?
		une valeur est supprimée lorsque la variable qui la possède sort de la portée ou si elle a été déplacée vers une autre variable.
- Vrai, faux ou autre ? "Une variable est "en vigueur" de sa déclaration jusqu'à la fin de l'exécution de programme."
		Faux, les variables peut sortir de la portée à tout moment en fonction de la structure du code.
- En terme de pile et de tas, quelle est la différence entre les types du chapitre 3 et le type String ? Pourquoi cette différence est-elle nécessaire ?
		la différence est la taille fixe des types du chapitre 3 or le type string a une taille non fixe. Cette différence est nécessaire car les types de taille fixe sont alloués sur la pile or les types non fixes tels que string sont alloués sur le tas cela permet une distinction et d'avoir une gestion plus efficace
- Donner une expression de type String.
  
		let s = String::from("hello");
- Les littéraux comme ", world" sont-ils de type String ?
		Non ce sont des littéraux de chaine
- Vrai, faux ou autre ? "Les chaînes de caractères littérales et celles saisies par l'utilisateur sont stockés dans la même zone mémoire."
		Faux car les saisies par l'utilisateur sont stocké sur le tas et les chaines de caractères littérales sont stocker sur la pile donc pas la meme zone memoire
- Quelle fonction est automatiquement appelée lorsque qu'un String sort de sa portée ?
		drop
- Une valeur de type String nécessite de la mémoire: uniquement dans le tas, uniquement sur la pile, ou sur les deux ?
		sur les deux
- Qu'est-ce qu'une "double libérations" (ou "double free")  et son rapport avec la sécurité informatique ?
		lorsque deux parties différentes d'un code essaient de libérer le même pointeur. Cela peut créer des crashs de programme, de la vunérabilité et des executions de code arbitraire
- Étant donnée la façon dont Rust gère la mémoire, pourquoi serait-il un problème que deux pointeurs possèdent la même mémoire dans la tas en même temps ?
		il pourrait y avoir double libération, violations de la propriété, et insécurité de la mémoire
- Vrai, faux ou autre ? "Une instruction telle que `let y = x;` peut parfois représentée une copie profonde couteuse."
		Faux, 
- Quelle est la différence entre un déplacement et une copie superficielle ?
		le déplacement est lorsqu'on assigne une variable à une autre ex(
			let x = y;)
		une copie superficielle est lorsqu'on utilise Copy et que la copie de la valeur est effectué mais sans déplacer la valeur d'origine

- Vrai, faux ou autre ? "Si on souhaite qu'un type fasse une copie il *suffit simplement* d'annoter ce type avec le trait Copy."
		Faux si le type requiert une gestion de la mémoire (alloue de la mémoire tas) il ne pourra pas implémenter copy ou si il a également le trait drop
- Vrai, faux ou autre ? "Tout ce qui est alloué durant l'exécution d'une fonction est désallouée à la fin de l'exécution de la fonction."
		Faux 
- Vrai, faux ou autre ? "Pour des raisons de sécurité et de cohérence, une fonction ne peut désallouer que ce qu'elle a alloué."
		Vrai
- Vrai, faux ou autre ? "Les fonctions n'introduisent aucunes subtilités sur la gestion de la possession: pour comprendre la possession il suffit comprendre ce qui se passe avec des variables."
		Faux, les variables de définitions ou les variables passé en parametre peuvent entrainer une gestion de la possession différente
- Dans le même style que l'encart 4-5, écrire une fonction `meme_longueur` qui reçoit deux String, et retourne un booléen indiquant s'ils ont la même taille, ainsi que les deux String pour rendre leur propriété. Écrire une fonction main similaire à celle de l'encart pour tester votre fonction.
  ```rust
		use std::{io};

		fn main() {
			let mut input_x = String::new();
			let mut input_y = String::new();

			println!("Veuillez entrer la valeur de x");
			println!("Veuillez entrer la valeur de y");

			io::stdin().read_line(&mut input_x).expect("error");
			io::stdin().read_line(&mut input_y).expect("error");

			let result = meme_longueur(input_x, input_y);
			println!("resultat: {}", result);
		}

		fn meme_longueur(x: String, y:String)-> bool{
			x.len() == y.len()
		}
		
	```
- Réécrire votre code précédent afin de profiter des références Rust.
  ```rust
		use std::{io};

		fn main() {
			let mut input_x = String::new();
			let mut input_y = String::new();

			println!("Veuillez entrer la valeur de x");
			println!("Veuillez entrer la valeur de y");

			io::stdin().read_line(&mut input_x).expect("error");
			io::stdin().read_line(&mut input_y).expect("error");

			let result = meme_longueur(&input_x, &input_y);
			println!("resultat: {}", result);
		}

		fn meme_longueur(x: &String, y: &String)-> bool{
			x.len() == y.len() 
		}
	```
- Vrai, faux ou autre ? "Lorsque d'une fonction à une référence vers un String, cette fonction *possède* ce String, et par les règles de la possession, cette String est désalloué à la fin de la fonction."
		Faux la fonction ne possède pas le String, elle n'a qu'une référence à celle ci
- Vrai, faux ou autre ? "Lorsque d'une fonction à une référence vers un String peut également modifier le String".
		Autres, cela dépend ci la référence est muttable ou non.
- Votre fonction `meme_longueur` précédente (version avec référence) peut-elle être appelé avec deux références identique, i.e. `meme_longueur(&s, &s)` ?
		Oui cela est possible psk la fonction utilise des références immuables
- Tentez d'écrire de mémoire les règles sur les emprunts.
		&s, &mut S, durée de vie des références, pas de mélange d'emprunts
- (question volontairement flou) Donner différents exemples d'emprunts invalides et essayer d'imaginer, pour chacun d'eux, un contexte dans lequel cela provoquerait un bogue.
  
		let mut s = String::from("hello");
   		let s1 = &s; // Référence immuable
		let s2 = &mut s; // Référence mutable (provoque une erreur)

		  let r;
    	{
        	let s = String::from("hello");
        	r = &s; // r fait référence à s, mais s est détruit à la fin de ce bloc
    	}
		println!("r: {}", r); 

- Quels soucis pourraient survenir si on choisi de faire une fonction `second_mot(&String) -> (usize,usize)` qui se contente de retourner la taille du premier et laisse le programmeur utiliser le String d'origine et cette taille lorsqu'il veut travailler avec le premier mot ?
		Incohérence d'indice, Risque d'erreur, gestion de durée de vie
- Y aurait-il une façon de résoudre ces soucis sans utiliser le concept de slice ? Quel compromis faut-il faire ?
		types énumérés: augment la compléxité, références avec lifetime: complique le code
- Si on a un variable `s` de type `String`, quel est le type de `&s` ? Vers quoi pointe le pointeur sous-jacent de `&s` ? quel est le type de `&s[..]` ?  Vers quoi pointe le pointeur sous-jacent de `&s[..]` ?<br>
		&s: type &String pointe vers l'objet String
<br>	&s[..]: type &str pointe vers le buffer de caractères de String
