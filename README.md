TFRT 01 - Présentation de Slint

Il existe bon nombre de solutions / librairies différentes pour créer des IHM avec Rust. (Voir ici => https://areweguiyet.com/)
Après avoir (très rapidement) essayé Egui, puis Iced, mon choix s'est arrêté sur Slint, voir les liens :
 => https://slint.rs/
 => https://slint.dev/ 
 => https://docs.slint.dev

Non pas que les 2 premières (Equi et iced) ne soient pas de qualité, elles permettent elles aussi de réaliser de très bonnes IHM, mais l'approche de Slint est celle qui m'a paru la plus claire, et Iced manque cruellement de documentation.
L'approche de Slint pourrait être assez comparable au QML du framework Qt, avec aussi pas mal de widgets directement utilisables.
Elle présente l'avantage d'une excellente séparation entre l'IHM et le code 'loqique'.

Voici un 1er exemple de code pour réaliser une IHM avec Slint, dérivé du template que Slint fourni pour démarrer 'en douceur'.

Ce template / projet 'de base' contient le minimum pour obtenir quelque chose de fonctionnel :
 - un fichier Cargo.toml qui contient les dépendances requises
 - un fichier build.rs qui dans sa fonction main() contient les instructions de compilation fichier slint
   pour qu'il soit intégré au binaire de l'application - Côté main.rs, on doit placer la ligne
	 'slint::include_modules!();' au début pour indiquer à cargo d'utiliser le fichier build.rs pour compiler
	 l'IHM (les fichiers de description de l'IHM étant dans un/des fichier(s) séparé(s) du code source).
 - un dossier ui/ qui contient le fichier de description de l'IHM, app-window.slint
 - un dossier src/ qui contient le fichier source principal de l'application, main.rs
 - un dossiser assets/img qui contient l'image qui sera utilisé comme icône de l'application (facultatif)

Dernière précision, je travaille avec VSCode, avec les plugins "rust-analyser" et "Slint" (Slint language support). Cela n'est pas un pré-requis, mais cette configuration fonctionne plutôt bien, reste simple et efficace. 
Le plugin "Slint" permet aussi d'obtenir une preview de l'IHM en cours, très utile quand on souhaite juste créer un nouveau widget avant même de penser à l'intégrer dans une application. 



Détails / description des fichiers 

Fichier app-window.slint :
 - la ligne 'import...' contient les imports nécessaires, selon les widgets utilisés
 - le mot-clé export de la ligne 'export component AppWindow inherits Window' permet au 
   reste de l'application d'accéder au composant AppWindow (de type 'Window')
 - on définit la taille min de la fenêtre avec 'min-width' et 'min-height'. Attention, on 
   doit spécifier l'unité à utiliser (généralement px, pixels logiques, mais il existe aussi
	 les pixels physiques, notés phx)
 - pour les positionnements et dimensionnements, voir => https://docs.slint.dev/latest/docs/slint/src/language/concepts/layouting)
 - on définit le titre et l'image de l'application avec les propriétés 'title' et 'icon'
 - la ligne 'in-out property <int> counter: 42;' définit la variable counter comme étant accessible
   en écriture (partie 'in', appel à set_counter() dans le fichier main.rs) 
	 et en lecture (partie 'out', appel à get_counter() dans le fichier main.rs) par l'utilisateur du composant
 - les lignes 'callback' déclarent l'existence de fonctions dont le corps devra être renseigné dans la partie
   utilisateur du composant (ici, dans le fichier main.rs).
 - ces fonctions 'callback' sont appelées à partir d'actions sur les widgets dans le fichier '.slint'.
   elles sont ici appelées sur l'évènement 'clicked' des boutons 'Decrease value' et 'Increase value'.
 - le widget de type 'Text' affiche la valeur de la variable 'counter' mis à jour par les appels
   aux fonctions request-increase-value() et request-decrease-value()
 - Slint créé automatiquement les getters et setters nécessaires pour les variables définies dans le fichier '.slint',
   ainsi que les fontions 'on_XXXXXXXXX' déclarées aux lignes 'callback'
 - les widgets sont définis ensuite dans une 'VerticalBox' afin d'être positionnés de haut en bas dans la fenêtre


Fichier buid.rs :
 - Ce fichier est utilisé pour la compilation du (ou des) fichiers .slint. La compilation sera effectuée
   selon les commandes présentes dans la fonction main() de ce fichier.

 - Il existe à la base deux approches pour compiler une IHM Slint :

   a/ La première méthode (la plus simple) utilise la commande suivante : 
      slint_build::compile("ui/app-window.slint").expect("Slint build failed");

      Cette méthode semble être la plus adaptée pour générer une IHM sans utiliser de thème particulier
      et pour utiliser des widgets 'custom', que nous verrons dans un autre mini-projet 


   b/ La seconde méthode permet de sélectionner un style à appliquer à tous les widgets.
      Les styles disponibles sont ici : https://docs.slint.dev/latest/docs/slint/src/advanced/style

    Les commandes à appliquer sont (pour le style 'fluent-dark') :
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/app-window.slint", config).unwrap();


Fichier main.rs :
  - ce fichier contient le code principal de l'application. 
	- il contient l'import std::error::Error nécessaire à la gestion d'erreur
	  possiblement renvoyée en sortie de la fonction main() (Box<dyn Error>),
	-	il contient aussi la macro 'slint::include_modules!();', nécessaire
	  à l'utilisation du fichier build.rs
	- il contient enfin une unique fonction main() qui va commencer par
	  créer notre fenêtre d'application : 
	      let ui = AppWindow::new()?;
	- si l'application est correctement créée (pas de sortie en erreur), on définit alors le contenu
		des méthodes de callback qui ont été déclarées dans les lignes 'callback' du fichier '.slint'
	    	ui.on_request_increase_value({...});
		    ui.on_request_decrease_value({...});
	- ensuite, on appelle la méthode run() de l'application fenêtrée : ui.run()?;
	- enfin on retourne Ok(()), car main() doit retourner quelque chose de type Result<>


Si vous avez créé un nouveau projet dans lequel vous avez placé les 4 fichiers ci-dessus,
il ne vous reste plus qu'à faire un 'cargo build' pour voir le résultat.
Personnellement, j'utilise plus souvent cargo run --release, afin d'obtenir
directement un exécutable plus léger.

Voilà, c'est tout pour ce premier Topic, en espérant qu'il aura pu vous être utile...

