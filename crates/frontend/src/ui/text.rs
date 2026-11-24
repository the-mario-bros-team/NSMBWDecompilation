//! The port's own menus in French, Spanish, German and Italian (after
//! WitherBlitz's NSMB-Wii-Recompilation's `ui_text`): every line is looked
//! up by its English wording, so a line not in the table stays English. The
//! game's own text comes from the disc in the language picked at boot; these
//! menus follow the Settings window's choice at once.

use std::collections::HashMap;
use std::sync::OnceLock;

/// (English, French, Spanish, German, Italian).
const TABLE: &[(&str, &str, &str, &str, &str)] = &[
    // The window, its tabs and prompts.
    ("Settings", "Paramètres", "Configuración", "Einstellungen", "Impostazioni"),
    ("Display", "Affichage", "Pantalla", "Anzeige", "Schermo"),
    ("Graphics", "Graphismes", "Gráficos", "Grafik", "Grafica"),
    ("Audio", "Audio", "Audio", "Audio", "Audio"),
    ("Keybinds", "Touches", "Controles", "Tasten", "Comandi"),
    ("Gameplay", "Jeu", "Juego", "Spiel", "Gioco"),
    ("Language", "Langue", "Idioma", "Sprache", "Lingua"),
    ("Select", "Choisir", "Elegir", "Wählen", "Seleziona"),
    ("Back", "Retour", "Atrás", "Zurück", "Indietro"),
    ("On", "Oui", "Sí", "An", "Sì"),
    ("Off", "Non", "No", "Aus", "No"),
    ("Yes", "Oui", "Sí", "Ja", "Sì"),
    ("No", "Non", "No", "Nein", "No"),
    ("OK", "OK", "Aceptar", "OK", "OK"),
    ("Cancel", "Annuler", "Cancelar", "Abbrechen", "Annulla"),
    ("How the game is shown on your screen.", "Comment le jeu s'affiche sur votre écran.", "Cómo se ve el juego en tu pantalla.", "Wie das Spiel auf deinem Bildschirm erscheint.", "Come appare il gioco sul tuo schermo."),
    ("How the game is drawn, and how smoothly it moves.", "Comment le jeu est dessiné, et sa fluidité.", "Cómo se dibuja el juego y con qué fluidez se mueve.", "Wie das Spiel gezeichnet wird und wie flüssig es läuft.", "Come viene disegnato il gioco e quanto è fluido."),
    ("How loud the game's music and sounds are.", "Le volume de la musique et des sons du jeu.", "El volumen de la música y los sonidos del juego.", "Wie laut Musik und Geräusche des Spiels sind.", "Il volume della musica e dei suoni del gioco."),
    ("Which keys and controller buttons play the Wii Remote's buttons.", "Les touches et boutons de manette qui jouent les boutons de la Wiimote.", "Las teclas y botones del mando que hacen de botones del mando de Wii.", "Welche Tasten und Controller-Knöpfe die Knöpfe der Wii-Fernbedienung ersetzen.", "Quali tasti e pulsanti del controller fanno da pulsanti del telecomando Wii."),
    ("Optional helpers. Online, only Exit Course applies, if every player has it on.", "Aides facultatives. En ligne, seul Quitter le niveau s'applique, si tous les joueurs l'ont activé.", "Ayudas opcionales. En línea solo se aplica Salir del nivel, si todos los jugadores lo tienen activado.", "Optionale Hilfen. Online gilt nur Level verlassen, wenn alle Spieler es aktiviert haben.", "Aiuti facoltativi. Online vale solo Esci dal livello, se tutti i giocatori lo hanno attivo."),
    ("Mods", "Mods", "Mods", "Mods", "Mod"),
    ("HD textures, and mods that change the game's files.", "Textures HD, et mods qui changent les fichiers du jeu.", "Texturas HD y mods que cambian los archivos del juego.", "HD-Texturen und Mods, die die Dateien des Spiels ändern.", "Texture HD e mod che cambiano i file del gioco."),
    ("The language of the game's text and these menus.", "La langue des textes du jeu et de ces menus.", "El idioma de los textos del juego y de estos menús.", "Die Sprache der Texte des Spiels und dieser Menüs.", "La lingua dei testi del gioco e di questi menu."),
    // Display.
    ("Resolution", "Résolution", "Resolución", "Auflösung", "Risoluzione"),
    ("Aspect Ratio", "Format d'image", "Formato de imagen", "Seitenverhältnis", "Formato"),
    ("Fullscreen", "Plein écran", "Pantalla completa", "Vollbild", "Schermo intero"),
    ("V-Sync", "V-Sync", "V-Sync", "V-Sync", "V-Sync"),
    ("Performance Monitor", "Moniteur de performances", "Monitor de rendimiento", "Leistungsanzeige", "Monitor prestazioni"),
    ("Native", "Native", "Nativa", "Nativ", "Nativa"),
    ("Original 4:3", "4:3 d'origine", "4:3 original", "Original 4:3", "4:3 originale"),
    ("The window's own resolution (from the next start).", "La résolution de la fenêtre (au prochain démarrage).", "La resolución de la ventana (al próximo inicio).", "Die Auflösung des Fensters (ab dem nächsten Start).", "La risoluzione della finestra (dal prossimo avvio)."),
    ("Rendered at this many lines, then scaled to the screen (from the next start).", "Rendu à ce nombre de lignes, puis mis à l'échelle (au prochain démarrage).", "Se dibuja con estas líneas y se escala a la pantalla (al próximo inicio).", "Mit so vielen Zeilen gezeichnet, dann auf den Bildschirm skaliert (ab dem nächsten Start).", "Disegnato con queste linee, poi scalato sullo schermo (dal prossimo avvio)."),
    ("Switching between 4:3 and widescreen takes effect after a restart.", "Le passage entre 4:3 et 16:9 prend effet au redémarrage.", "El cambio entre 4:3 y panorámico se aplica al reiniciar.", "Der Wechsel zwischen 4:3 und Breitbild gilt nach einem Neustart.", "Il passaggio tra 4:3 e panoramico vale dopo un riavvio."),
    ("Borderless: a window covering the whole screen (F11).", "Sans bordure : une fenêtre qui couvre tout l'écran (F11).", "Sin bordes: una ventana que cubre toda la pantalla (F11).", "Randlos: ein Fenster, das den ganzen Bildschirm füllt (F11).", "Senza bordi: una finestra che copre tutto lo schermo (F11)."),
    ("Off: the game plays in a window.", "Non : le jeu s'affiche dans une fenêtre.", "No: el juego se muestra en una ventana.", "Aus: Das Spiel läuft in einem Fenster.", "No: il gioco è in una finestra."),
    ("No tearing: frames wait for the display, the game keeps its pace.", "Sans déchirement : les images attendent l'écran, le jeu garde son rythme.", "Sin cortes: las imágenes esperan a la pantalla y el juego mantiene su ritmo.", "Kein Tearing: Bilder warten auf den Bildschirm, das Spiel behält sein Tempo.", "Niente tearing: i fotogrammi aspettano lo schermo, il gioco mantiene il ritmo."),
    ("Shows the FPS and each frame's time at the top left; left and right set its size.", "Affiche les FPS et le temps de chaque image en haut à gauche ; gauche et droite règlent sa taille.", "Muestra los FPS y el tiempo de cada imagen arriba a la izquierda; izquierda y derecha cambian su tamaño.", "Zeigt oben links die FPS und die Zeit jedes Bildes; links und rechts ändern die Größe.", "Mostra in alto a sinistra gli FPS e il tempo di ogni fotogramma; sinistra e destra ne cambiano la dimensione."),
    ("Ultrawide", "Ultra-large", "Ultrapanorámica", "Ultrabreit", "Ultrapanoramico"),
    ("The Wii's original 4:3 picture.", "L'image 4:3 d'origine de la Wii.", "La imagen 4:3 original de la Wii.", "Das originale 4:3-Bild der Wii.", "L'immagine 4:3 originale della Wii."),
    ("The game's 16:9 widescreen picture.", "L'image 16:9 du jeu en écran large.", "La imagen panorámica 16:9 del juego.", "Das 16:9-Breitbild des Spiels.", "L'immagine panoramica 16:9 del gioco."),
    ("For a 16:10 screen: fills it, with a slightly narrower view.", "Pour un écran 16:10 : le remplit, avec une vue un peu plus étroite.", "Para una pantalla 16:10: la llena, con una vista algo más estrecha.", "Für einen 16:10-Bildschirm: füllt ihn, mit etwas schmalerer Sicht.", "Per uno schermo 16:10: lo riempie, con una visuale un po' più stretta."),
    ("Fills a screen wider than 16:9 with a wider view (16:9 on others).", "Remplit un écran plus large que 16:9 avec une vue plus large (16:9 sur les autres).", "Llena una pantalla más ancha que 16:9 con una vista más amplia (16:9 en las demás).", "Füllt einen Bildschirm breiter als 16:9 mit weiterer Sicht (sonst 16:9).", "Riempie uno schermo più largo di 16:9 con una visuale più ampia (16:9 sugli altri)."),
    // Graphics.
    ("Spinning Coins", "Pièces animées", "Monedas giratorias", "Drehende Münzen", "Monete rotanti"),
    ("Depth Blur", "Flou de profondeur", "Desenfoque de profundidad", "Tiefenunschärfe", "Sfocatura di profondità"),
    ("Animates coins and blocks; off saves a little GPU work.", "Anime les pièces et les blocs ; désactivé, épargne un peu le GPU.", "Anima las monedas y los bloques; desactivado ahorra algo de trabajo a la GPU.", "Animiert Münzen und Blöcke; aus spart der GPU etwas Arbeit.", "Anima monete e blocchi; disattivato fa risparmiare un po' di lavoro alla GPU."),
    ("The blur and heat haze of desert, beach, water and lava areas.", "Le flou et la brume de chaleur des zones de désert, de plage, d'eau et de lave.", "El desenfoque y la calima de las zonas de desierto, playa, agua y lava.", "Unschärfe und Hitzeflimmern in Wüste, Strand, Wasser und Lava.", "La sfocatura e la foschia di calore di deserto, spiaggia, acqua e lava."),
    ("Renderer", "Rendu", "Renderizador", "Renderer", "Renderer"),
    ("Battery Saver", "Économie d'énergie", "Ahorro de batería", "Energiesparen", "Risparmio energetico"),
    ("Frame Interpolation", "Interpolation", "Interpolación", "Zwischenbilder", "Interpolazione"),
    ("Unlocked", "Illimité", "Sin límite", "Unbegrenzt", "Illimitato"),
    ("Automatic", "Automatique", "Automático", "Automatisch", "Automatico"),
    ("Vulkan, else Direct3D 12, else Direct3D 11 (from the next start).", "Vulkan, sinon Direct3D 12, sinon Direct3D 11 (au prochain démarrage).", "Vulkan; si no, Direct3D 12; si no, Direct3D 11 (al próximo inicio).", "Vulkan, sonst Direct3D 12, sonst Direct3D 11 (ab dem nächsten Start).", "Vulkan, altrimenti Direct3D 12, altrimenti Direct3D 11 (dal prossimo avvio)."),
    ("The graphics API the game draws with (from the next start).", "L'API graphique avec laquelle le jeu dessine (au prochain démarrage).", "La API gráfica con la que dibuja el juego (al próximo inicio).", "Die Grafik-API, mit der das Spiel zeichnet (ab dem nächsten Start).", "L'API grafica con cui disegna il gioco (dal prossimo avvio)."),
    ("Shows 30 frames a second while the game keeps running at 60.", "Affiche 30 images par seconde ; le jeu tourne toujours à 60.", "Muestra 30 imágenes por segundo; el juego sigue a 60.", "Zeigt 30 Bilder pro Sekunde; das Spiel läuft weiter mit 60.", "Mostra 30 fotogrammi al secondo; il gioco resta a 60."),
    ("Draws frames in between for smoother motion on screens above 60Hz.", "Ajoute des images intermédiaires pour plus de fluidité au-delà de 60 Hz.", "Dibuja imágenes intermedias para más fluidez en pantallas de más de 60 Hz.", "Zeichnet Zwischenbilder für flüssigere Bewegung auf Bildschirmen über 60 Hz.", "Disegna fotogrammi intermedi per più fluidità su schermi oltre 60 Hz."),
    ("Draws as many frames in between as the PC can, the game still at its own speed.", "Ajoute autant d'images intermédiaires que le PC le permet ; le jeu garde sa vitesse.", "Dibuja tantas imágenes intermedias como pueda el PC; el juego mantiene su velocidad.", "Zeichnet so viele Zwischenbilder, wie der PC schafft; das Spiel behält sein Tempo.", "Disegna tutti i fotogrammi intermedi che il PC riesce; il gioco mantiene la sua velocità."),
    // Audio.
    ("Master", "Général", "General", "Gesamt", "Generale"),
    ("Music", "Musique", "Música", "Musik", "Musica"),
    ("Sound Effects", "Effets sonores", "Efectos de sonido", "Soundeffekte", "Effetti sonori"),
    ("Menu Sounds", "Sons des menus", "Sonidos de menú", "Menütöne", "Suoni dei menu"),
    ("The volume of everything.", "Le volume de tout.", "El volumen de todo.", "Die Lautstärke von allem.", "Il volume di tutto."),
    ("The background music.", "La musique de fond.", "La música de fondo.", "Die Hintergrundmusik.", "La musica di sottofondo."),
    ("Jumps, coins, enemies and the rest.", "Sauts, pièces, ennemis et le reste.", "Saltos, monedas, enemigos y todo lo demás.", "Sprünge, Münzen, Gegner und der Rest.", "Salti, monete, nemici e il resto."),
    ("The menus' cursor, select and back sounds.", "Les sons de curseur, de choix et de retour des menus.", "Los sonidos de cursor, elección y retroceso de los menús.", "Die Cursor-, Auswahl- und Zurück-Töne der Menüs.", "I suoni di cursore, scelta e indietro dei menu."),
    // Keybinds.
    ("Up", "Haut", "Arriba", "Oben", "Su"),
    ("Down", "Bas", "Abajo", "Unten", "Giù"),
    ("Left", "Gauche", "Izquierda", "Links", "Sinistra"),
    ("Right", "Droite", "Derecha", "Rechts", "Destra"),
    ("Jump (2)", "Sauter (2)", "Saltar (2)", "Springen (2)", "Salta (2)"),
    ("Run / Fire (1)", "Courir / Tirer (1)", "Correr / Disparar (1)", "Rennen / Feuer (1)", "Corri / Spara (1)"),
    ("Shake (Spin)", "Secouer (Vrille)", "Agitar (Giro)", "Schütteln/Drehen", "Scuoti (Giro)"),
    ("Tilt Left", "Incliner G", "Inclinar izq.", "Links neigen", "Inclina a sin."),
    ("Tilt Right", "Incliner D", "Inclinar der.", "Rechts neigen", "Inclina a des."),
    ("+ (Pause)", "+ (Pause)", "+ (Pausa)", "+ (Pause)", "+ (Pausa)"),
    ("- (Minus)", "- (Moins)", "- (Menos)", "- (Minus)", "- (Meno)"),
    ("Press a key...", "Appuyez sur une touche...", "Pulsa una tecla...", "Drücke eine Taste...", "Premi un tasto..."),
    ("None", "Aucune", "Ninguna", "Keine", "Nessuno"),
    ("Press the new key. Esc cancels; Delete or Backspace leaves it no key.", "Appuyez sur la nouvelle touche. Échap annule ; Suppr ou Retour arrière n'en laisse aucune.", "Pulsa la nueva tecla. Esc cancela; Supr o Retroceso la deja sin tecla.", "Drücke die neue Taste. Esc bricht ab; Entf oder Rücktaste lässt sie frei.", "Premi il nuovo tasto. Esc annulla; Canc o Backspace lo lascia senza tasto."),
    ("Controllers: A jumps, B and X run, Y shakes, Start is +, Back is -.", "Manettes : A saute, B et X courent, Y secoue, Start est +, Back est -.", "Mandos: A salta, B y X corren, Y agita, Start es +, Back es -.", "Controller: A springt, B und X rennen, Y schüttelt, Start ist +, Back ist -.", "Controller: A salta, B e X corrono, Y scuote, Start è +, Back è -."),
    ("The D-pad's up: climb vines, and go through doors and pipes.", "Le haut de la croix : grimper aux lianes, passer portes et tuyaux.", "Arriba en la cruceta: trepar enredaderas y entrar por puertas y tuberías.", "Steuerkreuz oben: an Ranken klettern, durch Türen und Röhren gehen.", "Su sulla croce: arrampicarsi sui rampicanti, entrare in porte e tubi."),
    ("The D-pad's down: crouch, and ground-pound in the air.", "Le bas de la croix : s'accroupir, et attaque rodéo en l'air.", "Abajo en la cruceta: agacharse, y golpe de trasero en el aire.", "Steuerkreuz unten: ducken, und in der Luft eine Stampfattacke.", "Giù sulla croce: abbassarsi, e schianto a terra in aria."),
    ("The D-pad's left: walk left, and move a menu's cursor.", "La gauche de la croix : marcher à gauche, et déplacer le curseur des menus.", "Izquierda en la cruceta: caminar a la izquierda y mover el cursor de los menús.", "Steuerkreuz links: nach links gehen und den Cursor in Menüs bewegen.", "Sinistra sulla croce: camminare a sinistra e muovere il cursore dei menu."),
    ("The D-pad's right: walk right, and move a menu's cursor.", "La droite de la croix : marcher à droite, et déplacer le curseur des menus.", "Derecha en la cruceta: caminar a la derecha y mover el cursor de los menús.", "Steuerkreuz rechts: nach rechts gehen und den Cursor in Menüs bewegen.", "Destra sulla croce: camminare a destra e muovere il cursore dei menu."),
    ("The 2 button: jump and swim, and choose in menus.", "Le bouton 2 : sauter et nager, et choisir dans les menus.", "El botón 2: saltar y nadar, y elegir en los menús.", "Der 2-Knopf: springen und schwimmen, und in Menüs wählen.", "Il pulsante 2: saltare e nuotare, e scegliere nei menu."),
    ("The 1 button: run, throw fireballs, and go back in menus.", "Le bouton 1 : courir, lancer des boules de feu, et revenir dans les menus.", "El botón 1: correr, lanzar bolas de fuego y volver atrás en los menús.", "Der 1-Knopf: rennen, Feuerbälle werfen, und in Menüs zurück.", "Il pulsante 1: correre, lanciare palle di fuoco e tornare indietro nei menu."),
    ("The A button: view the whole world map.", "Le bouton A : voir toute la carte du monde.", "El botón A: ver todo el mapa del mundo.", "Der A-Knopf: die ganze Karte der Welt ansehen.", "Il pulsante A: vedere tutta la mappa del mondo."),
    ("Shakes the Wii Remote: a spin jump, or picking up and throwing.", "Secoue la Wiimote : saut en vrille, ou prendre et lancer.", "Agita el mando de Wii: salto con giro, o coger y lanzar.", "Schüttelt die Wii-Fernbedienung: Drehsprung, oder aufheben und werfen.", "Scuote il telecomando Wii: salto rotante, o prendere e lanciare."),
    ("Tilts the Wii Remote left, for the platforms that tilt.", "Incline la Wiimote à gauche, pour les plateformes qui basculent.", "Inclina el mando de Wii a la izquierda, para las plataformas que se inclinan.", "Neigt die Wii-Fernbedienung nach links, für kippbare Plattformen.", "Inclina il telecomando Wii a sinistra, per le piattaforme che si inclinano."),
    ("Tilts the Wii Remote right, for the platforms that tilt.", "Incline la Wiimote à droite, pour les plateformes qui basculent.", "Inclina el mando de Wii a la derecha, para las plataformas que se inclinan.", "Neigt die Wii-Fernbedienung nach rechts, für kippbare Plattformen.", "Inclina il telecomando Wii a destra, per le piattaforme che si inclinano."),
    ("The + button: pause a course, or open the world map's menu.", "Le bouton + : mettre un niveau en pause, ou ouvrir le menu de la carte.", "El botón +: pausar un nivel, o abrir el menú del mapa.", "Der +Knopf: einen Level pausieren oder das Kartenmenü öffnen.", "Il pulsante +: mettere in pausa un livello, o aprire il menu della mappa."),
    ("The - button: open Select a World on the world map.", "Le bouton - : ouvrir le choix du monde sur la carte.", "El botón -: abrir la selección de mundo en el mapa.", "Der -Knopf: auf der Karte Welt wählen öffnen.", "Il pulsante -: aprire Seleziona un mondo sulla mappa."),
    ("Bindings", "Commandes", "Asignación", "Belegung", "Assegnazione"),
    ("Keyboard", "Clavier", "Teclado", "Tastatur", "Tastiera"),
    ("Controller", "Manette", "Mando", "Controller", "Controller"),
    ("Rumble", "Vibrations", "Vibración", "Vibration", "Vibrazione"),
    ("Wii Remotes", "Wiimotes", "Mandos de Wii", "Wii-Fernbedienungen", "Telecomandi Wii"),
    ("Space Smart Spin", "Vrille intelligente (Espace)", "Giro inteligente (Espacio)", "Leertaste-Drehung", "Giro smart (Spazio)"),
    ("Reverse Tilt", "Inclinaison inversée", "Inclinación invertida", "Neigung umkehren", "Inclinazione invertita"),
    ("Mouse as Gyro", "Souris en gyroscope", "Ratón como giroscopio", "Maus als Gyro", "Mouse come giroscopio"),
    ("Shows the keyboard's keys or the controllers' buttons above.", "Affiche ci-dessus les touches du clavier ou les boutons des manettes.", "Muestra arriba las teclas del teclado o los botones de los mandos.", "Zeigt oben die Tasten der Tastatur oder die Knöpfe der Controller.", "Mostra sopra i tasti della tastiera o i pulsanti dei controller."),
    ("Controllers rumble when the game rumbles the Wii Remote.", "Les manettes vibrent quand le jeu fait vibrer la Wiimote.", "Los mandos vibran cuando el juego hace vibrar el mando de Wii.", "Controller vibrieren, wenn das Spiel die Wii-Fernbedienung vibrieren lässt.", "I controller vibrano quando il gioco fa vibrare il telecomando Wii."),
    ("Wii Remotes paired with Windows play as players 1 to 4 (not in net games).", "Les Wiimotes appairées à Windows jouent les joueurs 1 à 4 (pas en réseau).", "Los mandos de Wii emparejados con Windows juegan como jugadores 1 a 4 (no en red).", "Mit Windows gekoppelte Wii-Fernbedienungen spielen als Spieler 1 bis 4 (nicht im Netz).", "I telecomandi Wii associati a Windows sono i giocatori da 1 a 4 (non in rete)."),
    ("With Space as a Jump key, Space in the air does a spin.", "Avec Espace comme touche de saut, Espace en l'air fait une vrille.", "Con Espacio como tecla de salto, Espacio en el aire hace un giro.", "Ist die Leertaste eine Sprungtaste, macht sie in der Luft eine Drehung.", "Con Spazio come tasto di salto, Spazio in aria fa un giro."),
    ("Platforms you tilt lean the way the keys or the stick go.", "Les plateformes à incliner penchent du côté des touches ou du stick.", "Las plataformas que inclinas se ladean hacia donde van las teclas o el stick.", "Kippbare Plattformen neigen sich in Richtung der Tasten oder des Sticks.", "Le piattaforme da inclinare pendono verso i tasti o lo stick."),
    ("Moving the mouse tilts the remote; moving it back quickly shakes it.", "Bouger la souris incline la Wiimote ; la ramener vite la secoue.", "Mover el ratón inclina el mando; devolverlo rápido lo agita.", "Die Maus bewegen neigt die Fernbedienung; schnell zurück schüttelt sie.", "Muovere il mouse inclina il telecomando; riportarlo indietro in fretta lo scuote."),
    ("Press a button...", "Appuyez sur un bouton...", "Pulsa un botón...", "Drücke einen Knopf...", "Premi un pulsante..."),
    ("Hold a button, or 2 or 3 together, then let go. Esc cancels, Delete clears.", "Maintenez un bouton, ou 2 ou 3 ensemble, puis relâchez. Échap annule, Suppr efface.", "Mantén un botón, o 2 o 3 a la vez, y suelta. Esc cancela, Supr borra.", "Halte einen Knopf, oder 2 oder 3 zusammen, und lass los. Esc bricht ab, Entf löscht.", "Tieni premuto un pulsante, o 2 o 3 insieme, poi rilascia. Esc annulla, Canc cancella."),
    ("Hold a key or mouse button, or 2 or 3 together, then let go. Esc cancels, Delete clears.", "Maintenez une touche ou un bouton de souris, ou 2 ou 3 ensemble, puis relâchez. Échap annule, Suppr efface.", "Mantén una tecla o un botón del ratón, o 2 o 3 a la vez, y suelta. Esc cancela, Supr borra.", "Halte eine Taste oder Maustaste, oder 2 oder 3 zusammen, und lass los. Esc bricht ab, Entf löscht.", "Tieni premuto un tasto o un pulsante del mouse, o 2 o 3 insieme, poi rilascia. Esc annulla, Canc cancella."),
    // Gameplay.
    ("No Time Limit", "Sans limite de temps", "Sin límite de tiempo", "Kein Zeitlimit", "Tempo infinito"),
    ("Infinite Lives", "Vies infinies", "Vidas infinitas", "Unendlich Leben", "Vite infinite"),
    ("Power-Up", "Objet", "Objeto", "Power-Up", "Potenziamento"),
    ("The course timer stands still: take your time.", "Le chrono du niveau s'arrête : prenez votre temps.", "El tiempo del nivel se detiene: tómatelo con calma.", "Die Zeit im Level steht still: Lass dir Zeit.", "Il tempo del livello si ferma: fai con calma."),
    ("A miss costs no life.", "Une erreur ne coûte aucune vie.", "Fallar no cuesta ninguna vida.", "Ein Fehler kostet kein Leben.", "Perdere non costa vite."),
    ("Left and right pick one; press it to give it to player 1.", "Gauche et droite choisissent ; appuyez pour le donner au joueur 1.", "Izquierda y derecha eligen; púlsalo para dárselo al jugador 1.", "Links und rechts wählen; drücken gibt es Spieler 1.", "Sinistra e destra scelgono; premi per darlo al giocatore 1."),
    ("Player 1 gets it in the next course or on the world map.", "Le joueur 1 le reçoit au prochain niveau ou sur la carte.", "El jugador 1 lo recibe en el próximo nivel o en el mapa.", "Spieler 1 erhält es im nächsten Level oder auf der Karte.", "Il giocatore 1 lo riceve nel prossimo livello o sulla mappa."),
    ("Small", "Petit", "Pequeño", "Klein", "Piccolo"),
    ("Super Mushroom", "Super champignon", "Superchampiñón", "Superpilz", "Super fungo"),
    ("Fire Flower", "Fleur de feu", "Flor de fuego", "Feuerblume", "Fiore di fuoco"),
    ("Mini Mushroom", "Mini champignon", "Minichampiñón", "Mini-Pilz", "Minifungo"),
    ("Propeller Mushroom", "Champignon hélice", "Champiñón hélice", "Propeller-Pilz", "Fungo elica"),
    ("Penguin Suit", "Costume de pingouin", "Traje de pingüino", "Pinguin-Anzug", "Costume da pinguino"),
    ("Ice Flower", "Fleur de glace", "Flor de hielo", "Eisblume", "Fiore di ghiaccio"),
    ("Star", "Étoile", "Estrella", "Stern", "Stella"),
    ("Are you sure?", "Vous êtes sûr ?", "¿Seguro?", "Bist du sicher?", "Sei sicuro?"),
    ("This will mark your save as cheated.", "Votre sauvegarde sera marquée comme trichée.", "Tu partida quedará marcada como hecha con trucos.", "Dein Spielstand wird als geschummelt markiert.", "Il salvataggio verrà segnato come truccato."),
    ("Cheated", "Triche", "Trucos", "Geschummelt", "Trucchi"),
    ("Autosave", "Sauvegarde auto", "Autoguardado", "Autom. speichern", "Salvataggio auto"),
    ("Exit Course", "Quitter le niveau", "Salir del nivel", "Level verlassen", "Esci dal livello"),
    ("P1 Character", "Personnage J1", "Personaje J1", "Figur S1", "Personaggio G1"),
    ("The game is saved after every course you clear, as the map's Save does.", "Le jeu est sauvegardé après chaque niveau terminé, comme avec Sauvegarder sur la carte.", "La partida se guarda tras cada nivel superado, como hace Guardar en el mapa.", "Nach jedem geschafften Level wird gespeichert, wie mit Speichern auf der Karte.", "Si salva dopo ogni livello completato, come fa Salva sulla mappa."),
    ("The pause menu's Exit Course in every course, cleared or not.", "Quitter le niveau dans le menu pause de chaque niveau, terminé ou non.", "Salir del nivel en el menú de pausa de cada nivel, superado o no.", "Level verlassen im Pausemenü jedes Levels, geschafft oder nicht.", "Esci dal livello nel menu di pausa di ogni livello, completato o no."),
    ("Player 1 picks a character too, on the game's own screen (from the next start).", "Le joueur 1 choisit aussi un personnage, sur l'écran du jeu (au prochain démarrage).", "El jugador 1 también elige personaje, en la pantalla del propio juego (desde el próximo inicio).", "Auch Spieler 1 wählt eine Figur, im Bildschirm des Spiels (ab dem nächsten Start).", "Anche il giocatore 1 sceglie un personaggio, nella schermata del gioco (dal prossimo avvio)."),
    ("Power-Up Freeze", "Pause des transformations", "Pausa al transformarse", "Verwandlungspause", "Pausa trasformazioni"),
    ("Player Collision", "Collisions entre joueurs", "Choques entre jugadores", "Spieler-Kollision", "Urti tra giocatori"),
    ("Everything stops for a moment while a player grows, shrinks or dies. Off: the level keeps moving.", "Tout s'arrête un instant quand un joueur grandit, rapetisse ou perd une vie. Désactivé : le niveau continue.", "Todo se detiene un momento cuando un jugador crece, encoge o pierde una vida. Desactivado: el nivel sigue.", "Alles hält kurz an, wenn ein Spieler wächst, schrumpft oder ein Leben verliert. Aus: Der Level läuft weiter.", "Tutto si ferma un momento quando un giocatore cresce, si rimpicciolisce o perde una vita. Disattivato: il livello continua."),
    ("Players bump into and stand on each other. Off: they pass through each other.", "Les joueurs se heurtent et se tiennent les uns sur les autres. Désactivé : ils se traversent.", "Los jugadores chocan y se suben unos encima de otros. Desactivado: se atraviesan.", "Spieler stoßen zusammen und stehen aufeinander. Aus: Sie gehen durcheinander hindurch.", "I giocatori si urtano e salgono uno sull'altro. Disattivato: si attraversano."),
    ("Motion Sensors", "Capteurs de mouvement", "Sensores de movimiento", "Bewegungssensoren", "Sensori di movimento"),
    ("Bluetooth Passthrough", "Bluetooth direct", "Bluetooth directo", "Bluetooth direkt", "Bluetooth diretto"),
    ("A controller's motion sensors tilt (turn it like a wheel) and shake (jolt it) its remote.", "Les capteurs de mouvement d'une manette inclinent (tournez-la comme un volant) et secouent (donnez-lui un coup) sa Wiimote.", "Los sensores de movimiento de un mando inclinan (gíralo como un volante) y agitan (sacúdelo) su mando de Wii.", "Die Bewegungssensoren eines Controllers neigen (wie ein Lenkrad drehen) und schütteln (ruckartig bewegen) seine Fernbedienung.", "I sensori di movimento di un controller inclinano (giralo come un volante) e scuotono (dagli uno scatto) il suo telecomando."),
    ("Experimental: real Wii Remotes on a WinUSB Bluetooth adapter; F8 pairs (from the next start).", "Expérimental : de vraies Wiimotes sur un adaptateur Bluetooth WinUSB ; F8 les appaire (au prochain démarrage).", "Experimental: mandos de Wii reales en un adaptador Bluetooth WinUSB; F8 los empareja (al próximo inicio).", "Experimentell: echte Wii-Fernbedienungen an einem WinUSB-Bluetooth-Adapter; F8 koppelt (ab dem nächsten Start).", "Sperimentale: veri telecomandi Wii su un adattatore Bluetooth WinUSB; F8 li associa (dal prossimo avvio)."),
    // Language.
    ("Shows the game's text and these menus in {name}.", "Affiche les textes du jeu et ces menus en {name}.", "Muestra los textos del juego y estos menús en {name}.", "Zeigt die Texte des Spiels und diese Menüs auf {name}.", "Mostra i testi del gioco e questi menu in {name}."),
    // The Multiplayer screen (LAN Play).
    ("Multiplayer", "Multijoueur", "Multijugador", "Mehrspieler", "Multigiocatore"),
    ("LAN Play", "Réseau local", "Red local", "LAN-Spiel", "Gioco in LAN"),
    ("Play with other PCs on your network; each one needs the game.", "Jouez avec d'autres PC de votre réseau ; chacun a besoin du jeu.", "Juega con otros PC de tu red; cada uno necesita el juego.", "Spiele mit anderen PCs in deinem Netzwerk; jeder braucht das Spiel.", "Gioca con altri PC della tua rete; ognuno ha bisogno del gioco."),
    ("Your Room", "Votre salle", "Tu sala", "Dein Raum", "La tua stanza"),
    ("This Game", "Cette partie", "Esta partida", "Dieses Spiel", "Questa partita"),
    ("Host a Room", "Créer une salle", "Crear una sala", "Raum eröffnen", "Crea una stanza"),
    ("Stop Hosting", "Fermer la salle", "Cerrar la sala", "Raum schließen", "Chiudi la stanza"),
    ("Leave the Room", "Quitter la salle", "Salir de la sala", "Raum verlassen", "Esci dalla stanza"),
    ("Join by Address", "Rejoindre par adresse", "Unirse por dirección", "Per Adresse beitreten", "Entra con indirizzo"),
    ("Type an address", "Saisir une adresse", "Escribe una dirección", "Adresse eingeben", "Scrivi un indirizzo"),
    ("Looking for rooms...", "Recherche de salles...", "Buscando salas...", "Suche Räume...", "Ricerca delle stanze..."),
    ("Type the host's address, then Enter. Esc cancels.", "Saisissez l'adresse de l'hôte, puis Entrée. Échap annule.", "Escribe la dirección del anfitrión y pulsa Intro. Esc cancela.", "Gib die Adresse des Hosts ein, dann Enter. Esc bricht ab.", "Scrivi l'indirizzo di chi ospita, poi Invio. Esc annulla."),
    ("Others on your network can then join this PC. The game restarts.", "D'autres joueurs de votre réseau pourront rejoindre ce PC. Le jeu redémarre.", "Otros en tu red podrán unirse a este PC. El juego se reinicia.", "Andere in deinem Netzwerk können dann diesem PC beitreten. Das Spiel startet neu.", "Altri sulla tua rete potranno entrare in questo PC. Il gioco si riavvia."),
    ("Others join at {addr}. Stopping restarts the game.", "Les autres rejoignent à {addr}. Fermer redémarre le jeu.", "Los demás se unen en {addr}. Cerrarla reinicia el juego.", "Andere treten unter {addr} bei. Schließen startet das Spiel neu.", "Gli altri entrano da {addr}. Chiuderla riavvia il gioco."),
    ("In the room at {addr}. Leaving restarts the game.", "Dans la salle à {addr}. Quitter redémarre le jeu.", "En la sala de {addr}. Salir reinicia el juego.", "Im Raum unter {addr}. Verlassen startet das Spiel neu.", "Nella stanza di {addr}. Uscire riavvia il gioco."),
    ("A room by its address, such as 192.168.1.20.", "Une salle par son adresse, par exemple 192.168.1.20.", "Una sala por su dirección, por ejemplo 192.168.1.20.", "Ein Raum über seine Adresse, z. B. 192.168.1.20.", "Una stanza tramite il suo indirizzo, ad esempio 192.168.1.20."),
    ("Join this room. The game restarts and joins it.", "Rejoindre cette salle. Le jeu redémarre et la rejoint.", "Unirse a esta sala. El juego se reinicia y se une a ella.", "Diesem Raum beitreten. Das Spiel startet neu und tritt bei.", "Entra in questa stanza. Il gioco si riavvia ed entra."),
    // Mods.
    ("HD Texture Pack", "Pack de textures HD", "Pack de texturas HD", "HD-Texturpaket", "Pacchetto texture HD"),
    ("Download", "Télécharger", "Descargar", "Herunterladen", "Scarica"),
    ("Installing...", "Installation...", "Instalando...", "Installiere...", "Installazione..."),
    ("Try Again", "Réessayer", "Reintentar", "Erneut versuchen", "Riprova"),
    ("Pack", "Pack", "Pack", "Paket", "Pacchetto"),
    ("File mods: folders in {dir} laid out as the disc's files.", "Mods de fichiers : des dossiers dans {dir} organisés comme les fichiers du disque.", "Mods de archivos: carpetas en {dir} organizadas como los archivos del disco.", "Datei-Mods: Ordner in {dir}, aufgebaut wie die Dateien der Disc.", "Mod dei file: cartelle in {dir} organizzate come i file del disco."),
    ("Downloading from the pack's own download page.", "Téléchargement depuis la page de téléchargement du pack.", "Descargando desde la página de descarga del pack.", "Lade von der Download-Seite des Pakets herunter.", "Download dalla pagina di download del pacchetto."),
    ("Unpacking the textures.", "Décompression des textures.", "Descomprimiendo las texturas.", "Entpacke die Texturen.", "Estrazione delle texture."),
    ("HD textures by Bighead and friends: a 23 MB download. Or put a Dolphin pack in {dir}.", "Textures HD de Bighead et ses amis : 23 Mo à télécharger. Ou placez un pack Dolphin dans {dir}.", "Texturas HD de Bighead y compañía: una descarga de 23 MB. O pon un pack de Dolphin en {dir}.", "HD-Texturen von Bighead und Freunden: 23 MB Download. Oder lege ein Dolphin-Paket in {dir}.", "Texture HD di Bighead e amici: un download di 23 MB. Oppure metti un pacchetto Dolphin in {dir}."),
    ("HD textures by {by}.", "Textures HD de {by}.", "Texturas HD de {by}.", "HD-Texturen von {by}.", "Texture HD di {by}."),
    ("HD textures from {dir}\\{p}.", "Textures HD de {dir}\\{p}.", "Texturas HD de {dir}\\{p}.", "HD-Texturen aus {dir}\\{p}.", "Texture HD da {dir}\\{p}."),
    ("Off: the game's own textures.", "Désactivé : les textures d'origine du jeu.", "Desactivado: las texturas originales del juego.", "Aus: die Originaltexturen des Spiels.", "Disattivato: le texture originali del gioco."),
    ("Left and right pick one of the {n} packs in {dir}.", "Gauche et droite choisissent l'un des {n} packs de {dir}.", "Izquierda y derecha eligen uno de los {n} packs de {dir}.", "Links und rechts wählen eines der {n} Pakete in {dir}.", "Sinistra e destra scelgono uno dei {n} pacchetti in {dir}."),
    ("The pack in {dir}\\{p}; put others beside it to choose.", "Le pack de {dir}\\{p} ; placez-en d'autres à côté pour choisir.", "El pack de {dir}\\{p}; pon otros a su lado para elegir.", "Das Paket in {dir}\\{p}; lege weitere daneben, um zu wählen.", "Il pacchetto in {dir}\\{p}; mettine altri accanto per scegliere."),
    ("From the next start, the files in {dir}\\{name} replace the game's.", "Au prochain démarrage, les fichiers de {dir}\\{name} remplacent ceux du jeu.", "Desde el próximo inicio, los archivos de {dir}\\{name} sustituyen a los del juego.", "Ab dem nächsten Start ersetzen die Dateien in {dir}\\{name} die des Spiels.", "Dal prossimo avvio, i file in {dir}\\{name} sostituiscono quelli del gioco."),
    // Updates.
    ("Update available", "Mise à jour disponible", "Actualización disponible", "Update verfügbar", "Aggiornamento disponibile"),
    ("Version {v} is out. Update and restart the game?", "La version {v} est sortie. Mettre à jour et redémarrer le jeu ?", "Ha salido la versión {v}. ¿Actualizar y reiniciar el juego?", "Version {v} ist da. Aktualisieren und das Spiel neu starten?", "La versione {v} è uscita. Aggiornare e riavviare il gioco?"),
    ("Version {v} is out. Open its web page?", "La version {v} est sortie. Ouvrir sa page web ?", "Ha salido la versión {v}. ¿Abrir su página web?", "Version {v} ist da. Ihre Webseite öffnen?", "La versione {v} è uscita. Aprire la sua pagina web?"),
    ("Progress since your last save will be lost.", "La progression depuis la dernière sauvegarde sera perdue.", "Se perderá el progreso desde tu última partida guardada.", "Fortschritt seit dem letzten Speichern geht verloren.", "I progressi dall'ultimo salvataggio andranno persi."),
    ("Updating", "Mise à jour", "Actualizando", "Aktualisierung", "Aggiornamento"),
    ("Downloading version {version}: {percent}%", "Téléchargement de la version {version} : {percent} %", "Descargando la versión {version}: {percent} %", "Lade Version {version} herunter: {percent} %", "Download della versione {version}: {percent}%"),
    ("Installing version {version}...", "Installation de la version {version}...", "Instalando la versión {version}...", "Installiere Version {version}...", "Installazione della versione {version}..."),
    ("Done. The game is restarting.", "Terminé. Le jeu redémarre.", "Listo. El juego se está reiniciando.", "Fertig. Das Spiel startet neu.", "Fatto. Il gioco si sta riavviando."),
    ("The update failed", "La mise à jour a échoué", "La actualización ha fallado", "Das Update ist fehlgeschlagen", "L'aggiornamento non è riuscito"),
    // Practice, and its keys.
    ("Practice", "Entraînement", "Práctica", "Training", "Pratica"),
    ("Game speed and practice tools. LAN games run at 100% and can't restart.", "Vitesse du jeu et outils d'entraînement. En réseau local, le jeu tourne à 100 % et ne redémarre pas.", "Velocidad del juego y herramientas de práctica. En red local el juego va al 100 % y no se reinicia.", "Spieltempo und Trainingshilfen. LAN-Spiele laufen mit 100 % und starten nicht neu.", "Velocità di gioco e strumenti di allenamento. In LAN il gioco va al 100% e non si riavvia."),
    ("Game Speed", "Vitesse du jeu", "Velocidad del juego", "Spieltempo", "Velocità di gioco"),
    ("Fast-Forward Speed", "Vitesse d'avance rapide", "Velocidad de avance rápido", "Vorspultempo", "Velocità avanti veloce"),
    ("Level Timer", "Chrono du niveau", "Cronómetro del nivel", "Level-Zeit", "Cronometro del livello"),
    ("Death Counter", "Compteur de vies perdues", "Contador de vidas perdidas", "Verlorene Leben", "Contatore di vite perse"),
    ("Input Display", "Affichage des boutons", "Mostrar botones", "Eingabeanzeige", "Mostra pulsanti"),
    ("Unlimited", "Illimitée", "Ilimitada", "Unbegrenzt", "Illimitata"),
    ("Player 1", "Joueur 1", "Jugador 1", "Spieler 1", "Giocatore 1"),
    ("All Players", "Tous les joueurs", "Todos los jugadores", "Alle Spieler", "Tutti i giocatori"),
    ("As fast as the PC runs the game, without sound.", "Aussi vite que le PC fait tourner le jeu, sans le son.", "Tan rápido como el PC mueva el juego, sin sonido.", "So schnell, wie der PC das Spiel schafft, ohne Ton.", "Alla massima velocità del PC, senza audio."),
    ("The game, and its sound at its own pitch, at this speed.", "Le jeu, et son son à sa propre hauteur, à cette vitesse.", "El juego, y su sonido con su tono propio, a esta velocidad.", "Das Spiel, und sein Ton in eigener Tonhöhe, in diesem Tempo.", "Il gioco, e il suo audio con la sua intonazione, a questa velocità."),
    ("The speed while Fast Forward is held (its key is in Keybinds).", "La vitesse tant qu'Avance rapide est maintenue (sa touche est dans Touches).", "La velocidad mientras se mantiene Avance rápido (su tecla está en Controles).", "Das Tempo, solange Vorspulen gehalten wird (die Taste steht unter Tasten).", "La velocità mentre si tiene Avanti veloce (il tasto è in Comandi)."),
    ("The course's time in game frames from each start; it stops at the goal.", "Le temps du niveau en images de jeu depuis chaque départ ; il s'arrête à l'arrivée.", "El tiempo del nivel en fotogramas de juego desde cada salida; se detiene en la meta.", "Die Zeit im Level in Spielbildern ab jedem Start; sie stoppt im Ziel.", "Il tempo del livello in fotogrammi di gioco da ogni partenza; si ferma al traguardo."),
    ("Lives lost in this course, and since the game started.", "Vies perdues dans ce niveau, et depuis le lancement du jeu.", "Vidas perdidas en este nivel, y desde que empezó el juego.", "Verlorene Leben in diesem Level und seit dem Spielstart.", "Vite perse in questo livello, e da quando è partito il gioco."),
    ("The buttons the game gets, at the bottom left.", "Les boutons que reçoit le jeu, en bas à gauche.", "Los botones que recibe el juego, abajo a la izquierda.", "Die Knöpfe, die das Spiel erhält, unten links.", "I pulsanti che riceve il gioco, in basso a sinistra."),
    ("Fast Forward", "Avance rapide", "Avance rápido", "Vorspulen", "Avanti veloce"),
    ("Restart Course", "Recommencer le niveau", "Reiniciar el nivel", "Level neu starten", "Riavvia livello"),
    ("Held: the game runs at the Practice tab's Fast-Forward Speed (not in LAN games).", "Maintenue : le jeu tourne à la Vitesse d'avance rapide de l'onglet Entraînement (pas en réseau local).", "Mantenido: el juego va a la Velocidad de avance rápido de la pestaña Práctica (no en red local).", "Gehalten: Das Spiel läuft mit dem Vorspultempo aus Training (nicht im LAN).", "Tenuto: il gioco va alla Velocità avanti veloce di Pratica (non in LAN)."),
    ("Starts the course again from its start (not in LAN games).", "Recommence le niveau depuis son début (pas en réseau local).", "Vuelve a empezar el nivel desde su inicio (no en red local).", "Startet den Level von vorn (nicht im LAN).", "Ricomincia il livello dall'inizio (non in LAN)."),
];

/// `english` in the menus' language (`fr`, `es`, `de`, `it`; anything else
/// is English), or as it is when the table has no line for it.
pub fn tr<'a>(language: &str, english: &'a str) -> &'a str {
    static MAP: OnceLock<HashMap<&'static str, [&'static str; 4]>> = OnceLock::new();
    let column = match language {
        "fr" => 0,
        "es" => 1,
        "de" => 2,
        "it" => 3,
        _ => return english,
    };
    match MAP.get_or_init(|| TABLE.iter().map(|&(en, fr, es, de, it)| (en, [fr, es, de, it])).collect()).get(english) {
        Some(row) => row[column],
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
        assert_eq!(tr("de", "Settings"), "Einstellungen");
        assert_eq!(tr("it", "Settings"), "Impostazioni");
        assert_eq!(tr("en", "Settings"), "Settings");
        assert_eq!(tr("fr", "A line nobody wrote"), "A line nobody wrote");
        // Each English line once, and every translation keeps its fields.
        let mut seen = std::collections::HashSet::new();
        for &(en, fr, es, de, it) in TABLE {
            assert!(seen.insert(en), "{en} twice");
            for field in ["{v}", "{name}", "{version}", "{percent}", "{addr}", "{dir}", "{p}", "{by}", "{n}"] {
                for line in [fr, es, de, it] {
                    assert_eq!(en.contains(field), line.contains(field), "{en} / {line}");
                }
            }
        }
    }
}
