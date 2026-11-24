//! The port's own menus in French and Spanish (after WitherBlitz's
//! NSMB-Wii-Recompilation's `ui_text`): every line is looked up by its
//! English wording, so a line not in the table stays English. The game's
//! own text comes from the disc in the language picked at boot; these
//! menus follow the Settings window's choice at once.

use std::collections::HashMap;
use std::sync::OnceLock;

/// (English, French, Spanish).
const TABLE: &[(&str, &str, &str)] = &[
    // The window, its tabs and prompts.
    ("Settings", "Paramètres", "Configuración"),
    ("Display", "Affichage", "Pantalla"),
    ("Graphics", "Graphismes", "Gráficos"),
    ("Audio", "Audio", "Audio"),
    ("Keybinds", "Touches", "Controles"),
    ("Gameplay", "Jeu", "Juego"),
    ("Language", "Langue", "Idioma"),
    ("Select", "Choisir", "Elegir"),
    ("Back", "Retour", "Atrás"),
    ("On", "Oui", "Sí"),
    ("Off", "Non", "No"),
    ("Yes", "Oui", "Sí"),
    ("No", "Non", "No"),
    ("OK", "OK", "Aceptar"),
    ("Cancel", "Annuler", "Cancelar"),
    ("How the game is shown on your screen.", "Comment le jeu s'affiche sur votre écran.", "Cómo se ve el juego en tu pantalla."),
    ("How the game is drawn, and how smoothly it moves.", "Comment le jeu est dessiné, et sa fluidité.", "Cómo se dibuja el juego y con qué fluidez se mueve."),
    ("How loud the game's music and sounds are.", "Le volume de la musique et des sons du jeu.", "El volumen de la música y los sonidos del juego."),
    ("Which keys and controller buttons play the Wii Remote's buttons.", "Les touches et boutons de manette qui jouent les boutons de la Wiimote.", "Las teclas y botones del mando que hacen de botones del mando de Wii."),
    ("Optional helpers. Online, only Exit Course applies, if every player has it on.", "Aides facultatives. En ligne, seul Quitter le niveau s'applique, si tous les joueurs l'ont activé.", "Ayudas opcionales. En línea solo se aplica Salir del nivel, si todos los jugadores lo tienen activado."),
    ("Mods", "Mods", "Mods"),
    ("HD textures, and mods that change the game's files.", "Textures HD, et mods qui changent les fichiers du jeu.", "Texturas HD y mods que cambian los archivos del juego."),
    ("The language of the game's text and these menus.", "La langue des textes du jeu et de ces menus.", "El idioma de los textos del juego y de estos menús."),
    // Display.
    ("Resolution", "Résolution", "Resolución"),
    ("Aspect Ratio", "Format d'image", "Formato de imagen"),
    ("Fullscreen", "Plein écran", "Pantalla completa"),
    ("V-Sync", "V-Sync", "V-Sync"),
    ("Performance Monitor", "Moniteur de performances", "Monitor de rendimiento"),
    ("Native", "Native", "Nativa"),
    ("Original 4:3", "4:3 d'origine", "4:3 original"),
    ("The window's own resolution (from the next start).", "La résolution de la fenêtre (au prochain démarrage).", "La resolución de la ventana (al próximo inicio)."),
    ("Rendered at this many lines, then scaled to the screen (from the next start).", "Rendu à ce nombre de lignes, puis mis à l'échelle (au prochain démarrage).", "Se dibuja con estas líneas y se escala a la pantalla (al próximo inicio)."),
    ("Switching between 4:3 and widescreen takes effect after a restart.", "Le passage entre 4:3 et 16:9 prend effet au redémarrage.", "El cambio entre 4:3 y panorámico se aplica al reiniciar."),
    ("Borderless: a window covering the whole screen (F11).", "Sans bordure : une fenêtre qui couvre tout l'écran (F11).", "Sin bordes: una ventana que cubre toda la pantalla (F11)."),
    ("Off: the game plays in a window.", "Non : le jeu s'affiche dans une fenêtre.", "No: el juego se muestra en una ventana."),
    ("No tearing: frames wait for the display, the game keeps its pace.", "Sans déchirement : les images attendent l'écran, le jeu garde son rythme.", "Sin cortes: las imágenes esperan a la pantalla y el juego mantiene su ritmo."),
    ("Shows the FPS and each frame's time at the top left; left and right set its size.", "Affiche les FPS et le temps de chaque image en haut à gauche ; gauche et droite règlent sa taille.", "Muestra los FPS y el tiempo de cada imagen arriba a la izquierda; izquierda y derecha cambian su tamaño."),
    ("Ultrawide", "Ultra-large", "Ultrapanorámica"),
    ("The Wii's original 4:3 picture.", "L'image 4:3 d'origine de la Wii.", "La imagen 4:3 original de la Wii."),
    ("The game's 16:9 widescreen picture.", "L'image 16:9 du jeu en écran large.", "La imagen panorámica 16:9 del juego."),
    ("For a 16:10 screen: fills it, with a slightly narrower view.", "Pour un écran 16:10 : le remplit, avec une vue un peu plus étroite.", "Para una pantalla 16:10: la llena, con una vista algo más estrecha."),
    ("Fills a screen wider than 16:9 with a wider view (16:9 on others).", "Remplit un écran plus large que 16:9 avec une vue plus large (16:9 sur les autres).", "Llena una pantalla más ancha que 16:9 con una vista más amplia (16:9 en las demás)."),
    // Graphics.
    ("Spinning Coins", "Pièces animées", "Monedas giratorias"),
    ("Depth Blur", "Flou de profondeur", "Desenfoque de profundidad"),
    ("Animates coins and blocks; off saves a little GPU work.", "Anime les pièces et les blocs ; désactivé, épargne un peu le GPU.", "Anima las monedas y los bloques; desactivado ahorra algo de trabajo a la GPU."),
    ("The blur and heat haze of desert, beach, water and lava areas.", "Le flou et la brume de chaleur des zones de désert, de plage, d'eau et de lave.", "El desenfoque y la calima de las zonas de desierto, playa, agua y lava."),
    ("Renderer", "Rendu", "Renderizador"),
    ("Battery Saver", "Économie d'énergie", "Ahorro de batería"),
    ("Frame Interpolation", "Interpolation", "Interpolación"),
    ("Unlocked", "Illimité", "Sin límite"),
    ("Automatic", "Automatique", "Automático"),
    ("Vulkan, else Direct3D 12, else Direct3D 11 (from the next start).", "Vulkan, sinon Direct3D 12, sinon Direct3D 11 (au prochain démarrage).", "Vulkan; si no, Direct3D 12; si no, Direct3D 11 (al próximo inicio)."),
    ("The graphics API the game draws with (from the next start).", "L'API graphique avec laquelle le jeu dessine (au prochain démarrage).", "La API gráfica con la que dibuja el juego (al próximo inicio)."),
    ("Shows 30 frames a second while the game keeps running at 60.", "Affiche 30 images par seconde ; le jeu tourne toujours à 60.", "Muestra 30 imágenes por segundo; el juego sigue a 60."),
    ("Draws frames in between for smoother motion on screens above 60Hz.", "Ajoute des images intermédiaires pour plus de fluidité au-delà de 60 Hz.", "Dibuja imágenes intermedias para más fluidez en pantallas de más de 60 Hz."),
    ("Draws as many frames in between as the PC can, the game still at its own speed.", "Ajoute autant d'images intermédiaires que le PC le permet ; le jeu garde sa vitesse.", "Dibuja tantas imágenes intermedias como pueda el PC; el juego mantiene su velocidad."),
    // Audio.
    ("Master", "Général", "General"),
    ("Music", "Musique", "Música"),
    ("Sound Effects", "Effets sonores", "Efectos de sonido"),
    ("Menu Sounds", "Sons des menus", "Sonidos de menú"),
    ("The volume of everything.", "Le volume de tout.", "El volumen de todo."),
    ("The background music.", "La musique de fond.", "La música de fondo."),
    ("Jumps, coins, enemies and the rest.", "Sauts, pièces, ennemis et le reste.", "Saltos, monedas, enemigos y todo lo demás."),
    ("The menus' cursor, select and back sounds.", "Les sons de curseur, de choix et de retour des menus.", "Los sonidos de cursor, elección y retroceso de los menús."),
    // Keybinds.
    ("Up", "Haut", "Arriba"),
    ("Down", "Bas", "Abajo"),
    ("Left", "Gauche", "Izquierda"),
    ("Right", "Droite", "Derecha"),
    ("Jump (2)", "Sauter (2)", "Saltar (2)"),
    ("Run / Fire (1)", "Courir / Tirer (1)", "Correr / Disparar (1)"),
    ("Shake (Spin)", "Secouer (Vrille)", "Agitar (Giro)"),
    ("Tilt Left", "Incliner G", "Inclinar izq."),
    ("Tilt Right", "Incliner D", "Inclinar der."),
    ("+ (Pause)", "+ (Pause)", "+ (Pausa)"),
    ("- (Minus)", "- (Moins)", "- (Menos)"),
    ("Press a key...", "Appuyez sur une touche...", "Pulsa una tecla..."),
    ("None", "Aucune", "Ninguna"),
    ("Press the new key. Esc cancels; Delete or Backspace leaves it no key.", "Appuyez sur la nouvelle touche. Échap annule ; Suppr ou Retour arrière n'en laisse aucune.", "Pulsa la nueva tecla. Esc cancela; Supr o Retroceso la deja sin tecla."),
    ("Controllers: A jumps, B and X run, Y shakes, Start is +, Back is -.", "Manettes : A saute, B et X courent, Y secoue, Start est +, Back est -.", "Mandos: A salta, B y X corren, Y agita, Start es +, Back es -."),
    ("The D-pad's up: climb vines, and go through doors and pipes.", "Le haut de la croix : grimper aux lianes, passer portes et tuyaux.", "Arriba en la cruceta: trepar enredaderas y entrar por puertas y tuberías."),
    ("The D-pad's down: crouch, and ground-pound in the air.", "Le bas de la croix : s'accroupir, et attaque rodéo en l'air.", "Abajo en la cruceta: agacharse, y golpe de trasero en el aire."),
    ("The D-pad's left: walk left, and move a menu's cursor.", "La gauche de la croix : marcher à gauche, et déplacer le curseur des menus.", "Izquierda en la cruceta: caminar a la izquierda y mover el cursor de los menús."),
    ("The D-pad's right: walk right, and move a menu's cursor.", "La droite de la croix : marcher à droite, et déplacer le curseur des menus.", "Derecha en la cruceta: caminar a la derecha y mover el cursor de los menús."),
    ("The 2 button: jump and swim, and choose in menus.", "Le bouton 2 : sauter et nager, et choisir dans les menus.", "El botón 2: saltar y nadar, y elegir en los menús."),
    ("The 1 button: run, throw fireballs, and go back in menus.", "Le bouton 1 : courir, lancer des boules de feu, et revenir dans les menus.", "El botón 1: correr, lanzar bolas de fuego y volver atrás en los menús."),
    ("The A button: view the whole world map.", "Le bouton A : voir toute la carte du monde.", "El botón A: ver todo el mapa del mundo."),
    ("Shakes the Wii Remote: a spin jump, or picking up and throwing.", "Secoue la Wiimote : saut en vrille, ou prendre et lancer.", "Agita el mando de Wii: salto con giro, o coger y lanzar."),
    ("Tilts the Wii Remote left, for the platforms that tilt.", "Incline la Wiimote à gauche, pour les plateformes qui basculent.", "Inclina el mando de Wii a la izquierda, para las plataformas que se inclinan."),
    ("Tilts the Wii Remote right, for the platforms that tilt.", "Incline la Wiimote à droite, pour les plateformes qui basculent.", "Inclina el mando de Wii a la derecha, para las plataformas que se inclinan."),
    ("The + button: pause a course, or open the world map's menu.", "Le bouton + : mettre un niveau en pause, ou ouvrir le menu de la carte.", "El botón +: pausar un nivel, o abrir el menú del mapa."),
    ("The - button: open Select a World on the world map.", "Le bouton - : ouvrir le choix du monde sur la carte.", "El botón -: abrir la selección de mundo en el mapa."),
    ("Bindings", "Commandes", "Asignación"),
    ("Keyboard", "Clavier", "Teclado"),
    ("Controller", "Manette", "Mando"),
    ("Rumble", "Vibrations", "Vibración"),
    ("Wii Remotes", "Wiimotes", "Mandos de Wii"),
    ("Space Smart Spin", "Vrille intelligente (Espace)", "Giro inteligente (Espacio)"),
    ("Reverse Tilt", "Inclinaison inversée", "Inclinación invertida"),
    ("Mouse as Gyro", "Souris en gyroscope", "Ratón como giroscopio"),
    ("Shows the keyboard's keys or the controllers' buttons above.", "Affiche ci-dessus les touches du clavier ou les boutons des manettes.", "Muestra arriba las teclas del teclado o los botones de los mandos."),
    ("Controllers rumble when the game rumbles the Wii Remote.", "Les manettes vibrent quand le jeu fait vibrer la Wiimote.", "Los mandos vibran cuando el juego hace vibrar el mando de Wii."),
    ("Wii Remotes paired with Windows play as players 1 to 4 (not in net games).", "Les Wiimotes appairées à Windows jouent les joueurs 1 à 4 (pas en réseau).", "Los mandos de Wii emparejados con Windows juegan como jugadores 1 a 4 (no en red)."),
    ("With Space as a Jump key, Space in the air does a spin.", "Avec Espace comme touche de saut, Espace en l'air fait une vrille.", "Con Espacio como tecla de salto, Espacio en el aire hace un giro."),
    ("Platforms you tilt lean the way the keys or the stick go.", "Les plateformes à incliner penchent du côté des touches ou du stick.", "Las plataformas que inclinas se ladean hacia donde van las teclas o el stick."),
    ("Moving the mouse tilts the remote; moving it back quickly shakes it.", "Bouger la souris incline la Wiimote ; la ramener vite la secoue.", "Mover el ratón inclina el mando; devolverlo rápido lo agita."),
    ("Press a button...", "Appuyez sur un bouton...", "Pulsa un botón..."),
    ("Hold a button, or 2 or 3 together, then let go. Esc cancels, Delete clears.", "Maintenez un bouton, ou 2 ou 3 ensemble, puis relâchez. Échap annule, Suppr efface.", "Mantén un botón, o 2 o 3 a la vez, y suelta. Esc cancela, Supr borra."),
    ("Hold a key or mouse button, or 2 or 3 together, then let go. Esc cancels, Delete clears.", "Maintenez une touche ou un bouton de souris, ou 2 ou 3 ensemble, puis relâchez. Échap annule, Suppr efface.", "Mantén una tecla o un botón del ratón, o 2 o 3 a la vez, y suelta. Esc cancela, Supr borra."),
    // Gameplay.
    ("No Time Limit", "Sans limite de temps", "Sin límite de tiempo"),
    ("Infinite Lives", "Vies infinies", "Vidas infinitas"),
    ("Power-Up", "Objet", "Objeto"),
    ("The course timer stands still: take your time.", "Le chrono du niveau s'arrête : prenez votre temps.", "El tiempo del nivel se detiene: tómatelo con calma."),
    ("A miss costs no life.", "Une erreur ne coûte aucune vie.", "Fallar no cuesta ninguna vida."),
    ("Left and right pick one; press it to give it to player 1.", "Gauche et droite choisissent ; appuyez pour le donner au joueur 1.", "Izquierda y derecha eligen; púlsalo para dárselo al jugador 1."),
    ("Player 1 gets it in the next course or on the world map.", "Le joueur 1 le reçoit au prochain niveau ou sur la carte.", "El jugador 1 lo recibe en el próximo nivel o en el mapa."),
    ("Small", "Petit", "Pequeño"),
    ("Super Mushroom", "Super champignon", "Superchampiñón"),
    ("Fire Flower", "Fleur de feu", "Flor de fuego"),
    ("Mini Mushroom", "Mini champignon", "Minichampiñón"),
    ("Propeller Mushroom", "Champignon hélice", "Champiñón hélice"),
    ("Penguin Suit", "Costume de pingouin", "Traje de pingüino"),
    ("Ice Flower", "Fleur de glace", "Flor de hielo"),
    ("Star", "Étoile", "Estrella"),
    ("Are you sure?", "Vous êtes sûr ?", "¿Seguro?"),
    ("This will mark your save as cheated.", "Votre sauvegarde sera marquée comme trichée.", "Tu partida quedará marcada como hecha con trucos."),
    ("Cheated", "Triche", "Trucos"),
    ("Autosave", "Sauvegarde auto", "Autoguardado"),
    ("Exit Course", "Quitter le niveau", "Salir del nivel"),
    ("P1 Character", "Personnage J1", "Personaje J1"),
    ("The game is saved after every course you clear, as the map's Save does.", "Le jeu est sauvegardé après chaque niveau terminé, comme avec Sauvegarder sur la carte.", "La partida se guarda tras cada nivel superado, como hace Guardar en el mapa."),
    ("The pause menu's Exit Course in every course, cleared or not.", "Quitter le niveau dans le menu pause de chaque niveau, terminé ou non.", "Salir del nivel en el menú de pausa de cada nivel, superado o no."),
    ("Player 1 picks a character too, on the game's own screen (from the next start).", "Le joueur 1 choisit aussi un personnage, sur l'écran du jeu (au prochain démarrage).", "El jugador 1 también elige personaje, en la pantalla del propio juego (desde el próximo inicio)."),
    ("Power-Up Freeze", "Pause des transformations", "Pausa al transformarse"),
    ("Player Collision", "Collisions entre joueurs", "Choques entre jugadores"),
    ("Everything stops for a moment while a player grows, shrinks or dies. Off: the level keeps moving.", "Tout s'arrête un instant quand un joueur grandit, rapetisse ou perd une vie. Désactivé : le niveau continue.", "Todo se detiene un momento cuando un jugador crece, encoge o pierde una vida. Desactivado: el nivel sigue."),
    ("Players bump into and stand on each other. Off: they pass through each other.", "Les joueurs se heurtent et se tiennent les uns sur les autres. Désactivé : ils se traversent.", "Los jugadores chocan y se suben unos encima de otros. Desactivado: se atraviesan."),
    // Language.
    ("Shows the game's text and these menus in {name}.", "Affiche les textes du jeu et ces menus en {name}.", "Muestra los textos del juego y estos menús en {name}."),
    // The Multiplayer screen (LAN Play).
    ("Multiplayer", "Multijoueur", "Multijugador"),
    ("LAN Play", "Réseau local", "Red local"),
    ("Play with other PCs on your network; each one needs the game.", "Jouez avec d'autres PC de votre réseau ; chacun a besoin du jeu.", "Juega con otros PC de tu red; cada uno necesita el juego."),
    ("Your Room", "Votre salle", "Tu sala"),
    ("This Game", "Cette partie", "Esta partida"),
    ("Host a Room", "Créer une salle", "Crear una sala"),
    ("Stop Hosting", "Fermer la salle", "Cerrar la sala"),
    ("Leave the Room", "Quitter la salle", "Salir de la sala"),
    ("Join by Address", "Rejoindre par adresse", "Unirse por dirección"),
    ("Type an address", "Saisir une adresse", "Escribe una dirección"),
    ("Looking for rooms...", "Recherche de salles...", "Buscando salas..."),
    ("Type the host's address, then Enter. Esc cancels.", "Saisissez l'adresse de l'hôte, puis Entrée. Échap annule.", "Escribe la dirección del anfitrión y pulsa Intro. Esc cancela."),
    ("Others on your network can then join this PC. The game restarts.", "D'autres joueurs de votre réseau pourront rejoindre ce PC. Le jeu redémarre.", "Otros en tu red podrán unirse a este PC. El juego se reinicia."),
    ("Others join at {addr}. Stopping restarts the game.", "Les autres rejoignent à {addr}. Fermer redémarre le jeu.", "Los demás se unen en {addr}. Cerrarla reinicia el juego."),
    ("In the room at {addr}. Leaving restarts the game.", "Dans la salle à {addr}. Quitter redémarre le jeu.", "En la sala de {addr}. Salir reinicia el juego."),
    ("A room by its address, such as 192.168.1.20.", "Une salle par son adresse, par exemple 192.168.1.20.", "Una sala por su dirección, por ejemplo 192.168.1.20."),
    ("Join this room. The game restarts and joins it.", "Rejoindre cette salle. Le jeu redémarre et la rejoint.", "Unirse a esta sala. El juego se reinicia y se une a ella."),
    // Mods.
    ("HD Texture Pack", "Pack de textures HD", "Pack de texturas HD"),
    ("Download", "Télécharger", "Descargar"),
    ("Installing...", "Installation...", "Instalando..."),
    ("Try Again", "Réessayer", "Reintentar"),
    ("Pack", "Pack", "Pack"),
    ("File mods: folders in {dir} laid out as the disc's files.", "Mods de fichiers : des dossiers dans {dir} organisés comme les fichiers du disque.", "Mods de archivos: carpetas en {dir} organizadas como los archivos del disco."),
    ("Downloading from the pack's own download page.", "Téléchargement depuis la page de téléchargement du pack.", "Descargando desde la página de descarga del pack."),
    ("Unpacking the textures.", "Décompression des textures.", "Descomprimiendo las texturas."),
    ("HD textures by Bighead and friends: a 23 MB download. Or put a Dolphin pack in {dir}.", "Textures HD de Bighead et ses amis : 23 Mo à télécharger. Ou placez un pack Dolphin dans {dir}.", "Texturas HD de Bighead y compañía: una descarga de 23 MB. O pon un pack de Dolphin en {dir}."),
    ("HD textures by {by}.", "Textures HD de {by}.", "Texturas HD de {by}."),
    ("HD textures from {dir}\\{p}.", "Textures HD de {dir}\\{p}.", "Texturas HD de {dir}\\{p}."),
    ("Off: the game's own textures.", "Désactivé : les textures d'origine du jeu.", "Desactivado: las texturas originales del juego."),
    ("Left and right pick one of the {n} packs in {dir}.", "Gauche et droite choisissent l'un des {n} packs de {dir}.", "Izquierda y derecha eligen uno de los {n} packs de {dir}."),
    ("The pack in {dir}\\{p}; put others beside it to choose.", "Le pack de {dir}\\{p} ; placez-en d'autres à côté pour choisir.", "El pack de {dir}\\{p}; pon otros a su lado para elegir."),
    ("From the next start, the files in {dir}\\{name} replace the game's.", "Au prochain démarrage, les fichiers de {dir}\\{name} remplacent ceux du jeu.", "Desde el próximo inicio, los archivos de {dir}\\{name} sustituyen a los del juego."),
    // Updates.
    ("Update available", "Mise à jour disponible", "Actualización disponible"),
    ("Version {v} is out. Update and restart the game?", "La version {v} est sortie. Mettre à jour et redémarrer le jeu ?", "Ha salido la versión {v}. ¿Actualizar y reiniciar el juego?"),
    ("Version {v} is out. Open its web page?", "La version {v} est sortie. Ouvrir sa page web ?", "Ha salido la versión {v}. ¿Abrir su página web?"),
    ("Progress since your last save will be lost.", "La progression depuis la dernière sauvegarde sera perdue.", "Se perderá el progreso desde tu última partida guardada."),
    ("Updating", "Mise à jour", "Actualizando"),
    ("Downloading version {version}: {percent}%", "Téléchargement de la version {version} : {percent} %", "Descargando la versión {version}: {percent} %"),
    ("Installing version {version}...", "Installation de la version {version}...", "Instalando la versión {version}..."),
    ("Done. The game is restarting.", "Terminé. Le jeu redémarre.", "Listo. El juego se está reiniciando."),
    ("The update failed", "La mise à jour a échoué", "La actualización ha fallado"),
];

/// `english` in the menus' language (`fr`, `es`; anything else is
/// English), or as it is when the table has no line for it.
pub fn tr<'a>(language: &str, english: &'a str) -> &'a str {
    static MAP: OnceLock<HashMap<&'static str, (&'static str, &'static str)>> = OnceLock::new();
    let column = match language {
        "fr" => 0,
        "es" => 1,
        _ => return english,
    };
    match MAP.get_or_init(|| TABLE.iter().map(|&(en, fr, es)| (en, (fr, es))).collect()).get(english) {
        Some(&(fr, es)) => {
            if column == 0 {
                fr
            } else {
                es
            }
        }
        None => english,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_are_looked_up_by_their_english() {
        assert_eq!(tr("fr", "Settings"), "Paramètres");
        assert_eq!(tr("es", "Settings"), "Configuración");
        assert_eq!(tr("en", "Settings"), "Settings");
        assert_eq!(tr("fr", "A line nobody wrote"), "A line nobody wrote");
        // Each English line once, and every translation keeps its fields.
        let mut seen = std::collections::HashSet::new();
        for (en, fr, es) in TABLE {
            assert!(seen.insert(en), "{en} twice");
            for field in ["{v}", "{name}", "{version}", "{percent}", "{addr}", "{dir}", "{p}", "{by}", "{n}"] {
                assert_eq!(en.contains(field), fr.contains(field), "{en} / {fr}");
                assert_eq!(en.contains(field), es.contains(field), "{en} / {es}");
            }
        }
    }
}
