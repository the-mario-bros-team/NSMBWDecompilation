package com.wither.nsmbwport;

import android.content.Context;
import android.util.Log;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

/**
 * The start menu's language, and the game's (the Wii's language in its SYSCONF, set at each start:
 * wiiLanguage). From NSMB-Wii-Recompilation's app, where it is the game's own Language setting. The USA disc's three languages, and
 * German and Italian, whose game text comes from a language pack (LauncherActivity.languagePack) or, on the
 * European disc, from the disc; English is the default. Text is looked up by its English wording.
 */
final class Lang {
    static final String[] CODES = {"en", "es", "fr", "de", "it"};
    /** Each language's name in itself, as the game's Language tab lists them. */
    static final String[] NAMES = {"English", "Español", "Français", "Deutsch", "Italiano"};

    private static int current;
    /** The imported disc is the European one, which has German and Italian text of its own. */
    private static boolean european;
    private static final Map<String, String[]> TABLE = new HashMap<>();

    private Lang() {}

    /** Reads the game's language; call before building any text. */
    static void load(Context context) {
        current = Math.max(0, Arrays.asList(CODES).indexOf(new AppSettings(context).language()));
        european = GameFiles.region(GameFiles.gameDir(context)) == 'P';
    }

    static int index() { return current; }

    /** Whether the disc has this language's text (the others need a language pack). */
    static boolean onDisc(int index) {
        return index < 3 || (european && (CODES[index].equals("de") || CODES[index].equals("it")));
    }

    /** Sets the game's language (it applies the next time the game starts) and the start menu's. */
    static void set(Context context, int index) {
        new AppSettings(context).setLanguage(CODES[index]);
        current = index;
    }

    /**
     * The Wii's language code (SYSCONF IPL.LNG) for the game: 1 English, 3 French, 4 Spanish, the
     * USA disc's three, and 2 German and 5 Italian on the European disc. Otherwise German and Italian
     * menus keep the game in English (its text would need a language pack).
     */
    static int wiiLanguage() {
        switch (CODES[current]) {
            case "fr": return 3;
            case "es": return 4;
            case "de": return european ? 2 : 1;
            case "it": return european ? 5 : 1;
            default: return 1;
        }
    }

    /** `english` in the start menu's language. */
    static String t(String english) {
        if (current == 0) return english;
        final String[] row = TABLE.get(english);
        if (row == null || row[current - 1] == null) {
            Log.w("NSMBW", "untranslated: " + english);
            return english;
        }
        return row[current - 1];
    }

    /** t(english), with `args` put in (String.format). */
    static String f(String english, Object... args) {
        return String.format(Locale.ROOT, t(english), args);
    }

    private static void put(String english, String spanish, String french) {
        final String[] row = row(english);
        row[0] = spanish;
        row[1] = french;
    }

    private static void de(String english, String german) {
        row(english)[2] = german;
    }

    private static void it(String english, String italian) {
        row(english)[3] = italian;
    }

    private static void newer(String english, String spanish, String french, String italian) {
        put(english, spanish, french);
        it(english, italian);
    }

    private static String[] row(String english) {
        return TABLE.computeIfAbsent(english, k -> new String[CODES.length - 1]);
    }

    static {
        // --- start screen ---
        put("Language", "Idioma", "Langue");
        put("Help", "Ayuda", "Aide");
        put("New Super Mario Bros. Wii, recompiled to run natively",
                "New Super Mario Bros. Wii, recompilado para ejecutarse de forma nativa",
                "New Super Mario Bros. Wii, recompilé pour fonctionner en natif");
        put("Play", "Jugar", "Jouer");
        put("Mods", "Mods", "Mods");
        put("Coming Soon!", "¡Próximamente!", "Bientôt disponible !");
        put("Updates", "Actualizaciones", "Mises à jour");
        put("Check for Updates", "Buscar actualizaciones", "Rechercher des mises à jour");
        put("Auto Update", "Actualización automática", "Mise à jour automatique");
        put("On: opening the app installs the latest release from GitHub when there is one. Your saves, "
                + "settings and game files are kept.",
                "Activado: al abrir la app se instala la última versión de GitHub, si hay una. Se conservan "
                + "tus partidas, ajustes y archivos del juego.",
                "Activé : à l'ouverture, l'appli installe la dernière version de GitHub s'il y en a une. Vos "
                + "sauvegardes, réglages et fichiers du jeu sont conservés.");
        put("Options", "Opciones", "Options");
        put("Settings", "Ajustes", "Réglages");
        put("Tailscale & LAN Devices", "Dispositivos Tailscale y LAN", "Appareils Tailscale et LAN");
        put("Skip Start Menu", "Omitir menú de inicio", "Passer le menu de démarrage");
        put("On: opening the app goes straight into the game. In the game, the phone's Back gesture and "
                + "Start Menu bring you back here.",
                "Activado: al abrir la app se entra directamente al juego. En el juego, el gesto Atrás del "
                + "teléfono y Start Menu te traen de vuelta aquí.",
                "Activé : l'appli lance directement le jeu. En jeu, le geste Retour du téléphone et Start "
                + "Menu vous ramènent ici.");
        put("Xiaomi Mode", "Modo Xiaomi", "Mode Xiaomi");
        put("On: the game runs in the app's main process. For Xiaomi, Redmi and POCO phones (on by default "
                + "there), whose system closes the game's own process and leaves a black screen.",
                "Activado: el juego se ejecuta en el proceso principal de la app. Para teléfonos Xiaomi, "
                + "Redmi y POCO (activado por defecto en ellos), cuyo sistema cierra el proceso del juego y "
                + "deja la pantalla negra.",
                "Activé : le jeu tourne dans le processus principal de l'appli. Pour les téléphones Xiaomi, "
                + "Redmi et POCO (activé par défaut sur ceux-ci), dont le système ferme le processus du jeu "
                + "et laisse un écran noir.");
        put("GPU Driver", "Controlador de GPU", "Pilote GPU");
        put("Run the game on a driver from a zip, such as Turnip, in place of the phone's own. If a new "
                + "driver stops the game from starting, Default comes back by itself.",
                "Ejecuta el juego con un controlador de un zip, como Turnip, en lugar del propio del teléfono. "
                + "Si uno nuevo impide que el juego arranque, se vuelve solo a Predeterminado.",
                "Lance le jeu avec un pilote fourni dans un zip, comme Turnip, à la place de celui du téléphone. "
                + "Si un nouveau pilote empêche le jeu de démarrer, Par défaut revient tout seul.");
        put("Only for phones with a Snapdragon chip.", "Solo para teléfonos con chip Snapdragon.",
                "Uniquement pour les téléphones à puce Snapdragon.");
        put("Default", "Predeterminado", "Par défaut");
        put("Import…", "Importar…", "Importer…");
        put("Delete…", "Eliminar…", "Supprimer…");
        put("Delete a GPU driver", "Eliminar un controlador de GPU", "Supprimer un pilote GPU");
        put("Importing the GPU driver", "Importando el controlador de GPU", "Importation du pilote GPU");
        put("%s is ready. The game uses it from its next start.",
                "%s está listo. El juego lo usará desde su próximo inicio.",
                "%s est prêt. Le jeu l'utilisera dès son prochain démarrage.");
        put("This zip isn't a GPU driver: it has no meta.json.",
                "Este zip no es un controlador de GPU: no tiene meta.json.",
                "Ce zip n'est pas un pilote GPU : il n'a pas de meta.json.");
        put("This zip isn't a GPU driver: its meta.json can't be read.",
                "Este zip no es un controlador de GPU: no se puede leer su meta.json.",
                "Ce zip n'est pas un pilote GPU : son meta.json est illisible.");
        put("This zip isn't a GPU driver: the library its meta.json names isn't in it.",
                "Este zip no es un controlador de GPU: no contiene la biblioteca que indica su meta.json.",
                "Ce zip n'est pas un pilote GPU : la bibliothèque indiquée par son meta.json n'y est pas.");
        put("This driver needs Android API %s or newer.",
                "Este controlador necesita la API %s de Android o posterior.",
                "Ce pilote nécessite l'API Android %s ou plus récente.");
        put("The game closed right after starting on the new GPU driver, so it's back on Default. You can "
                + "pick that driver again under GPU Driver.",
                "El juego se cerró justo después de iniciarse con el nuevo controlador de GPU, así que se ha "
                + "vuelto a Predeterminado. Puedes elegirlo de nuevo en Controlador de GPU.",
                "Le jeu s'est fermé juste après son démarrage avec le nouveau pilote GPU, il est donc revenu à "
                + "Par défaut. Vous pouvez le choisir à nouveau sous Pilote GPU.");
        put("On", "Sí", "Oui");
        put("Off", "No", "Non");
        put("Game Data", "Datos del juego", "Données du jeu");
        put("Get the game files (once)", "Consigue los archivos del juego (una vez)",
                "Obtenir les fichiers du jeu (une fois)");
        put("1. In Dolphin on a computer, right-click New Super Mario Bros. Wii and choose Properties.\n"
                + "2. Open Filesystem, right-click the disc at the top and choose Extract Entire Disc.\n"
                + "3. Copy that folder (or a zip of it) to this phone.\n"
                + "4. Tap Import from Extracted Game Data Folder… or Import Game Data Zip… below and "
                + "choose it.\n\n"
                + "Use the USA (SMNE01) or European (SMNP01) disc, revision 1: Dolphin shows the revision under "
                + "Properties > Info.",
                "1. En Dolphin, en una computadora, haz clic derecho en New Super Mario Bros. Wii y elige "
                + "Propiedades.\n"
                + "2. Abre Sistema de archivos, haz clic derecho en el disco de arriba y elige Extraer disco "
                + "completo.\n"
                + "3. Copia esa carpeta (o un zip de ella) a este teléfono.\n"
                + "4. Toca Importar desde la carpeta de datos extraídos… o Importar zip de datos del "
                + "juego… abajo y elígela.\n\n"
                + "Usa el disco de EE. UU. (SMNE01) o el europeo (SMNP01), revisión 1: Dolphin muestra la "
                + "revisión en Propiedades > Información.",
                "1. Dans Dolphin sur un ordinateur, faites un clic droit sur New Super Mario Bros. Wii et "
                + "choisissez Propriétés.\n"
                + "2. Ouvrez Système de fichiers, faites un clic droit sur le disque en haut et choisissez "
                + "Extraire le disque entier.\n"
                + "3. Copiez ce dossier (ou un zip de celui-ci) sur ce téléphone.\n"
                + "4. Touchez Importer depuis le dossier de données extrait… ou Importer un zip des "
                + "données du jeu… ci-dessous et choisissez-le.\n\n"
                + "Utilisez le disque américain (SMNE01) ou européen (SMNP01), révision 1 : Dolphin affiche la révision dans "
                + "Propriétés > Infos.");
        put("Import your own New Super Mario Bros. Wii (USA, SMNE01, or Europe, SMNP01) once. Easiest: the "
                + "folder Dolphin's "
                + "Extract Entire Disc makes (DATA, or the folder that holds it), or a zip of that folder. "
                + "Your saves are kept.",
                "Importa tu propio New Super Mario Bros. Wii (EE. UU., SMNE01, o Europa, SMNP01) una vez. Lo "
                + "más fácil: la "
                + "carpeta que crea Extraer disco completo de Dolphin (DATA, o la carpeta que la contiene), o "
                + "un zip de esa carpeta. Tus partidas se conservan.",
                "Importez une fois votre propre New Super Mario Bros. Wii (USA, SMNE01, ou Europe, SMNP01). Le "
                + "plus simple : le "
                + "dossier créé par Extraire le disque entier de Dolphin (DATA, ou le dossier qui le contient), "
                + "ou un zip de ce dossier. Vos sauvegardes sont conservées.");
        put("Import from Extracted Game Data Folder…", "Importar desde la carpeta de datos extraídos…",
                "Importer depuis le dossier de données extrait…");
        put("Import Game Data Zip…", "Importar zip de datos del juego…",
                "Importer un zip des données du jeu…");
        put("Import Disc Image (.iso)…", "Importar imagen de disco (.iso)…", "Importer une image disque (.iso)…");
        put("Import Disc Image (ISO, WBFS, RVZ)…", "Importar imagen de disco (ISO, WBFS, RVZ)…",
                "Importer une image disque (ISO, WBFS, RVZ)…");
        put("Or import your disc image straight away (.iso, .wbfs, .rvz or .wia): it is extracted once.",
                "O importa directamente tu imagen de disco (.iso, .wbfs, .rvz o .wia): se extrae una sola vez.",
                "Ou importez directement votre image disque (.iso, .wbfs, .rvz ou .wia) : elle est extraite une seule fois.");
        put("That isn't a Wii disc image this app can read (ISO, WBFS, RVZ or WIA).",
                "Eso no es una imagen de disco de Wii que esta app pueda leer (ISO, WBFS, RVZ o WIA).",
                "Ce n'est pas une image disque Wii que cette appli peut lire (ISO, WBFS, RVZ ou WIA).");
        put("Extracting the disc image…", "Extrayendo la imagen de disco…", "Extraction de l'image disque…");
        put("Have the disc image itself? Import the .iso: it's extracted on this phone, the way Dolphin does it, "
                + "with no computer needed.",
                "¿Tienes la imagen del disco? Importa el .iso: se extrae en este teléfono, como lo hace Dolphin, "
                + "sin necesidad de un ordenador.",
                "Vous avez l'image du disque ? Importez le .iso : il est extrait sur ce téléphone, comme le fait "
                + "Dolphin, sans ordinateur.");
        put("Extracting the disc image", "Extrayendo la imagen del disco", "Extraction de l'image disque");
        put("Reading the disc image…", "Leyendo la imagen del disco…", "Lecture de l'image disque…");
        put("That file isn't a Wii disc image. Pick the .iso itself; compressed images (.rvz, .wbfs, .wia) can be "
                + "turned into one with Dolphin's Convert File.",
                "Ese archivo no es una imagen de disco de Wii. Elige el propio .iso; las imágenes comprimidas "
                + "(.rvz, .wbfs, .wia) se pueden convertir en una con Convertir archivo de Dolphin.",
                "Ce fichier n'est pas une image disque Wii. Choisissez le .iso lui-même ; les images compressées "
                + "(.rvz, .wbfs, .wia) peuvent être converties avec Convertir le fichier de Dolphin.");
        put("There isn't enough free space on this phone: the game takes %d MB.",
                "No hay suficiente espacio libre en este teléfono: el juego ocupa %d MB.",
                "Il n'y a pas assez d'espace libre sur ce téléphone : le jeu occupe %d Mo.");
        put("that isn't a Wii disc image", "no es una imagen de disco de Wii", "ce n'est pas une image disque Wii");
        put("that disc image isn't encrypted the way a Wii disc is",
                "esa imagen de disco no está cifrada como un disco de Wii",
                "cette image disque n'est pas chiffrée comme un disque Wii");
        put("the disc image is cut short", "la imagen del disco está incompleta", "l'image disque est incomplète");
        put("that disc image isn't from a retail Wii disc", "esa imagen no es de un disco de Wii comercial",
                "cette image ne provient pas d'un disque Wii du commerce");
        put("that disc image has no game partition", "esa imagen de disco no tiene partición de juego",
                "cette image disque n'a pas de partition de jeu");
        put("the disc's file table is damaged", "la tabla de archivos del disco está dañada",
                "la table des fichiers du disque est endommagée");
        put("Download Logs (For Github Reports)", "Descargar registros (para reportes en GitHub)",
                "Télécharger les journaux (pour les rapports GitHub)");
        put("No game data yet. Import the folder Dolphin made, or a zip of it.",
                "Aún no hay datos del juego. Importa la carpeta que creó Dolphin, o un zip de ella.",
                "Pas encore de données du jeu. Importez le dossier créé par Dolphin, ou un zip de celui-ci.");
        put("The imported game can't be played. %s", "El juego importado no se puede jugar. %s",
                "Le jeu importé ne peut pas être lancé. %s");
        put("Game data is installed and checked.", "Los datos del juego están instalados y verificados.",
                "Les données du jeu sont installées et vérifiées.");
        put("Starting the LAN game...", "Iniciando la partida LAN...", "Lancement de la partie LAN...");
        put("OK", "Aceptar", "OK");
        put("Done", "Listo", "Terminé");
        put("Back", "Atrás", "Retour");
        put("Cancel", "Cancelar", "Annuler");

        // --- updates ---
        put("Checking for updates…", "Buscando actualizaciones…", "Recherche de mises à jour…");
        put("Couldn't check for updates: %s", "No se pudo buscar actualizaciones: %s",
                "Impossible de rechercher des mises à jour : %s");
        put("The latest release has no Android download.", "La última versión no tiene descarga para Android.",
                "La dernière version n'a pas de téléchargement Android.");
        put("You have the latest version (%s).", "Tienes la última versión (%s).",
                "Vous avez la dernière version (%s).");
        put("NSMBW %s is available", "NSMBW %s está disponible", "NSMBW %s est disponible");
        put("To install updates, let NSMBW install apps: turn on Allow from this source on the next screen, "
                + "then come back here.",
                "Para instalar actualizaciones, permite que NSMBW instale apps: activa Permitir desde esta "
                + "fuente en la siguiente pantalla y luego vuelve aquí.",
                "Pour installer les mises à jour, autorisez NSMBW à installer des applis : activez Autoriser "
                + "cette source sur l'écran suivant, puis revenez ici.");
        put("Open Settings", "Abrir ajustes", "Ouvrir les réglages");
        put("Not Now", "Ahora no", "Pas maintenant");
        put("Downloading…", "Descargando…", "Téléchargement…");
        put("Downloading… %d of %d MB", "Descargando… %d de %d MB", "Téléchargement… %d sur %d Mo");
        put("Updating to NSMBW %s", "Actualizando a NSMBW %s", "Mise à jour vers NSMBW %s");
        put("The update failed: %s", "La actualización falló: %s", "La mise à jour a échoué : %s");
        put("Installing NSMBW %s…", "Instalando NSMBW %s…", "Installation de NSMBW %s…");

        // --- help ---
        put("Getting Started", "Primeros pasos", "Premiers pas");
        put("1. Import New Super Mario Bros. Wii", "1. Importa New Super Mario Bros. Wii",
                "1. Importer New Super Mario Bros. Wii");
        put("Use your own New Super Mario Bros. Wii for the USA (SMNE01) or Europe (SMNP01), revision 1. "
                + "Easiest: in Dolphin on a "
                + "computer, right-click the game, choose Properties, Filesystem, then right-click the disc and "
                + "choose Extract Entire Disc. Copy that folder (or a zip of it) to this phone and tap Import. "
                + "Dolphin shows the revision under Properties > Info.",
                "Usa tu propio New Super Mario Bros. Wii de EE. UU. (SMNE01) o de Europa (SMNP01), revisión 1. "
                + "Lo más fácil: en "
                + "Dolphin, en una computadora, haz clic derecho en el juego, elige Propiedades, Sistema de "
                + "archivos, luego haz clic derecho en el disco y elige Extraer disco completo. Copia esa "
                + "carpeta (o un zip de ella) a este teléfono y toca Importar. Dolphin muestra la revisión en "
                + "Propiedades > Información.",
                "Utilisez votre propre New Super Mario Bros. Wii américain (SMNE01) ou européen (SMNP01), "
                + "révision 1. Le plus "
                + "simple : dans Dolphin sur un ordinateur, faites un clic droit sur le jeu, choisissez "
                + "Propriétés, Système de fichiers, puis faites un clic droit sur le disque et choisissez "
                + "Extraire le disque entier. Copiez ce dossier (ou un zip de celui-ci) sur ce téléphone et "
                + "touchez Importer. Dolphin affiche la révision dans Propriétés > Infos.");
        put("2. Play", "2. Juega", "2. Jouer");
        put("Tap Play. For LAN play, move up to the Couch button at the top right of the save file screen and "
                + "switch it to LAN; every player imports their own game.",
                "Toca Jugar. Para jugar en LAN, sube al botón Couch arriba a la derecha de la pantalla de "
                + "archivos y cámbialo a LAN; cada jugador importa su propio juego.",
                "Touchez Jouer. Pour jouer en LAN, montez jusqu'au bouton Couch en haut à droite de l'écran "
                + "des fichiers et passez-le sur LAN ; chaque joueur importe son propre jeu.");
        put("Stuck on a step?", "¿Atascado en un paso?", "Bloqué à une étape ?");
        put("The GitHub page covers the supported game, the steps above and common problems.",
                "La página de GitHub explica el juego compatible, los pasos anteriores y los problemas comunes.",
                "La page GitHub présente le jeu pris en charge, les étapes ci-dessus et les problèmes courants.");
        put("Setup Guide on GitHub", "Guía de instalación en GitHub", "Guide d'installation sur GitHub");

        // --- logs ---
        put("Saved, but there are no game logs yet: play once, then download them again.",
                "Guardado, pero aún no hay registros del juego: juega una vez y luego descárgalos de nuevo.",
                "Enregistré, mais il n'y a pas encore de journaux du jeu : jouez une fois, puis "
                + "téléchargez-les à nouveau.");
        put("Saved the logs of the last game session. Attach the zip to your GitHub issue.",
                "Se guardaron los registros de la última sesión de juego. Adjunta el zip a tu reporte en "
                + "GitHub.",
                "Les journaux de la dernière session de jeu sont enregistrés. Joignez le zip à votre "
                + "rapport GitHub.");
        put("Saved the logs of the last %d game sessions. Attach the zip to your GitHub issue.",
                "Se guardaron los registros de las últimas %d sesiones de juego. Adjunta el zip a tu reporte "
                + "en GitHub.",
                "Les journaux des %d dernières sessions de jeu sont enregistrés. Joignez le zip à votre "
                + "rapport GitHub.");
        put("Could not save the logs: %s", "No se pudieron guardar los registros: %s",
                "Impossible d'enregistrer les journaux : %s");

        // --- import ---
        put("Looking at the game files…", "Revisando los archivos del juego…",
                "Examen des fichiers du jeu…");
        put("Importing game data", "Importando datos del juego", "Importation des données du jeu");
        put("%s\n\nNothing was changed.", "%s\n\nNo se cambió nada.", "%s\n\nRien n'a été modifié.");
        put("Import failed: %s", "La importación falló: %s", "L'importation a échoué : %s");
        put("Imported %d files. Tap Play to start.", "Se importaron %d archivos. Toca Jugar para empezar.",
                "%d fichiers importés. Touchez Jouer pour commencer.");
        put("%d of %d files", "%d de %d archivos", "%d fichiers sur %d");
        put("%d of %d MB", "%d de %d MB", "%d sur %d Mo");
        put("%d files", "%d archivos", "%d fichiers");
        put("The folder doesn't include all required files.",
                "La carpeta no incluye todos los archivos necesarios.",
                "Le dossier ne contient pas tous les fichiers requis.");
        put("That folder's sys/boot.bin is too short, so the game can't be identified.",
                "El sys/boot.bin de esa carpeta es demasiado corto, así que no se puede identificar el juego.",
                "Le sys/boot.bin de ce dossier est trop court, le jeu ne peut donc pas être identifié.");
        put("Its sys/boot.bin is too short, so the game can't be identified.",
                "Su sys/boot.bin es demasiado corto, así que no se puede identificar el juego.",
                "Son sys/boot.bin est trop court, le jeu ne peut donc pas être identifié.");
        put("That is %s, not New Super Mario Bros. Wii for the USA (SMNE01) or Europe (SMNP01). This port "
                + "runs those discs, revision 1.",
                "Eso es %s, no New Super Mario Bros. Wii de EE. UU. (SMNE01) ni de Europa (SMNP01). Este port "
                + "funciona con esos discos, revisión 1.",
                "C'est %s, pas New Super Mario Bros. Wii américain (SMNE01) ni européen (SMNP01). Ce portage "
                + "fonctionne avec ces disques, révision 1.");
        put("That is New Super Mario Bros. Wii revision %d. This port runs only revision 1 (Dolphin shows it "
                + "under Properties > Info).",
                "Eso es New Super Mario Bros. Wii revisión %d. Este port solo funciona con la revisión 1 "
                + "(Dolphin la muestra en Propiedades > Información).",
                "C'est New Super Mario Bros. Wii révision %d. Ce portage ne fonctionne qu'avec la révision 1 "
                + "(Dolphin l'affiche dans Propriétés > Infos).");

        // --- Settings ---
        put("Display", "Pantalla", "Affichage");
        put("Controls", "Controles", "Commandes");
        put("Aspect Ratio", "Relación de aspecto", "Format d'image");
        put("Render Resolution", "Resolución de renderizado", "Résolution de rendu");
        put("Performance Monitor", "Monitor de rendimiento", "Moniteur de performances");
        put("Spinning Coins", "Monedas giratorias", "Pièces qui tournent");
        put("V-Sync", "V-Sync", "V-Sync");
        put("Graphics Compatibility", "Compatibilidad gráfica", "Compatibilité graphique");
        put("Renderer", "Renderizador", "Moteur de rendu");
        put("Automatic", "Automático", "Automatique");
        put("Automatic (recommended): Vulkan, else OpenGL ES", "Automático (recomendado): Vulkan, si no OpenGL ES",
                "Automatique (recommandé) : Vulkan, sinon OpenGL ES");
        put("OpenGL ES 3.1+ (in a later version)", "OpenGL ES 3.1+ (en una versión futura)",
                "OpenGL ES 3.1+ (dans une version future)");
        put("Auto", "Auto", "Auto");
        put("Auto (recommended): on for the GPUs that need it",
                "Auto (recomendado): activado para las GPU que lo necesitan",
                "Auto (recommandé) : activé pour les GPU qui en ont besoin");
        put("On: always use the GPU fixes", "Sí: usar siempre las correcciones de GPU",
                "Oui : toujours utiliser les correctifs GPU");
        put("Off: never use them", "No: no usarlas nunca", "Non : ne jamais les utiliser");
        put("Original 4:3", "4:3 original", "4:3 d'origine");
        put("Fill Screen", "Llenar pantalla", "Plein écran");
        put("Match screen", "Igual a la pantalla", "Comme l'écran");
        put("Touch Controls", "Controles táctiles", "Commandes tactiles");
        put("Touch Control Opacity", "Opacidad de controles táctiles", "Opacité des commandes tactiles");
        put("Tilt (turn the phone)", "Inclinación (gira el teléfono)", "Inclinaison (tournez le téléphone)");
        put("Shake (jolt the phone)", "Sacudida (agita el teléfono)", "Secousse (secouez le téléphone)");
        put("Vibration", "Vibración", "Vibration");

        // --- Tailscale & LAN Devices ---
        put("This phone", "Este teléfono", "Ce téléphone");
        put("No network", "Sin red", "Aucun réseau");
        put("Rooms on your Wi-Fi are found by themselves. Over Tailscale, PCs find every device, and devices "
                + "that have played together find each other. To find another phone the first time, add its "
                + "Tailscale name or address here (or it adds yours).",
                "Las salas de tu Wi-Fi se encuentran solas. Por Tailscale, las PC encuentran todos los "
                + "dispositivos, y los dispositivos que ya jugaron juntos se encuentran entre sí. Para "
                + "encontrar otro teléfono la primera vez, añade aquí su nombre o dirección de Tailscale (o "
                + "que él añada la tuya).",
                "Les salles de votre Wi-Fi sont trouvées toutes seules. Par Tailscale, les PC trouvent tous "
                + "les appareils, et les appareils qui ont déjà joué ensemble se retrouvent. Pour trouver un "
                + "autre téléphone la première fois, ajoutez ici son nom ou son adresse Tailscale (ou il "
                + "ajoute la vôtre).");
        put("Devices asked for rooms", "Dispositivos a los que se piden salas", "Appareils interrogés pour les salles");
        put("None yet", "Ninguno todavía", "Aucun pour l'instant");
        put("Stop asking %s for rooms?", "¿Dejar de pedir salas a %s?", "Ne plus demander de salles à %s ?");
        put("Remove", "Quitar", "Retirer");
        put("Add Device…", "Añadir dispositivo…", "Ajouter un appareil…");
        put("Add a Device", "Añadir un dispositivo", "Ajouter un appareil");
        put("Its Tailscale name (as in the Tailscale app) or address. It is asked for rooms from now on.",
                "Su nombre de Tailscale (como aparece en la app de Tailscale) o su dirección. A partir de ahora "
                + "se le pedirán salas.",
                "Son nom Tailscale (comme dans l'appli Tailscale) ou son adresse. On lui demandera des salles "
                + "à partir de maintenant.");
        put("Add", "Añadir", "Ajouter");
        put("e.g. s26-ultra or 100.84.140.32", "p. ej. s26-ultra o 100.84.140.32", "ex. s26-ultra ou 100.84.140.32");

        // --- in the game (GameActivity) ---
        put("Start Menu", "Menú de inicio", "Menu de démarrage");
        put("Leave LAN Game", "Salir de la partida LAN", "Quitter la partie LAN");
        put("Quit Game", "Salir del juego", "Quitter le jeu");
        put("Back to the Game", "Volver al juego", "Retour au jeu");
        put("Connect", "Conectar", "Se connecter");
        put("To install updates, let NSMBW install apps: turn on Allow from this source on the next screen, "
                + "then come back to the game.",
                "Para instalar actualizaciones, permite que NSMBW instale apps: activa Permitir desde esta "
                + "fuente en la siguiente pantalla y luego vuelve al juego.",
                "Pour installer les mises à jour, autorisez NSMBW à installer des applis : activez Autoriser "
                + "cette source sur l'écran suivant, puis revenez au jeu.");
        // --- Settings (SettingsActivity), worded as the game's settings window ---
        newer("Video", "Vídeo", "Vidéo", "Video");
        // --- Practice ---
        newer("Practice", "Práctica", "Entraînement", "Allenamento");
        newer("Game Speed", "Velocidad del juego", "Vitesse du jeu", "Velocità di gioco");
        newer("Level Timer", "Cronómetro del nivel", "Chrono du niveau", "Cronometro del livello");
        newer("Death Counter", "Contador de vidas perdidas", "Compteur de vies perdues", "Contatore di vite perse");
        newer("Input Display", "Mostrar botones", "Affichage des boutons", "Mostra pulsanti");
        newer("Unlimited", "Ilimitada", "Illimitée", "Illimitata");
        newer("Player 1", "Jugador 1", "Joueur 1", "Giocatore 1");
        newer("All Players", "Todos los jugadores", "Tous les joueurs", "Tutti i giocatori");
        newer("The game, and its sound at its own pitch, at this speed. LAN games run at 100%.",
                "El juego, y su sonido con su tono propio, a esta velocidad. En red local va al 100 %.",
                "Le jeu, et son son à sa propre hauteur, à cette vitesse. En réseau local, il tourne à 100 %.",
                "Il gioco, e il suo audio con la sua intonazione, a questa velocità. In LAN va al 100%.");
        newer("The course's time in game frames from each start; it stops at the goal.",
                "El tiempo del nivel en fotogramas de juego desde cada salida; se detiene en la meta.",
                "Le temps du niveau en images de jeu depuis chaque départ ; il s'arrête à l'arrivée.",
                "Il tempo del livello in fotogrammi di gioco da ogni partenza; si ferma al traguardo.");
        newer("Lives lost in this course, and since the game started.",
                "Vidas perdidas en este nivel, y desde que empezó el juego.",
                "Vies perdues dans ce niveau, et depuis le lancement du jeu.",
                "Vite perse in questo livello, e da quando è partito il gioco.");
        newer("The buttons the game gets, at the bottom left.",
                "Los botones que recibe el juego, abajo a la izquierda.",
                "Les boutons que reçoit le jeu, en bas à gauche.",
                "I pulsanti che riceve il gioco, in basso a sinistra.");
        newer("Game speed and practice tools, applied when you go back to the game.",
                "Velocidad del juego y herramientas de práctica; se aplican al volver al juego.",
                "Vitesse du jeu et outils d'entraînement, appliqués au retour dans le jeu.",
                "Velocità di gioco e strumenti di allenamento, applicati al ritorno nel gioco.");
        newer("Audio", "Audio", "Audio", "Audio");
        newer("Resolution", "Resolución", "Résolution", "Risoluzione");
        newer("Rendered at this many lines, then scaled to the screen. Match screen: the screen's own.",
                "Se renderiza con estas líneas y luego se escala a la pantalla. Igualar pantalla: la de la pantalla.",
                "Rendu avec ce nombre de lignes, puis mis à l'échelle de l'écran. Écran : celle de l'écran.",
                "Renderizzato con queste linee, poi scalato sullo schermo. Come lo schermo: quella dello schermo.");
        newer("Fill Screen: a wider view filling the whole screen. 16:9 and 4:3 add black bars.",
                "Llenar pantalla: una vista más ancha que llena toda la pantalla. 16:9 y 4:3 añaden barras negras.",
                "Plein écran : une vue plus large qui remplit tout l'écran. 16:9 et 4:3 ajoutent des bandes noires.",
                "Riempi schermo: una visuale più ampia che riempie lo schermo. 16:9 e 4:3 aggiungono bande nere.");
        newer("Automatic: Vulkan, else OpenGL ES. Takes effect at the next start.",
                "Automático: Vulkan, si no OpenGL ES. Se aplica en el próximo inicio.",
                "Automatique : Vulkan, sinon OpenGL ES. Prend effet au prochain démarrage.",
                "Automatico: Vulkan, altrimenti OpenGL ES. Vale dal prossimo avvio.");
        newer("No tearing: frames wait for the display, the game keeps its pace.",
                "Sin cortes: los fotogramas esperan a la pantalla, el juego mantiene su ritmo.",
                "Pas de déchirure : les images attendent l'écran, le jeu garde son rythme.",
                "Niente tearing: i fotogrammi aspettano lo schermo, il gioco mantiene il ritmo.");
        newer("FPS and each frame's time, top right. The slider sets its size.",
                "FPS y el tiempo de cada fotograma, arriba a la derecha. El control deslizante fija su tamaño.",
                "FPS et le temps de chaque image, en haut à droite. Le curseur règle sa taille.",
                "FPS e il tempo di ogni fotogramma, in alto a destra. Il cursore ne regola la dimensione.");
        newer("Monitor Size", "Tamaño del monitor", "Taille du moniteur", "Dimensione del monitor");
        newer("Animated coins and blocks. Off saves a little GPU work.",
                "Monedas y bloques animados. Desactivado ahorra algo de trabajo a la GPU.",
                "Pièces et blocs animés. Désactivé épargne un peu le GPU.",
                "Monete e blocchi animati. Disattivato risparmia un po' di lavoro alla GPU.");
        newer("Fixes for phone GPUs that scramble the game's models. Auto: on for the GPUs that need them.",
                "Arreglos para GPU de teléfonos que deforman los modelos. Auto: activados en las GPU que los necesitan.",
                "Correctifs pour les GPU de téléphone qui déforment les modèles. Auto : activés pour les GPU qui en "
                + "ont besoin.",
                "Correzioni per le GPU dei telefoni che rovinano i modelli. Auto: attive sulle GPU che ne hanno bisogno.");
        newer("Master", "General", "Général", "Generale");
        newer("Music", "Música", "Musique", "Musica");
        newer("Sound Effects", "Efectos de sonido", "Effets sonores", "Effetti sonori");
        newer("The volume of everything.", "El volumen de todo.", "Le volume de tout.", "Il volume di tutto.");
        newer("The background music.", "La música de fondo.", "La musique de fond.", "La musica di sottofondo.");
        newer("Jumps, coins, enemies and the rest.", "Saltos, monedas, enemigos y lo demás.",
                "Sauts, pièces, ennemis et le reste.", "Salti, monete, nemici e il resto.");
        newer("Menu Sounds", "Sonidos de menú", "Sons des menus", "Suoni dei menu");
        newer("The menus' cursor, select and back sounds.", "El cursor, elegir y volver de los menús.",
                "Le curseur, la sélection et le retour des menus.", "Cursore, scelta e indietro dei menu.");
        newer("Depth Blur", "Desenfoque", "Flou de profondeur", "Sfocatura");
        newer("The blur and heat haze of desert, beach, water and lava areas.",
                "El desenfoque y la calima del desierto, la playa, el agua y la lava.",
                "Le flou et les ondulations du désert, de la plage, de l'eau et de la lave.",
                "Sfocatura e foschia di deserto, spiaggia, acqua e lava.");
        newer("The on-screen Wii Remote. A controller hides it while you use it.",
                "El Wii Remote en pantalla. Un mando lo oculta mientras lo usas.",
                "La Wiimote à l'écran. Une manette la masque pendant que vous l'utilisez.",
                "Il Wii Remote sullo schermo. Un controller lo nasconde mentre lo usi.");
        newer("How solid the on-screen remote looks.", "Lo opaco que se ve el mando en pantalla.",
                "L'opacité de la manette à l'écran.", "Quanto appare solido il telecomando sullo schermo.");
        newer("Turning the phone tilts the Wii Remote, for the levels and platforms that use it.",
                "Girar el teléfono inclina el Wii Remote, para los niveles y plataformas que lo usan.",
                "Tourner le téléphone incline la Wiimote, pour les niveaux et plateformes qui l'utilisent.",
                "Ruotare il telefono inclina il Wii Remote, per i livelli e le piattaforme che lo usano.");
        newer("A jolt of the phone shakes the Wii Remote: spin jumps and picking things up.",
                "Una sacudida del teléfono agita el Wii Remote: saltos giratorios y recoger cosas.",
                "Une secousse du téléphone secoue la Wiimote : sauts tournoyants et ramasser des objets.",
                "Uno scossone del telefono agita il Wii Remote: salti rotanti e raccogliere oggetti.");
        newer("A short buzz when a touch button is pressed.", "Una breve vibración al pulsar un botón táctil.",
                "Une brève vibration quand un bouton tactile est pressé.",
                "Una breve vibrazione quando premi un pulsante touch.");
        newer("Restart the game to play in this language.", "Reinicia el juego para jugar en este idioma.",
                "Redémarrez le jeu pour jouer dans cette langue.", "Riavvia il gioco per giocare in questa lingua.");
        newer("The game's own text needs a language pack: the start menu's language button imports one.",
                "El texto del juego necesita un paquete de idioma: el botón de idioma del menú de inicio lo importa.",
                "Le texte du jeu a besoin d'un pack de langue : le bouton de langue du menu de départ en importe un.",
                "Il testo del gioco ha bisogno di un pacchetto lingua: il pulsante della lingua nel menu iniziale "
                + "ne importa uno.");
        // --- The game's code modules (AppFiles.moduleProblem) ---
        newer("It has no files/rels/%s.rel.LZ: this port needs the whole disc, as Dolphin's Extract Entire Disc makes it.",
                "No tiene files/rels/%s.rel.LZ: este port necesita el disco completo, como lo hace Extraer disco "
                + "completo de Dolphin.",
                "Il n'a pas files/rels/%s.rel.LZ : ce port a besoin du disque entier, comme le fait Extraire "
                + "tout le disque de Dolphin.",
                "Manca files/rels/%s.rel.LZ: questo port ha bisogno del disco intero, come lo crea Estrai disco "
                + "intero di Dolphin.");
        newer("Its files/rels/%s is changed: it isn't the original disc (a modded or repacked image?). This port runs only the unchanged disc.",
                "Su files/rels/%s está modificado: no es el disco original (¿una imagen con mods o reempaquetada?). "
                + "Este port solo funciona con el disco sin cambios.",
                "Son files/rels/%s est modifié : ce n'est pas le disque d'origine (une image moddée ou "
                + "reconditionnée ?). Ce port ne fonctionne qu'avec le disque d'origine.",
                "Il suo files/rels/%s è modificato: non è il disco originale (un'immagine modificata o "
                + "ricompattata?). Questo port funziona solo con il disco originale.");
        // --- Settings: the Gameplay tab ---
        newer("Gameplay", "Juego", "Jeu", "Partita");
        newer("Power-Up Freeze", "Pausa al transformarse", "Pause des transformations", "Pausa per le trasformazioni");
        newer("Everything stops for a moment while a player grows, shrinks or dies. Off: the level keeps moving.",
                "Todo se detiene un momento cuando un jugador crece, encoge o pierde una vida. Desactivado: el nivel sigue.",
                "Tout s'arrête un instant quand un joueur grandit, rapetisse ou perd une vie. Désactivé : le niveau continue.",
                "Tutto si ferma un momento quando un giocatore cresce, si rimpicciolisce o perde una vita. Disattivato: il livello continua.");
        newer("Player Collision", "Choques entre jugadores", "Collisions entre joueurs", "Urti tra giocatori");
        newer("Players bump into and stand on each other. Off: they pass through each other.",
                "Los jugadores chocan y se suben unos encima de otros. Desactivado: se atraviesan.",
                "Les joueurs se heurtent et se tiennent les uns sur les autres. Désactivé : ils se traversent.",
                "I giocatori si urtano e salgono uno sull'altro. Disattivato: si attraversano.");
        newer("Online games keep the game's own rules. Takes effect at the next start.",
                "Las partidas en línea mantienen las reglas del juego. Se aplica en el próximo inicio.",
                "Les parties en ligne gardent les règles du jeu. Prend effet au prochain démarrage.",
                "Le partite online mantengono le regole del gioco. Ha effetto al prossimo avvio.");
        newer("No Time Limit", "Sin límite de tiempo", "Sans limite de temps", "Tempo infinito");
        newer("Infinite Lives", "Vidas infinitas", "Vies infinies", "Vite infinite");
        newer("Cancel", "Cancelar", "Annuler", "Annulla");
        newer("Add", "Añadir", "Ajouter", "Aggiungi");
        newer("Delete", "Eliminar", "Supprimer", "Elimina");
        newer("Profiles", "Perfiles", "Profils", "Profili");
        newer("Save as a new profile", "Guardar como perfil nuevo", "Enregistrer comme nouveau profil", "Salva come nuovo profilo");
        newer("Every button is on the screen already.", "Todos los botones ya están en pantalla.",
                "Tous les boutons sont déjà à l'écran.", "Tutti i pulsanti sono già sullo schermo.");
        newer("D-Pad", "Cruceta", "Croix directionnelle", "Croce direzionale");
        newer("Shake", "Sacudir", "Secouer", "Scuoti");
        newer("Are you sure?", "¿Seguro?", "Êtes-vous sûr ?", "Sei sicuro?");
        newer("This will mark your save as cheated.", "Esto marcará tu partida como trucada.",
                "Votre sauvegarde sera marquée comme trichée.", "Il salvataggio verrà segnato come truccato.");
        newer("Yes", "Sí", "Oui", "Sì");
        newer("No", "No", "Non", "No");
        newer("Power-Up", "Potenciador", "Bonus", "Potenziamento");
        newer("What the Cheat button gives you.", "Lo que te da el botón de truco.",
                "Ce que le bouton de triche vous donne.", "Ciò che ti dà il pulsante trucco.");
        newer("Cheat Button", "Botón de truco", "Bouton de triche", "Pulsante trucco");
        newer("An on-screen button that gives you the Power-Up. Edit Touch Controls moves it.",
                "Un botón en pantalla que te da el potenciador. Se mueve en Editar controles táctiles.",
                "Un bouton à l'écran qui vous donne le bonus. Modifier les commandes tactiles le déplace.",
                "Un pulsante su schermo che ti dà il potenziamento. Si sposta da Modifica controlli touch.");
        newer("Small", "Pequeño", "Petit", "Piccolo");
        newer("Super Mushroom", "Superchampiñón", "Super champignon", "Super fungo");
        newer("Fire Flower", "Flor de fuego", "Fleur de feu", "Fiore di fuoco");
        newer("Mini Mushroom", "Minichampiñón", "Mini champignon", "Minifungo");
        newer("Propeller Mushroom", "Champiñón hélice", "Champignon hélice", "Fungo elica");
        newer("Penguin Suit", "Traje de pingüino", "Costume de pingouin", "Costume da pinguino");
        newer("Ice Flower", "Flor de hielo", "Fleur de glace", "Fiore di ghiaccio");
        newer("Star", "Estrella", "Étoile", "Stella");
        newer("Hammer Suit", "Traje de martillo", "Costume de marteau", "Tuta martello");
        newer("The course timer stands still: take your time.",
                "El tiempo del nivel se detiene: tómate tu tiempo.",
                "Le chrono du niveau s'arrête : prenez votre temps.",
                "Il tempo del livello si ferma: fai con calma.");
        newer("A miss costs no life.", "Perder no cuesta vidas.", "Perdre ne coûte aucune vie.",
                "Perdere non costa vite.");
        newer("These keep the game's own rules in LAN play.",
                "En LAN se juega con las reglas normales del juego.",
                "En LAN, ce sont les règles normales du jeu.",
                "In LAN valgono le regole normali del gioco.");
        // --- LAN Play (LanPlay) ---
        newer("LAN Play", "Juego en LAN", "Jeu en LAN", "Gioco in LAN");
        newer("Play with other phones and PCs on your Wi-Fi, or over Tailscale. Every player needs their own "
                + "game and the same version of NSMBW.",
                "Juega con otros teléfonos y PC de tu Wi-Fi, o por Tailscale. Cada jugador necesita su propio "
                + "juego y la misma versión de NSMBW.",
                "Jouez avec d'autres téléphones et PC de votre Wi-Fi, ou par Tailscale. Chaque joueur a besoin "
                + "de son propre jeu et de la même version de NSMBW.",
                "Gioca con altri telefoni e PC sulla tua rete Wi-Fi, o con Tailscale. Ogni giocatore ha bisogno "
                + "del proprio gioco e della stessa versione di NSMBW.");
        newer("Create Room", "Crear sala", "Créer une salle", "Crea stanza");
        newer("Rooms", "Salas", "Salles", "Stanze");
        newer("Looking for rooms…", "Buscando salas…", "Recherche de salles…", "Ricerca delle stanze…");
        newer("Join by Address…", "Unirse por dirección…", "Rejoindre par adresse…", "Entra tramite indirizzo…");
        newer("Join a Room", "Unirse a una sala", "Rejoindre une salle", "Entra in una stanza");
        newer("The host's address, as its LAN Play shows it, or its Tailscale name.",
                "La dirección del anfitrión, como la muestra su Juego en LAN, o su nombre de Tailscale.",
                "L'adresse de l'hôte, telle que son Jeu en LAN l'affiche, ou son nom Tailscale.",
                "L'indirizzo di chi ospita, come lo mostra il suo Gioco in LAN, o il suo nome Tailscale.");
        newer("Join", "Unirse", "Rejoindre", "Entra");
        newer("Can't look for rooms: %s", "No se pueden buscar salas: %s", "Impossible de chercher des salles : %s",
                "Impossibile cercare le stanze: %s");
        // --- Edit Touch Controls (TouchLayoutEditor) ---
        newer("Edit Touch Controls", "Editar controles táctiles", "Modifier les commandes tactiles",
                "Modifica i controlli touch");
        newer("Drag a control to move it. Pinch it, or tap it and use Bigger or Smaller, to resize it.",
                "Arrastra un control para moverlo. Pellízcalo, o tócalo y usa Más grande o Más pequeño, para "
                + "cambiar su tamaño.",
                "Faites glisser une commande pour la déplacer. Pincez-la, ou touchez-la puis Plus grand ou Plus "
                + "petit, pour la redimensionner.",
                "Trascina un controllo per spostarlo. Pizzicalo, o toccalo e usa Più grande o Più piccolo, per "
                + "ridimensionarlo.");
        newer("Smaller", "Más pequeño", "Plus petit", "Più piccolo");
        newer("Bigger", "Más grande", "Plus grand", "Più grande");
        newer("Reset", "Restablecer", "Réinitialiser", "Ripristina");
        // --- Newer Super Mario Bros. Wii (its tab, its module, its game data) ---
        newer("Newer Super Mario Bros. Wii, recompiled to run natively",
                "Newer Super Mario Bros. Wii, recompilado para ejecutarse de forma nativa",
                "Newer Super Mario Bros. Wii, recompilé pour fonctionner en natif",
                "Newer Super Mario Bros. Wii, ricompilato per funzionare in modo nativo");
        newer("Newer Game Data", "Datos del juego de Newer", "Données du jeu Newer", "Dati del gioco di Newer");
        newer("1. Have your own Newer Super Mario Bros. Wii v1.30 for the USA (SMNE03) as a disc image (.iso).\n"
                + "2. Copy it to this phone and tap Import Disc Image (.iso)… below: it's extracted here.\n\n"
                + "Or extract it in Dolphin (Properties, Filesystem, Extract Entire Disc) and import that folder or a zip of it.",
                "1. Ten tu propio Newer Super Mario Bros. Wii v1.30 de EE. UU. (SMNE03) como imagen de disco (.iso).\n"
                + "2. Cópiala a este teléfono y toca Importar imagen de disco (.iso)… abajo: se extrae aquí.\n\n"
                + "O extráela en Dolphin (Propiedades, Sistema de archivos, Extraer disco entero) e importa esa "
                + "carpeta o un zip de ella.",
                "1. Ayez votre propre Newer Super Mario Bros. Wii v1.30 américain (SMNE03) en image disque (.iso).\n"
                + "2. Copiez-la sur ce téléphone et touchez Importer une image disque (.iso)… ci-dessous : elle est "
                + "extraite ici.\n\n"
                + "Ou extrayez-la dans Dolphin (Propriétés, Système de fichiers, Extraire le disque entier) et "
                + "importez ce dossier ou un zip de celui-ci.",
                "1. Procurati il tuo Newer Super Mario Bros. Wii v1.30 USA (SMNE03) come immagine disco (.iso).\n"
                + "2. Copiala su questo telefono e tocca Importa immagine disco (.iso)… qui sotto: viene estratta qui.\n\n"
                + "Oppure estraila in Dolphin (Proprietà, File system, Estrai intero disco) e importa quella "
                + "cartella o un suo zip.");
        newer("Import your own Newer Super Mario Bros. Wii v1.30 (USA, SMNE03) once: its disc image, the folder "
                + "Dolphin's Extract Entire Disc makes from it, or a zip of that folder. Newer keeps saves of its own.",
                "Importa una vez tu propio Newer Super Mario Bros. Wii v1.30 (EE. UU., SMNE03): su imagen de disco, "
                + "la carpeta que crea Extraer disco entero de Dolphin o un zip de esa carpeta. Newer tiene sus "
                + "propias partidas guardadas.",
                "Importez une fois votre propre Newer Super Mario Bros. Wii v1.30 (USA, SMNE03) : son image disque, "
                + "le dossier créé par Extraire le disque entier de Dolphin, ou un zip de ce dossier. Newer a ses "
                + "propres sauvegardes.",
                "Importa una volta il tuo Newer Super Mario Bros. Wii v1.30 (USA, SMNE03): la sua immagine disco, "
                + "la cartella creata da Estrai intero disco di Dolphin, o uno zip di quella cartella. Newer ha "
                + "salvataggi suoi.");
        newer("Remove the NewerSMBW Support Module", "Quitar el módulo de soporte de NewerSMBW",
                "Retirer le module de prise en charge NewerSMBW", "Rimuovi il modulo di supporto NewerSMBW");
        newer("Remove the NewerSMBW support module? Newer's game files and saves stay; download the module again "
                + "to play it.",
                "¿Quitar el módulo de soporte de NewerSMBW? Los archivos y partidas de Newer se conservan; vuelve a "
                + "descargar el módulo para jugarlo.",
                "Retirer le module de prise en charge NewerSMBW ? Les fichiers et sauvegardes de Newer restent ; "
                + "téléchargez à nouveau le module pour y jouer.",
                "Rimuovere il modulo di supporto NewerSMBW? I file e i salvataggi di Newer restano; scarica di nuovo "
                + "il modulo per giocarci.");
        newer("Download the NewerSMBW support module!", "¡Descarga el módulo de soporte de NewerSMBW!",
                "Téléchargez le module de prise en charge NewerSMBW !", "Scarica il modulo di supporto NewerSMBW!");
        newer("Downloading the NewerSMBW support module", "Descargando el módulo de soporte de NewerSMBW",
                "Téléchargement du module de prise en charge NewerSMBW", "Download del modulo di supporto NewerSMBW");
        newer("The NewerSMBW support module is installed. Import your Newer Super Mario Bros. Wii, then tap Play.",
                "El módulo de soporte de NewerSMBW está instalado. Importa tu Newer Super Mario Bros. Wii y toca Jugar.",
                "Le module de prise en charge NewerSMBW est installé. Importez votre Newer Super Mario Bros. Wii, "
                + "puis touchez Jouer.",
                "Il modulo di supporto NewerSMBW è installato. Importa il tuo Newer Super Mario Bros. Wii, poi tocca "
                + "Gioca.");
        newer("This build of the app comes without the NewerSMBW support module.",
                "Esta compilación de la app no incluye el módulo de soporte de NewerSMBW.",
                "Cette version de l'appli est fournie sans le module de prise en charge NewerSMBW.",
                "Questa build dell'app non ha il modulo di supporto NewerSMBW.");
        newer("Newer Super Mario Bros. Wii runs on a build of the game of its own: the support module, a separate "
                + "download from this version's GitHub release, so the app stays small for everyone else.",
                "Newer Super Mario Bros. Wii funciona con una compilación propia del juego: el módulo de soporte, "
                + "una descarga aparte de la versión de GitHub de esta app, para que la app siga siendo pequeña "
                + "para todos los demás.",
                "Newer Super Mario Bros. Wii fonctionne avec sa propre version du jeu : le module de prise en "
                + "charge, un téléchargement séparé depuis la version GitHub de cette appli, pour que l'appli reste "
                + "légère pour tous les autres.",
                "Newer Super Mario Bros. Wii gira su una build del gioco tutta sua: il modulo di supporto, un "
                + "download a parte dalla release GitHub di questa versione, così l'app resta leggera per tutti "
                + "gli altri.");
        newer("Newer Super Mario Bros. Wii needs its support module: download it below.",
                "Newer Super Mario Bros. Wii necesita su módulo de soporte: descárgalo abajo.",
                "Newer Super Mario Bros. Wii a besoin de son module de prise en charge : téléchargez-le ci-dessous.",
                "Newer Super Mario Bros. Wii ha bisogno del suo modulo di supporto: scaricalo qui sotto.");
        newer("No Newer game data yet. Import its disc image, the folder Dolphin made, or a zip of it.",
                "Aún no hay datos de Newer. Importa su imagen de disco, la carpeta que creó Dolphin o un zip de ella.",
                "Pas encore de données de Newer. Importez son image disque, le dossier créé par Dolphin, ou un zip "
                + "de celui-ci.",
                "Ancora nessun dato di Newer. Importa la sua immagine disco, la cartella creata da Dolphin o un "
                + "suo zip.");
        newer("That is another version of Newer Super Mario Bros. Wii. This port runs v1.30 for the USA.",
                "Esa es otra versión de Newer Super Mario Bros. Wii. Este port ejecuta la v1.30 de EE. UU.",
                "C'est une autre version de Newer Super Mario Bros. Wii. Ce portage lance la v1.30 américaine.",
                "Questa è un'altra versione di Newer Super Mario Bros. Wii. Questo port esegue la v1.30 USA.");
        newer("That is %s, not Newer Super Mario Bros. Wii for the USA (SMNE03).",
                "Eso es %s, no Newer Super Mario Bros. Wii de EE. UU. (SMNE03).",
                "C'est %s, pas Newer Super Mario Bros. Wii américain (SMNE03).",
                "Quello è %s, non Newer Super Mario Bros. Wii USA (SMNE03).");
        newer("this version of the app has no Newer module", "esta versión de la app no tiene módulo de Newer",
                "cette version de l'appli n'a pas de module Newer", "questa versione dell'app non ha il modulo Newer");
        newer("GitHub answered %d", "GitHub respondió %d", "GitHub a répondu %d", "GitHub ha risposto %d");
        newer("the download doesn't match this version's module",
                "la descarga no coincide con el módulo de esta versión",
                "le téléchargement ne correspond pas au module de cette version",
                "il download non corrisponde al modulo di questa versione");
        // --- this port's Android extras ---
        newer("Continuing where you left off.", "Continuando donde lo dejaste.", "Reprise là où vous en étiez.",
                "Riprendi da dove avevi lasciato.");
        newer("Your saves, settings and game files are kept.",
                "Se conservan tus partidas, ajustes y archivos del juego.",
                "Vos sauvegardes, réglages et fichiers du jeu sont conservés.",
                "Salvataggi, impostazioni e file del gioco restano.");
        newer("Update", "Actualizar", "Mettre à jour", "Aggiorna");
        newer("Stand-ins for GPUs without dual-source blending or logic ops, such as most Mali GPUs. Auto: on "
                + "for the GPUs that need them. Takes effect at the next start.",
                "Sustitutos para GPU sin mezcla de doble fuente ni operaciones lógicas, como la mayoría de las "
                + "Mali. Auto: activados en las GPU que los necesitan. Se aplica en el próximo inicio.",
                "Des remplaçants pour les GPU sans mélange à double source ni opérations logiques, comme la "
                + "plupart des Mali. Auto : activés pour les GPU qui en ont besoin. Prend effet au prochain "
                + "démarrage.",
                "Sostituti per le GPU senza blending a doppia sorgente né operazioni logiche, come la maggior "
                + "parte delle Mali. Auto: attivi sulle GPU che ne hanno bisogno. Vale dal prossimo avvio.");
        // --- Italian, by the same English wording ---
        it("Language", "Lingua");
        it("Help", "Aiuto");
        it("New Super Mario Bros. Wii, recompiled to run natively",
                "New Super Mario Bros. Wii, ricompilato per funzionare in modo nativo");
        it("Play", "Gioca");
        it("Mods", "Mod");
        it("Coming Soon!", "Prossimamente!");
        it("Updates", "Aggiornamenti");
        it("Check for Updates", "Cerca aggiornamenti");
        it("Auto Update", "Aggiornamento automatico");
        it("On: opening the app installs the latest release from GitHub when there is one. Your saves, "
                + "settings and game files are kept.",
                "Sì: all'apertura l'app installa l'ultima versione da GitHub, se ce n'è una. Salvataggi, "
                + "impostazioni e file del gioco vengono mantenuti.");
        it("Options", "Opzioni");
        it("Settings", "Impostazioni");
        it("Tailscale & LAN Devices", "Dispositivi Tailscale e LAN");
        it("Skip Start Menu", "Salta il menu iniziale");
        it("On: opening the app goes straight into the game. In the game, the phone's Back gesture and "
                + "Start Menu bring you back here.",
                "Sì: all'apertura l'app entra subito nel gioco. Nel gioco, il gesto Indietro del telefono e "
                + "Start Menu ti riportano qui.");
        it("Xiaomi Mode", "Modalità Xiaomi");
        it("On: the game runs in the app's main process. For Xiaomi, Redmi and POCO phones (on by default "
                + "there), whose system closes the game's own process and leaves a black screen.",
                "Sì: il gioco gira nel processo principale dell'app. Per i telefoni Xiaomi, Redmi e POCO "
                + "(attiva di serie su di essi), il cui sistema chiude il processo del gioco e lascia lo "
                + "schermo nero.");
        it("GPU Driver", "Driver GPU");
        it("Run the game on a driver from a zip, such as Turnip, in place of the phone's own. If a new "
                + "driver stops the game from starting, Default comes back by itself.",
                "Esegue il gioco con un driver da uno zip, come Turnip, al posto di quello del telefono. Se un "
                + "nuovo driver impedisce l'avvio del gioco, Predefinito torna da solo.");
        it("Only for phones with a Snapdragon chip.", "Solo per i telefoni con chip Snapdragon.");
        it("Default", "Predefinito");
        it("Import…", "Importa…");
        it("Delete…", "Elimina…");
        it("Delete a GPU driver", "Elimina un driver GPU");
        it("Importing the GPU driver", "Importazione del driver GPU");
        it("%s is ready. The game uses it from its next start.",
                "%s è pronto. Il gioco lo userà dal prossimo avvio.");
        it("This zip isn't a GPU driver: it has no meta.json.",
                "Questo zip non è un driver GPU: non ha un meta.json.");
        it("This zip isn't a GPU driver: its meta.json can't be read.",
                "Questo zip non è un driver GPU: il suo meta.json non si può leggere.");
        it("This zip isn't a GPU driver: the library its meta.json names isn't in it.",
                "Questo zip non è un driver GPU: non contiene la libreria indicata dal suo meta.json.");
        it("This driver needs Android API %s or newer.",
                "Questo driver richiede l'API Android %s o successiva.");
        it("The game closed right after starting on the new GPU driver, so it's back on Default. You can "
                + "pick that driver again under GPU Driver.",
                "Il gioco si è chiuso subito dopo l'avvio con il nuovo driver GPU, quindi è tornato a "
                + "Predefinito. Puoi sceglierlo di nuovo in Driver GPU.");
        it("On", "Sì");
        it("Off", "No");
        it("Game Data", "Dati del gioco");
        it("Get the game files (once)", "Ottieni i file del gioco (una volta)");
        it("1. In Dolphin on a computer, right-click New Super Mario Bros. Wii and choose Properties.\n"
                + "2. Open Filesystem, right-click the disc at the top and choose Extract Entire Disc.\n"
                + "3. Copy that folder (or a zip of it) to this phone.\n"
                + "4. Tap Import from Extracted Game Data Folder… or Import Game Data Zip… below and "
                + "choose it.\n\n"
                + "Use the USA (SMNE01) or European (SMNP01) disc, revision 1: Dolphin shows the revision under "
                + "Properties > Info.",
                "1. In Dolphin su un computer, fai clic destro su New Super Mario Bros. Wii e scegli "
                + "Proprietà.\n"
                + "2. Apri Filesystem, fai clic destro sul disco in alto e scegli Estrai intero disco.\n"
                + "3. Copia quella cartella (o un suo zip) su questo telefono.\n"
                + "4. Tocca Importa dalla cartella dei dati estratti… o Importa zip dei dati del gioco… qui "
                + "sotto e sceglila.\n\n"
                + "Usa il disco USA (SMNE01) o europeo (SMNP01), revisione 1: Dolphin mostra la revisione in "
                + "Proprietà > Info.");
        it("Import your own New Super Mario Bros. Wii (USA, SMNE01, or Europe, SMNP01) once. Easiest: the "
                + "folder Dolphin's "
                + "Extract Entire Disc makes (DATA, or the folder that holds it), or a zip of that folder. "
                + "Your saves are kept.",
                "Importa una volta il tuo New Super Mario Bros. Wii (USA, SMNE01, o Europa, SMNP01). Il modo "
                + "più semplice: la "
                + "cartella creata da Estrai intero disco di Dolphin (DATA, o la cartella che la contiene), "
                + "o uno zip di quella cartella. I salvataggi vengono mantenuti.");
        it("Import from Extracted Game Data Folder…", "Importa dalla cartella dei dati estratti…");
        it("Import Game Data Zip…", "Importa zip dei dati del gioco…");
        it("Import Disc Image (ISO, WBFS, RVZ)…", "Importa immagine disco (ISO, WBFS, RVZ)…");
        it("Or import your disc image straight away (.iso, .wbfs, .rvz or .wia): it is extracted once.",
                "Oppure importa direttamente l'immagine disco (.iso, .wbfs, .rvz o .wia): viene estratta una sola volta.");
        it("That isn't a Wii disc image this app can read (ISO, WBFS, RVZ or WIA).",
                "Non è un'immagine disco Wii che questa app può leggere (ISO, WBFS, RVZ o WIA).");
        it("Extracting the disc image…", "Estrazione dell'immagine disco…");
        it("Import Disc Image (.iso)…", "Importa immagine disco (.iso)…");
        it("Have the disc image itself? Import the .iso: it's extracted on this phone, the way Dolphin does it, "
                + "with no computer needed.",
                "Hai l'immagine del disco? Importa il file .iso: viene estratto su questo telefono, come fa "
                + "Dolphin, senza bisogno di un computer.");
        it("Extracting the disc image", "Estrazione dell'immagine disco");
        it("Reading the disc image…", "Lettura dell'immagine disco…");
        it("That file isn't a Wii disc image. Pick the .iso itself; compressed images (.rvz, .wbfs, .wia) can be "
                + "turned into one with Dolphin's Convert File.",
                "Quel file non è un'immagine disco Wii. Scegli il file .iso; le immagini compresse (.rvz, .wbfs, "
                + ".wia) si possono convertire con Converti file di Dolphin.");
        it("There isn't enough free space on this phone: the game takes %d MB.",
                "Non c'è abbastanza spazio libero su questo telefono: il gioco occupa %d MB.");
        it("that isn't a Wii disc image", "non è un'immagine disco Wii");
        it("that disc image isn't encrypted the way a Wii disc is", "quell'immagine non è cifrata come un disco Wii");
        it("the disc image is cut short", "l'immagine disco è incompleta");
        it("that disc image isn't from a retail Wii disc", "quell'immagine non viene da un disco Wii originale");
        it("that disc image has no game partition", "quell'immagine disco non ha una partizione di gioco");
        it("the disc's file table is damaged", "la tabella dei file del disco è danneggiata");
        it("Download Logs (For Github Reports)", "Scarica i log (per le segnalazioni su GitHub)");
        it("No game data yet. Import the folder Dolphin made, or a zip of it.",
                "Ancora nessun dato del gioco. Importa la cartella creata da Dolphin, o un suo zip.");
        it("The imported game can't be played. %s", "Il gioco importato non si può avviare. %s");
        it("Game data is installed and checked.", "I dati del gioco sono installati e verificati.");
        it("Starting the LAN game...", "Avvio della partita in LAN...");
        it("OK", "OK");
        it("Done", "Fatto");
        it("Back", "Indietro");
        it("Cancel", "Annulla");

        it("Checking for updates…", "Ricerca aggiornamenti…");
        it("Couldn't check for updates: %s", "Impossibile cercare aggiornamenti: %s");
        it("The latest release has no Android download.", "L'ultima versione non ha un download per Android.");
        it("You have the latest version (%s).", "Hai l'ultima versione (%s).");
        it("NSMBW %s is available", "NSMBW %s è disponibile");
        it("To install updates, let NSMBW install apps: turn on Allow from this source on the next screen, "
                + "then come back here.",
                "Per installare gli aggiornamenti, consenti a NSMBW di installare app: attiva Consenti da "
                + "questa fonte nella prossima schermata, poi torna qui.");
        it("Open Settings", "Apri impostazioni");
        it("Not Now", "Non ora");
        it("Downloading…", "Download…");
        it("Downloading… %d of %d MB", "Download… %d di %d MB");
        it("Updating to NSMBW %s", "Aggiornamento a NSMBW %s");
        it("The update failed: %s", "L'aggiornamento non è riuscito: %s");
        it("Installing NSMBW %s…", "Installazione di NSMBW %s…");

        it("Getting Started", "Primi passi");
        it("1. Import New Super Mario Bros. Wii", "1. Importa New Super Mario Bros. Wii");
        it("Use your own New Super Mario Bros. Wii for the USA (SMNE01) or Europe (SMNP01), revision 1. "
                + "Easiest: in Dolphin on a "
                + "computer, right-click the game, choose Properties, Filesystem, then right-click the disc and "
                + "choose Extract Entire Disc. Copy that folder (or a zip of it) to this phone and tap Import. "
                + "Dolphin shows the revision under Properties > Info.",
                "Usa il tuo New Super Mario Bros. Wii USA (SMNE01) o europeo (SMNP01), revisione 1. Il modo più "
                + "semplice: in "
                + "Dolphin su un computer, fai clic destro sul gioco, scegli Proprietà, Filesystem, poi fai "
                + "clic destro sul disco e scegli Estrai intero disco. Copia quella cartella (o un suo zip) "
                + "su questo telefono e tocca Importa. Dolphin mostra la revisione in Proprietà > Info.");
        it("2. Play", "2. Gioca");
        it("Tap Play. For LAN play, move up to the Couch button at the top right of the save file screen and "
                + "switch it to LAN; every player imports their own game.",
                "Tocca Gioca. Per giocare in LAN, sali fino al pulsante Locale in alto a destra nella "
                + "schermata dei file e passalo a LAN; ogni giocatore importa il proprio gioco.");
        it("Stuck on a step?", "Bloccato a un passaggio?");
        it("The GitHub page covers the supported game, the steps above and common problems.",
                "La pagina GitHub spiega il gioco supportato, i passaggi qui sopra e i problemi comuni.");
        it("Setup Guide on GitHub", "Guida all'installazione su GitHub");

        it("Saved, but there are no game logs yet: play once, then download them again.",
                "Salvato, ma non ci sono ancora log del gioco: gioca una volta, poi scaricali di nuovo.");
        it("Saved the logs of the last game session. Attach the zip to your GitHub issue.",
                "Salvati i log dell'ultima sessione di gioco. Allega lo zip alla tua segnalazione su GitHub.");
        it("Saved the logs of the last %d game sessions. Attach the zip to your GitHub issue.",
                "Salvati i log delle ultime %d sessioni di gioco. Allega lo zip alla tua segnalazione su "
                + "GitHub.");
        it("Could not save the logs: %s", "Impossibile salvare i log: %s");

        it("Looking at the game files…", "Controllo dei file del gioco…");
        it("Importing game data", "Importazione dei dati del gioco");
        it("%s\n\nNothing was changed.", "%s\n\nNon è stato cambiato nulla.");
        it("Import failed: %s", "Importazione non riuscita: %s");
        it("Imported %d files. Tap Play to start.", "Importati %d file. Tocca Gioca per iniziare.");
        it("%d of %d files", "%d di %d file");
        it("%d of %d MB", "%d di %d MB");
        it("%d files", "%d file");
        it("The folder doesn't include all required files.", "La cartella non include tutti i file necessari.");
        it("That folder's sys/boot.bin is too short, so the game can't be identified.",
                "Il sys/boot.bin di quella cartella è troppo corto, quindi il gioco non può essere "
                + "identificato.");
        it("Its sys/boot.bin is too short, so the game can't be identified.",
                "Il suo sys/boot.bin è troppo corto, quindi il gioco non può essere identificato.");
        it("That is %s, not New Super Mario Bros. Wii for the USA (SMNE01) or Europe (SMNP01). This port "
                + "runs those discs, revision 1.",
                "Questo è %s, non New Super Mario Bros. Wii USA (SMNE01) o europeo (SMNP01). Questo port "
                + "funziona con quei dischi, revisione 1.");
        it("That is New Super Mario Bros. Wii revision %d. This port runs only revision 1 (Dolphin shows it "
                + "under Properties > Info).",
                "Questo è New Super Mario Bros. Wii revisione %d. Questo port funziona solo con la revisione 1 "
                + "(Dolphin la mostra in Proprietà > Info).");

        it("Display", "Schermo");
        it("Controls", "Comandi");
        it("Aspect Ratio", "Formato");
        it("Render Resolution", "Risoluzione di rendering");
        it("Performance Monitor", "Monitor prestazioni");
        it("Spinning Coins", "Monete rotanti");
        it("V-Sync", "V-Sync");
        it("Graphics Compatibility", "Compatibilità grafica");
        it("Renderer", "Renderer");
        it("Automatic", "Automatico");
        it("Automatic (recommended): Vulkan, else OpenGL ES", "Automatico (consigliato): Vulkan, altrimenti OpenGL ES");
        it("OpenGL ES 3.1+ (in a later version)", "OpenGL ES 3.1+ (in una versione futura)");
        it("Auto", "Auto");
        it("Auto (recommended): on for the GPUs that need it", "Auto (consigliato): attiva per le GPU che ne hanno bisogno");
        it("On: always use the GPU fixes", "Sì: usa sempre le correzioni per la GPU");
        it("Off: never use them", "No: non usarle mai");
        it("Original 4:3", "4:3 originale");
        it("Fill Screen", "Riempi lo schermo");
        it("Match screen", "Come lo schermo");
        it("Touch Controls", "Comandi touch");
        it("Touch Control Opacity", "Opacità dei comandi touch");
        it("Tilt (turn the phone)", "Inclinazione (ruota il telefono)");
        it("Shake (jolt the phone)", "Scossa (scuoti il telefono)");
        it("Vibration", "Vibrazione");

        it("This phone", "Questo telefono");
        it("No network", "Nessuna rete");
        it("Rooms on your Wi-Fi are found by themselves. Over Tailscale, PCs find every device, and devices "
                + "that have played together find each other. To find another phone the first time, add its "
                + "Tailscale name or address here (or it adds yours).",
                "Le stanze sulla tua rete Wi-Fi si trovano da sole. Con Tailscale, i PC trovano ogni "
                + "dispositivo e i dispositivi che hanno già giocato insieme si ritrovano. Per trovare un "
                + "altro telefono la prima volta, aggiungi qui il suo nome o indirizzo Tailscale (o lui "
                + "aggiunge il tuo).");
        it("Devices asked for rooms", "Dispositivi a cui chiedere le stanze");
        it("None yet", "Ancora nessuno");
        it("Stop asking %s for rooms?", "Smettere di chiedere le stanze a %s?");
        it("Remove", "Rimuovi");
        it("Add Device…", "Aggiungi dispositivo…");
        it("Add a Device", "Aggiungi un dispositivo");
        it("Its Tailscale name (as in the Tailscale app) or address. It is asked for rooms from now on.",
                "Il suo nome Tailscale (come nell'app Tailscale) o il suo indirizzo. D'ora in poi gli verranno "
                + "chieste le stanze.");
        it("Add", "Aggiungi");
        it("e.g. s26-ultra or 100.84.140.32", "es. s26-ultra o 100.84.140.32");

        it("Start Menu", "Menu iniziale");
        it("Leave LAN Game", "Esci dalla partita LAN");
        it("Quit Game", "Esci dal gioco");
        it("Back to the Game", "Torna al gioco");
        it("Connect", "Connetti");
        it("To install updates, let NSMBW install apps: turn on Allow from this source on the next screen, "
                + "then come back to the game.",
                "Per installare gli aggiornamenti, consenti a NSMBW di installare app: attiva Consenti da "
                + "questa fonte nella prossima schermata, poi torna al gioco.");

        // --- the language pack (only a language the disc doesn't have shows these) ---
        it("Language Pack…", "Pacchetto lingua…");
        it("Language Pack", "Pacchetto lingua");
        it("A language pack is installed: the game's own text is in this language.",
                "È installato un pacchetto lingua: anche i testi del gioco sono in questa lingua.");
        it("The start menu and the port's menus are in this language. For the game's own text, import a "
                + "language pack zip made by a translator; until then it stays English.",
                "Il menu iniziale e i menu del port sono in questa lingua. Per i testi del gioco, importa lo "
                + "zip di un pacchetto lingua creato da un traduttore; fino ad allora restano in inglese.");
        it("Replace…", "Sostituisci…");
        it("Import Language Pack…", "Importa pacchetto lingua…");
        it("Importing the language pack", "Importazione del pacchetto lingua");
        it("That zip doesn't hold a language pack: it needs a Message folder, as in the disc's files/US/EngUS.",
                "Quello zip non contiene un pacchetto lingua: serve una cartella Message, come in "
                + "files/US/EngUS del disco.");
        it("The language pack is installed. The game uses it the next time it starts.",
                "Il pacchetto lingua è installato. Il gioco lo userà al prossimo avvio.");

        // --- lines the app shows that had no row yet ---
        newer("The game stopped", "El juego se detuvo", "Le jeu s'est arrêté", "Il gioco si è fermato");
        newer("New Super Mario Bros. Wii, the Rust port", "New Super Mario Bros. Wii, el port en Rust",
                "New Super Mario Bros. Wii, le portage en Rust", "New Super Mario Bros. Wii, il port in Rust");
        newer("Tap Play. The on-screen Wii Remote is held sideways; turn the phone to tilt and jolt it to shake. "
                + "The Back gesture opens the game's menu.",
                "Toca Jugar. El Wii Remote en pantalla se sujeta de lado; gira el teléfono para inclinarlo y "
                + "sacúdelo para agitarlo. El gesto Atrás abre el menú del juego.",
                "Touchez Jouer. La Wiimote à l'écran se tient à l'horizontale ; tournez le téléphone pour "
                + "l'incliner et secouez-le pour la secouer. Le geste Retour ouvre le menu du jeu.",
                "Tocca Gioca. Il Wii Remote sullo schermo si tiene di lato; ruota il telefono per inclinarlo e "
                + "scuotilo per agitarlo. Il gesto Indietro apre il menu del gioco.");
        newer("16:9 and 4:3 add black bars. Fill Screen shows 16:9 in this port for now.",
                "16:9 y 4:3 añaden barras negras. Llenar pantalla muestra 16:9 en este port por ahora.",
                "16:9 et 4:3 ajoutent des bandes noires. Plein écran affiche du 16:9 dans ce portage pour l'instant.",
                "16:9 e 4:3 aggiungono bande nere. Riempi lo schermo per ora mostra 16:9 in questo port.");
        newer("Takes effect at the next start.", "Se aplica en el próximo inicio.", "Prend effet au prochain démarrage.",
                "Vale dal prossimo avvio.");
        newer("Reverse Tilt", "Inclinación invertida", "Inclinaison inversée", "Inclinazione invertita");
        newer("Tilting platforms go the way you tilt, whatever the tilt comes from.",
                "Las plataformas inclinables van hacia donde inclinas, venga de donde venga la inclinación.",
                "Les plateformes inclinables penchent du côté où vous inclinez, d'où que vienne l'inclinaison.",
                "Le piattaforme inclinabili vanno dove inclini, da qualunque cosa venga l'inclinazione.");

        // --- German, by the same English wording ---
        de("Language", "Sprache");
        de("Help", "Hilfe");
        de("New Super Mario Bros. Wii, recompiled to run natively",
                "New Super Mario Bros. Wii, neu kompiliert, um nativ zu laufen");
        de("Play", "Spielen");
        de("Mods", "Mods");
        de("Coming Soon!", "Demnächst!");
        de("Updates", "Updates");
        de("Check for Updates", "Nach Updates suchen");
        de("Auto Update", "Automatische Updates");
        de("On: opening the app installs the latest release from GitHub when there is one. Your saves, "
                + "settings and game files are kept.",
                "An: Beim Öffnen installiert die App die neueste Version von GitHub, falls es eine gibt. "
                + "Spielstände, Einstellungen und Spieldateien bleiben erhalten.");
        de("Options", "Optionen");
        de("Settings", "Einstellungen");
        de("Tailscale & LAN Devices", "Tailscale- und LAN-Geräte");
        de("Skip Start Menu", "Startmenü überspringen");
        de("On: opening the app goes straight into the game. In the game, the phone's Back gesture and "
                + "Start Menu bring you back here.",
                "An: Die App startet direkt das Spiel. Im Spiel bringen dich die Zurück-Geste des Telefons und "
                + "Startmenü hierher zurück.");
        de("Xiaomi Mode", "Xiaomi-Modus");
        de("On: the game runs in the app's main process. For Xiaomi, Redmi and POCO phones (on by default "
                + "there), whose system closes the game's own process and leaves a black screen.",
                "An: Das Spiel läuft im Hauptprozess der App. Für Xiaomi-, Redmi- und POCO-Telefone (dort "
                + "standardmäßig an), deren System den eigenen Prozess des Spiels schließt und einen schwarzen "
                + "Bildschirm hinterlässt.");
        de("GPU Driver", "GPU-Treiber");
        de("Run the game on a driver from a zip, such as Turnip, in place of the phone's own. If a new "
                + "driver stops the game from starting, Default comes back by itself.",
                "Startet das Spiel mit einem Treiber aus einer ZIP-Datei, etwa Turnip, statt mit dem des "
                + "Telefons. Startet das Spiel mit einem neuen Treiber nicht, kehrt Standard von selbst zurück.");
        de("Only for phones with a Snapdragon chip.", "Nur für Telefone mit Snapdragon-Chip.");
        de("Default", "Standard");
        de("Import…", "Importieren…");
        de("Delete…", "Löschen…");
        de("Delete a GPU driver", "Einen GPU-Treiber löschen");
        de("Importing the GPU driver", "GPU-Treiber wird importiert");
        de("%s is ready. The game uses it from its next start.",
                "%s ist bereit. Das Spiel nutzt ihn ab dem nächsten Start.");
        de("This zip isn't a GPU driver: it has no meta.json.",
                "Diese ZIP-Datei ist kein GPU-Treiber: Sie hat keine meta.json.");
        de("This zip isn't a GPU driver: its meta.json can't be read.",
                "Diese ZIP-Datei ist kein GPU-Treiber: Ihre meta.json ist nicht lesbar.");
        de("This zip isn't a GPU driver: the library its meta.json names isn't in it.",
                "Diese ZIP-Datei ist kein GPU-Treiber: Die Bibliothek aus ihrer meta.json fehlt darin.");
        de("This driver needs Android API %s or newer.",
                "Dieser Treiber braucht Android-API %s oder neuer.");
        de("The game closed right after starting on the new GPU driver, so it's back on Default. You can "
                + "pick that driver again under GPU Driver.",
                "Das Spiel wurde mit dem neuen GPU-Treiber direkt nach dem Start beendet, daher gilt wieder "
                + "Standard. Unter GPU-Treiber kannst du ihn erneut wählen.");
        de("On", "An");
        de("Off", "Aus");
        de("Game Data", "Spieldaten");
        de("Get the game files (once)", "Die Spieldateien holen (einmalig)");
        de("1. In Dolphin on a computer, right-click New Super Mario Bros. Wii and choose Properties.\n"
                + "2. Open Filesystem, right-click the disc at the top and choose Extract Entire Disc.\n"
                + "3. Copy that folder (or a zip of it) to this phone.\n"
                + "4. Tap Import from Extracted Game Data Folder… or Import Game Data Zip… below and "
                + "choose it.\n\n"
                + "Use the USA (SMNE01) or European (SMNP01) disc, revision 1: Dolphin shows the revision under "
                + "Properties > Info.",
                "1. Klicke in Dolphin auf einem Computer mit der rechten Maustaste auf New Super Mario Bros. Wii "
                + "und wähle Eigenschaften.\n"
                + "2. Öffne Dateisystem, klicke oben mit der rechten Maustaste auf die Disc und wähle Gesamte "
                + "Disc extrahieren.\n"
                + "3. Kopiere diesen Ordner (oder eine ZIP-Datei davon) auf dieses Telefon.\n"
                + "4. Tippe unten auf Aus extrahiertem Spieldaten-Ordner importieren… oder Spieldaten-ZIP "
                + "importieren… und wähle ihn aus.\n\n"
                + "Nutze die US-Disc (SMNE01) oder die europäische (SMNP01), Revision 1: Dolphin zeigt die "
                + "Revision unter Eigenschaften > Info.");
        de("Import your own New Super Mario Bros. Wii (USA, SMNE01, or Europe, SMNP01) once. Easiest: the "
                + "folder Dolphin's "
                + "Extract Entire Disc makes (DATA, or the folder that holds it), or a zip of that folder. "
                + "Your saves are kept.",
                "Importiere einmal dein eigenes New Super Mario Bros. Wii (USA, SMNE01, oder Europa, SMNP01). "
                + "Am einfachsten: der Ordner, den Gesamte Disc extrahieren in Dolphin erstellt (DATA oder der "
                + "Ordner, der ihn enthält), oder eine ZIP-Datei davon. Deine Spielstände bleiben erhalten.");
        de("Import from Extracted Game Data Folder…", "Aus extrahiertem Spieldaten-Ordner importieren…");
        de("Import Game Data Zip…", "Spieldaten-ZIP importieren…");
        de("Import Disc Image (.iso)…", "Disc-Image importieren (.iso)…");
        de("Import Disc Image (ISO, WBFS, RVZ)…", "Disc-Image importieren (ISO, WBFS, RVZ)…");
        de("Or import your disc image straight away (.iso, .wbfs, .rvz or .wia): it is extracted once.",
                "Oder importiere direkt dein Disc-Image (.iso, .wbfs, .rvz oder .wia): Es wird einmal extrahiert.");
        de("That isn't a Wii disc image this app can read (ISO, WBFS, RVZ or WIA).",
                "Das ist kein Wii-Disc-Image, das diese App lesen kann (ISO, WBFS, RVZ oder WIA).");
        de("Extracting the disc image…", "Disc-Image wird extrahiert…");
        de("Have the disc image itself? Import the .iso: it's extracted on this phone, the way Dolphin does it, "
                + "with no computer needed.",
                "Hast du das Disc-Image selbst? Importiere die .iso: Sie wird auf diesem Telefon extrahiert, wie "
                + "Dolphin es tut, ganz ohne Computer.");
        de("Extracting the disc image", "Disc-Image wird extrahiert");
        de("Reading the disc image…", "Disc-Image wird gelesen…");
        de("That file isn't a Wii disc image. Pick the .iso itself; compressed images (.rvz, .wbfs, .wia) can be "
                + "turned into one with Dolphin's Convert File.",
                "Diese Datei ist kein Wii-Disc-Image. Wähle die .iso selbst; komprimierte Images (.rvz, .wbfs, "
                + ".wia) lassen sich mit Datei konvertieren in Dolphin umwandeln.");
        de("There isn't enough free space on this phone: the game takes %d MB.",
                "Auf diesem Telefon ist nicht genug Speicher frei: Das Spiel braucht %d MB.");
        de("that isn't a Wii disc image", "das ist kein Wii-Disc-Image");
        de("that disc image isn't encrypted the way a Wii disc is",
                "dieses Disc-Image ist nicht wie eine Wii-Disc verschlüsselt");
        de("the disc image is cut short", "das Disc-Image ist unvollständig");
        de("that disc image isn't from a retail Wii disc", "dieses Image stammt nicht von einer Wii-Kauf-Disc");
        de("that disc image has no game partition", "dieses Disc-Image hat keine Spielpartition");
        de("the disc's file table is damaged", "die Dateitabelle der Disc ist beschädigt");
        de("Download Logs (For Github Reports)", "Logs herunterladen (für GitHub-Meldungen)");
        de("No game data yet. Import the folder Dolphin made, or a zip of it.",
                "Noch keine Spieldaten. Importiere den Ordner, den Dolphin erstellt hat, oder eine ZIP-Datei davon.");
        de("The imported game can't be played. %s", "Das importierte Spiel kann nicht gespielt werden. %s");
        de("Game data is installed and checked.", "Die Spieldaten sind installiert und geprüft.");
        de("Starting the LAN game...", "LAN-Spiel wird gestartet...");
        de("OK", "OK");
        de("Done", "Fertig");
        de("Back", "Zurück");
        de("Cancel", "Abbrechen");

        de("Checking for updates…", "Suche nach Updates…");
        de("Couldn't check for updates: %s", "Suche nach Updates fehlgeschlagen: %s");
        de("The latest release has no Android download.", "Die neueste Version hat keinen Android-Download.");
        de("You have the latest version (%s).", "Du hast die neueste Version (%s).");
        de("NSMBW %s is available", "NSMBW %s ist verfügbar");
        de("To install updates, let NSMBW install apps: turn on Allow from this source on the next screen, "
                + "then come back here.",
                "Um Updates zu installieren, erlaube NSMBW, Apps zu installieren: Aktiviere auf dem nächsten "
                + "Bildschirm Dieser Quelle vertrauen und komm dann hierher zurück.");
        de("Open Settings", "Einstellungen öffnen");
        de("Not Now", "Nicht jetzt");
        de("Downloading…", "Wird heruntergeladen…");
        de("Downloading… %d of %d MB", "Wird heruntergeladen… %d von %d MB");
        de("Updating to NSMBW %s", "Update auf NSMBW %s");
        de("The update failed: %s", "Das Update ist fehlgeschlagen: %s");
        de("Installing NSMBW %s…", "NSMBW %s wird installiert…");

        de("Getting Started", "Erste Schritte");
        de("1. Import New Super Mario Bros. Wii", "1. New Super Mario Bros. Wii importieren");
        de("Use your own New Super Mario Bros. Wii for the USA (SMNE01) or Europe (SMNP01), revision 1. "
                + "Easiest: in Dolphin on a "
                + "computer, right-click the game, choose Properties, Filesystem, then right-click the disc and "
                + "choose Extract Entire Disc. Copy that folder (or a zip of it) to this phone and tap Import. "
                + "Dolphin shows the revision under Properties > Info.",
                "Nutze dein eigenes New Super Mario Bros. Wii für die USA (SMNE01) oder Europa (SMNP01), "
                + "Revision 1. Am einfachsten: Klicke in Dolphin auf einem Computer mit der rechten Maustaste auf "
                + "das Spiel, wähle Eigenschaften, Dateisystem, klicke dann mit der rechten Maustaste auf die Disc "
                + "und wähle Gesamte Disc extrahieren. Kopiere diesen Ordner (oder eine ZIP-Datei davon) auf "
                + "dieses Telefon und tippe auf Importieren. Dolphin zeigt die Revision unter Eigenschaften > Info.");
        de("2. Play", "2. Spielen");
        de("Tap Play. For LAN play, move up to the Couch button at the top right of the save file screen and "
                + "switch it to LAN; every player imports their own game.",
                "Tippe auf Spielen. Für LAN-Spiele gehst du zur Couch-Schaltfläche oben rechts im "
                + "Dateiauswahl-Bildschirm und stellst sie auf LAN; jeder Spieler importiert sein eigenes Spiel.");
        de("Stuck on a step?", "Bei einem Schritt hängen geblieben?");
        de("The GitHub page covers the supported game, the steps above and common problems.",
                "Die GitHub-Seite erklärt das unterstützte Spiel, die Schritte oben und häufige Probleme.");
        de("Setup Guide on GitHub", "Einrichtungsanleitung auf GitHub");

        de("Saved, but there are no game logs yet: play once, then download them again.",
                "Gespeichert, aber es gibt noch keine Spiel-Logs: Spiele einmal und lade sie dann erneut herunter.");
        de("Saved the logs of the last game session. Attach the zip to your GitHub issue.",
                "Die Logs der letzten Spielsitzung wurden gespeichert. Hänge die ZIP-Datei an deine GitHub-Meldung an.");
        de("Saved the logs of the last %d game sessions. Attach the zip to your GitHub issue.",
                "Die Logs der letzten %d Spielsitzungen wurden gespeichert. Hänge die ZIP-Datei an deine "
                + "GitHub-Meldung an.");
        de("Could not save the logs: %s", "Die Logs konnten nicht gespeichert werden: %s");

        de("Looking at the game files…", "Spieldateien werden geprüft…");
        de("Importing game data", "Spieldaten werden importiert");
        de("%s\n\nNothing was changed.", "%s\n\nEs wurde nichts geändert.");
        de("Import failed: %s", "Import fehlgeschlagen: %s");
        de("Imported %d files. Tap Play to start.", "%d Dateien importiert. Tippe zum Starten auf Spielen.");
        de("%d of %d files", "%d von %d Dateien");
        de("%d of %d MB", "%d von %d MB");
        de("%d files", "%d Dateien");
        de("The folder doesn't include all required files.", "Im Ordner fehlen benötigte Dateien.");
        de("That folder's sys/boot.bin is too short, so the game can't be identified.",
                "Die sys/boot.bin dieses Ordners ist zu kurz, daher lässt sich das Spiel nicht erkennen.");
        de("Its sys/boot.bin is too short, so the game can't be identified.",
                "Seine sys/boot.bin ist zu kurz, daher lässt sich das Spiel nicht erkennen.");
        de("That is %s, not New Super Mario Bros. Wii for the USA (SMNE01) or Europe (SMNP01). This port "
                + "runs those discs, revision 1.",
                "Das ist %s, nicht New Super Mario Bros. Wii für die USA (SMNE01) oder Europa (SMNP01). Dieser "
                + "Port läuft mit diesen Discs, Revision 1.");
        de("That is New Super Mario Bros. Wii revision %d. This port runs only revision 1 (Dolphin shows it "
                + "under Properties > Info).",
                "Das ist New Super Mario Bros. Wii Revision %d. Dieser Port läuft nur mit Revision 1 (Dolphin "
                + "zeigt sie unter Eigenschaften > Info).");

        de("Display", "Anzeige");
        de("Controls", "Steuerung");
        de("Aspect Ratio", "Seitenverhältnis");
        de("Render Resolution", "Render-Auflösung");
        de("Performance Monitor", "Leistungsanzeige");
        de("Spinning Coins", "Drehende Münzen");
        de("V-Sync", "V-Sync");
        de("Graphics Compatibility", "Grafik-Kompatibilität");
        de("Renderer", "Renderer");
        de("Automatic", "Automatisch");
        de("Automatic (recommended): Vulkan, else OpenGL ES", "Automatisch (empfohlen): Vulkan, sonst OpenGL ES");
        de("OpenGL ES 3.1+ (in a later version)", "OpenGL ES 3.1+ (in einer späteren Version)");
        de("Auto", "Auto");
        de("Auto (recommended): on for the GPUs that need it", "Auto (empfohlen): an für GPUs, die es brauchen");
        de("On: always use the GPU fixes", "An: GPU-Korrekturen immer nutzen");
        de("Off: never use them", "Aus: nie nutzen");
        de("Original 4:3", "Original 4:3");
        de("Fill Screen", "Bildschirm füllen");
        de("Match screen", "Wie Bildschirm");
        de("Touch Controls", "Touch-Steuerung");
        de("Touch Control Opacity", "Deckkraft der Touch-Steuerung");
        de("Tilt (turn the phone)", "Neigen (Telefon drehen)");
        de("Shake (jolt the phone)", "Schütteln (Telefon rucken)");
        de("Vibration", "Vibration");

        de("This phone", "Dieses Telefon");
        de("No network", "Kein Netzwerk");
        de("Rooms on your Wi-Fi are found by themselves. Over Tailscale, PCs find every device, and devices "
                + "that have played together find each other. To find another phone the first time, add its "
                + "Tailscale name or address here (or it adds yours).",
                "Räume in deinem WLAN werden von selbst gefunden. Über Tailscale finden PCs jedes Gerät, und "
                + "Geräte, die schon zusammen gespielt haben, finden einander. Um ein anderes Telefon zum ersten "
                + "Mal zu finden, füge hier seinen Tailscale-Namen oder seine Adresse hinzu (oder es fügt deine hinzu).");
        de("Devices asked for rooms", "Nach Räumen gefragte Geräte");
        de("None yet", "Noch keine");
        de("Stop asking %s for rooms?", "%s nicht mehr nach Räumen fragen?");
        de("Remove", "Entfernen");
        de("Add Device…", "Gerät hinzufügen…");
        de("Add a Device", "Ein Gerät hinzufügen");
        de("Its Tailscale name (as in the Tailscale app) or address. It is asked for rooms from now on.",
                "Sein Tailscale-Name (wie in der Tailscale-App) oder seine Adresse. Es wird ab jetzt nach Räumen "
                + "gefragt.");
        de("Add", "Hinzufügen");
        de("e.g. s26-ultra or 100.84.140.32", "z. B. s26-ultra oder 100.84.140.32");

        de("Start Menu", "Startmenü");
        de("Leave LAN Game", "LAN-Spiel verlassen");
        de("Quit Game", "Spiel beenden");
        de("Back to the Game", "Zurück zum Spiel");
        de("Connect", "Verbinden");
        de("To install updates, let NSMBW install apps: turn on Allow from this source on the next screen, "
                + "then come back to the game.",
                "Um Updates zu installieren, erlaube NSMBW, Apps zu installieren: Aktiviere auf dem nächsten "
                + "Bildschirm Dieser Quelle vertrauen und kehre dann zum Spiel zurück.");

        de("Video", "Video");
        de("Practice", "Training");
        de("Game Speed", "Spieltempo");
        de("Level Timer", "Level-Zeit");
        de("Death Counter", "Verlorene Leben");
        de("Input Display", "Eingabeanzeige");
        de("Unlimited", "Unbegrenzt");
        de("Player 1", "Spieler 1");
        de("All Players", "Alle Spieler");
        de("The game, and its sound at its own pitch, at this speed. LAN games run at 100%.",
                "Das Spiel, und sein Ton in eigener Tonhöhe, in diesem Tempo. LAN-Spiele laufen mit 100 %.");
        de("The course's time in game frames from each start; it stops at the goal.",
                "Die Zeit im Level in Spielbildern ab jedem Start; sie stoppt im Ziel.");
        de("Lives lost in this course, and since the game started.",
                "Verlorene Leben in diesem Level und seit dem Spielstart.");
        de("The buttons the game gets, at the bottom left.", "Die Knöpfe, die das Spiel erhält, unten links.");
        de("Game speed and practice tools, applied when you go back to the game.",
                "Spieltempo und Trainingshilfen, wirksam, wenn du zum Spiel zurückkehrst.");
        de("Audio", "Audio");
        de("Resolution", "Auflösung");
        de("Rendered at this many lines, then scaled to the screen. Match screen: the screen's own.",
                "Mit so vielen Zeilen gezeichnet, dann auf den Bildschirm skaliert. Wie Bildschirm: die des "
                + "Bildschirms.");
        de("Fill Screen: a wider view filling the whole screen. 16:9 and 4:3 add black bars.",
                "Bildschirm füllen: eine breitere Sicht über den ganzen Bildschirm. 16:9 und 4:3 fügen schwarze "
                + "Balken hinzu.");
        de("Automatic: Vulkan, else OpenGL ES. Takes effect at the next start.",
                "Automatisch: Vulkan, sonst OpenGL ES. Gilt ab dem nächsten Start.");
        de("No tearing: frames wait for the display, the game keeps its pace.",
                "Kein Tearing: Bilder warten auf den Bildschirm, das Spiel behält sein Tempo.");
        de("FPS and each frame's time, top right. The slider sets its size.",
                "FPS und die Zeit jedes Bildes, oben rechts. Der Regler bestimmt die Größe.");
        de("Monitor Size", "Größe der Anzeige");
        de("Animated coins and blocks. Off saves a little GPU work.",
                "Animierte Münzen und Blöcke. Aus spart der GPU etwas Arbeit.");
        de("Fixes for phone GPUs that scramble the game's models. Auto: on for the GPUs that need them.",
                "Korrekturen für Telefon-GPUs, die die Modelle des Spiels verzerren. Auto: an für GPUs, die sie "
                + "brauchen.");
        de("Master", "Gesamt");
        de("Music", "Musik");
        de("Sound Effects", "Soundeffekte");
        de("The volume of everything.", "Die Lautstärke von allem.");
        de("The background music.", "Die Hintergrundmusik.");
        de("Jumps, coins, enemies and the rest.", "Sprünge, Münzen, Gegner und der Rest.");
        de("Menu Sounds", "Menütöne");
        de("The menus' cursor, select and back sounds.", "Die Cursor-, Auswahl- und Zurück-Töne der Menüs.");
        de("Depth Blur", "Tiefenunschärfe");
        de("The blur and heat haze of desert, beach, water and lava areas.",
                "Unschärfe und Hitzeflimmern in Wüste, Strand, Wasser und Lava.");
        de("The on-screen Wii Remote. A controller hides it while you use it.",
                "Die Wii-Fernbedienung auf dem Bildschirm. Ein Controller blendet sie aus, solange du ihn nutzt.");
        de("How solid the on-screen remote looks.", "Wie deckend die Fernbedienung auf dem Bildschirm aussieht.");
        de("Turning the phone tilts the Wii Remote, for the levels and platforms that use it.",
                "Das Telefon drehen neigt die Wii-Fernbedienung, für Level und Plattformen, die das nutzen.");
        de("A jolt of the phone shakes the Wii Remote: spin jumps and picking things up.",
                "Ein Ruck des Telefons schüttelt die Wii-Fernbedienung: Drehsprünge und Dinge aufheben.");
        de("A short buzz when a touch button is pressed.", "Ein kurzes Vibrieren beim Drücken eines Touch-Knopfs.");
        de("Restart the game to play in this language.", "Starte das Spiel neu, um in dieser Sprache zu spielen.");
        de("The game's own text needs a language pack: the start menu's language button imports one.",
                "Die Texte des Spiels brauchen ein Sprachpaket: Die Sprach-Schaltfläche im Startmenü importiert eins.");
        de("It has no files/rels/%s.rel.LZ: this port needs the whole disc, as Dolphin's Extract Entire Disc makes it.",
                "Es fehlt files/rels/%s.rel.LZ: Dieser Port braucht die ganze Disc, wie Gesamte Disc extrahieren "
                + "in Dolphin sie erstellt.");
        de("Its files/rels/%s is changed: it isn't the original disc (a modded or repacked image?). This port runs only the unchanged disc.",
                "Seine files/rels/%s ist verändert: Das ist nicht die Original-Disc (ein gemoddetes oder neu "
                + "gepacktes Image?). Dieser Port läuft nur mit der unveränderten Disc.");
        de("Gameplay", "Spiel");
        de("Power-Up Freeze", "Verwandlungspause");
        de("Everything stops for a moment while a player grows, shrinks or dies. Off: the level keeps moving.",
                "Alles hält kurz an, wenn ein Spieler wächst, schrumpft oder ein Leben verliert. Aus: Der Level "
                + "läuft weiter.");
        de("Player Collision", "Spieler-Kollision");
        de("Players bump into and stand on each other. Off: they pass through each other.",
                "Spieler stoßen zusammen und stehen aufeinander. Aus: Sie gehen durcheinander hindurch.");
        de("Online games keep the game's own rules. Takes effect at the next start.",
                "Online-Spiele behalten die Regeln des Spiels. Gilt ab dem nächsten Start.");
        de("No Time Limit", "Kein Zeitlimit");
        de("Infinite Lives", "Unendlich Leben");
        de("Delete", "Löschen");
        de("Profiles", "Profile");
        de("Save as a new profile", "Als neues Profil speichern");
        de("Every button is on the screen already.", "Alle Knöpfe sind schon auf dem Bildschirm.");
        de("D-Pad", "Steuerkreuz");
        de("Shake", "Schütteln");
        de("Are you sure?", "Bist du sicher?");
        de("This will mark your save as cheated.", "Dein Spielstand wird als geschummelt markiert.");
        de("Yes", "Ja");
        de("No", "Nein");
        de("Power-Up", "Power-Up");
        de("What the Cheat button gives you.", "Was dir der Cheat-Knopf gibt.");
        de("Cheat Button", "Cheat-Knopf");
        de("An on-screen button that gives you the Power-Up. Edit Touch Controls moves it.",
                "Ein Knopf auf dem Bildschirm, der dir das Power-Up gibt. Unter Touch-Steuerung bearbeiten "
                + "lässt er sich verschieben.");
        de("Small", "Klein");
        de("Super Mushroom", "Superpilz");
        de("Fire Flower", "Feuerblume");
        de("Mini Mushroom", "Mini-Pilz");
        de("Propeller Mushroom", "Propeller-Pilz");
        de("Penguin Suit", "Pinguin-Anzug");
        de("Ice Flower", "Eisblume");
        de("Star", "Stern");
        de("Hammer Suit", "Hammer-Anzug");
        de("The course timer stands still: take your time.", "Die Zeit im Level steht still: Lass dir Zeit.");
        de("A miss costs no life.", "Ein Fehler kostet kein Leben.");
        de("These keep the game's own rules in LAN play.", "Im LAN gelten die normalen Regeln des Spiels.");
        de("LAN Play", "LAN-Spiel");
        de("Play with other phones and PCs on your Wi-Fi, or over Tailscale. Every player needs their own "
                + "game and the same version of NSMBW.",
                "Spiele mit anderen Telefonen und PCs in deinem WLAN oder über Tailscale. Jeder Spieler braucht "
                + "sein eigenes Spiel und dieselbe Version von NSMBW.");
        de("Create Room", "Raum erstellen");
        de("Rooms", "Räume");
        de("Looking for rooms…", "Suche Räume…");
        de("Join by Address…", "Per Adresse beitreten…");
        de("Join a Room", "Einem Raum beitreten");
        de("The host's address, as its LAN Play shows it, or its Tailscale name.",
                "Die Adresse des Hosts, wie sein LAN-Spiel sie zeigt, oder sein Tailscale-Name.");
        de("Join", "Beitreten");
        de("Can't look for rooms: %s", "Suche nach Räumen nicht möglich: %s");
        de("Edit Touch Controls", "Touch-Steuerung bearbeiten");
        de("Drag a control to move it. Pinch it, or tap it and use Bigger or Smaller, to resize it.",
                "Ziehe ein Bedienelement, um es zu verschieben. Ziehe es mit zwei Fingern auf oder zu, oder "
                + "tippe es an und nutze Größer oder Kleiner, um seine Größe zu ändern.");
        de("Smaller", "Kleiner");
        de("Bigger", "Größer");
        de("Reset", "Zurücksetzen");
        de("Newer Super Mario Bros. Wii, recompiled to run natively",
                "Newer Super Mario Bros. Wii, neu kompiliert, um nativ zu laufen");
        de("Newer Game Data", "Newer-Spieldaten");
        de("1. Have your own Newer Super Mario Bros. Wii v1.30 for the USA (SMNE03) as a disc image (.iso).\n"
                + "2. Copy it to this phone and tap Import Disc Image (.iso)… below: it's extracted here.\n\n"
                + "Or extract it in Dolphin (Properties, Filesystem, Extract Entire Disc) and import that folder or a zip of it.",
                "1. Halte dein eigenes Newer Super Mario Bros. Wii v1.30 für die USA (SMNE03) als Disc-Image "
                + "(.iso) bereit.\n"
                + "2. Kopiere es auf dieses Telefon und tippe unten auf Disc-Image importieren (.iso)…: Es wird "
                + "hier extrahiert.\n\n"
                + "Oder extrahiere es in Dolphin (Eigenschaften, Dateisystem, Gesamte Disc extrahieren) und "
                + "importiere diesen Ordner oder eine ZIP-Datei davon.");
        de("Import your own Newer Super Mario Bros. Wii v1.30 (USA, SMNE03) once: its disc image, the folder "
                + "Dolphin's Extract Entire Disc makes from it, or a zip of that folder. Newer keeps saves of its own.",
                "Importiere einmal dein eigenes Newer Super Mario Bros. Wii v1.30 (USA, SMNE03): sein Disc-Image, "
                + "den Ordner, den Gesamte Disc extrahieren in Dolphin daraus erstellt, oder eine ZIP-Datei davon. "
                + "Newer hat eigene Spielstände.");
        de("Remove the NewerSMBW Support Module", "Das NewerSMBW-Zusatzmodul entfernen");
        de("Remove the NewerSMBW support module? Newer's game files and saves stay; download the module again "
                + "to play it.",
                "Das NewerSMBW-Zusatzmodul entfernen? Spieldateien und Spielstände von Newer bleiben; lade das "
                + "Modul erneut herunter, um es zu spielen.");
        de("Download the NewerSMBW support module!", "Lade das NewerSMBW-Zusatzmodul herunter!");
        de("Downloading the NewerSMBW support module", "NewerSMBW-Zusatzmodul wird heruntergeladen");
        de("The NewerSMBW support module is installed. Import your Newer Super Mario Bros. Wii, then tap Play.",
                "Das NewerSMBW-Zusatzmodul ist installiert. Importiere dein Newer Super Mario Bros. Wii und tippe "
                + "dann auf Spielen.");
        de("This build of the app comes without the NewerSMBW support module.",
                "Diese Version der App kommt ohne das NewerSMBW-Zusatzmodul.");
        de("Newer Super Mario Bros. Wii runs on a build of the game of its own: the support module, a separate "
                + "download from this version's GitHub release, so the app stays small for everyone else.",
                "Newer Super Mario Bros. Wii läuft mit einer eigenen Version des Spiels: dem Zusatzmodul, ein "
                + "separater Download aus der GitHub-Release dieser Version, damit die App für alle anderen klein "
                + "bleibt.");
        de("Newer Super Mario Bros. Wii needs its support module: download it below.",
                "Newer Super Mario Bros. Wii braucht sein Zusatzmodul: Lade es unten herunter.");
        de("No Newer game data yet. Import its disc image, the folder Dolphin made, or a zip of it.",
                "Noch keine Newer-Spieldaten. Importiere sein Disc-Image, den Ordner, den Dolphin erstellt hat, "
                + "oder eine ZIP-Datei davon.");
        de("That is another version of Newer Super Mario Bros. Wii. This port runs v1.30 for the USA.",
                "Das ist eine andere Version von Newer Super Mario Bros. Wii. Dieser Port läuft mit v1.30 für die USA.");
        de("That is %s, not Newer Super Mario Bros. Wii for the USA (SMNE03).",
                "Das ist %s, nicht Newer Super Mario Bros. Wii für die USA (SMNE03).");
        de("this version of the app has no Newer module", "diese Version der App hat kein Newer-Modul");
        de("GitHub answered %d", "GitHub antwortete %d");
        de("the download doesn't match this version's module",
                "der Download passt nicht zum Modul dieser Version");
        de("Continuing where you left off.", "Es geht da weiter, wo du aufgehört hast.");
        de("Your saves, settings and game files are kept.",
                "Spielstände, Einstellungen und Spieldateien bleiben erhalten.");
        de("Update", "Aktualisieren");
        de("Stand-ins for GPUs without dual-source blending or logic ops, such as most Mali GPUs. Auto: on "
                + "for the GPUs that need them. Takes effect at the next start.",
                "Ersatzlösungen für GPUs ohne Dual-Source-Blending oder Logikoperationen, etwa die meisten "
                + "Mali-GPUs. Auto: an für GPUs, die sie brauchen. Gilt ab dem nächsten Start.");

        de("Language Pack…", "Sprachpaket…");
        de("Language Pack", "Sprachpaket");
        de("A language pack is installed: the game's own text is in this language.",
                "Ein Sprachpaket ist installiert: Auch die Texte des Spiels sind in dieser Sprache.");
        de("The start menu and the port's menus are in this language. For the game's own text, import a "
                + "language pack zip made by a translator; until then it stays English.",
                "Das Startmenü und die Menüs des Ports sind in dieser Sprache. Für die Texte des Spiels "
                + "importiere die ZIP-Datei eines Sprachpakets von einem Übersetzer; bis dahin bleiben sie "
                + "Englisch.");
        de("Replace…", "Ersetzen…");
        de("Import Language Pack…", "Sprachpaket importieren…");
        de("Importing the language pack", "Sprachpaket wird importiert");
        de("That zip doesn't hold a language pack: it needs a Message folder, as in the disc's files/US/EngUS.",
                "Diese ZIP-Datei enthält kein Sprachpaket: Sie braucht einen Message-Ordner, wie in "
                + "files/US/EngUS der Disc.");
        de("The language pack is installed. The game uses it the next time it starts.",
                "Das Sprachpaket ist installiert. Das Spiel nutzt es beim nächsten Start.");

        de("The game stopped", "Das Spiel wurde beendet");
        de("New Super Mario Bros. Wii, the Rust port", "New Super Mario Bros. Wii, der Rust-Port");
        de("Tap Play. The on-screen Wii Remote is held sideways; turn the phone to tilt and jolt it to shake. "
                + "The Back gesture opens the game's menu.",
                "Tippe auf Spielen. Die Wii-Fernbedienung auf dem Bildschirm wird quer gehalten; drehe das "
                + "Telefon zum Neigen und ruck es zum Schütteln. Die Zurück-Geste öffnet das Menü des Spiels.");
        de("16:9 and 4:3 add black bars. Fill Screen shows 16:9 in this port for now.",
                "16:9 und 4:3 fügen schwarze Balken hinzu. Bildschirm füllen zeigt in diesem Port vorerst 16:9.");
        de("Takes effect at the next start.", "Gilt ab dem nächsten Start.");
        de("Reverse Tilt", "Neigung umkehren");
        de("Tilting platforms go the way you tilt, whatever the tilt comes from.",
                "Kippbare Plattformen neigen sich in deine Richtung, egal woher die Neigung kommt.");
    }
}
