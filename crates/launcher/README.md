# The launcher (`nsmbw-launcher.exe`)

A Windows launcher for the port: pick a version of the game, point it at your
own extracted copy, and play. It is built on its own
(`cargo build --release -p launcher`) and lives next to `nsmbw-port.exe`.

## What it does

- **Play** starts `nsmbw-port.exe --game <folder>` for the selected version and
  closes. If the game stops at once, the launcher stays and says what the game
  said.
- **Skip Launcher** (next to Play): from the next start the launcher starts
  the game straight away. Holding **Shift** while it starts, or **Back** on an
  Xbox controller, or `--launcher`, shows it again. A start that cannot reach
  the game (a moved folder, a missing program) shows the launcher with the
  reason.
- **Switch Version** chooses which game is played. Each version has its own
  colours, title and picture, and its own game folder, settings and saves.
- **Game folder**: a folder browser that works with a controller (Windows' own
  dialog is one button away; a folder can also be dropped on the window). The
  launcher reads the game ID and revision from `sys\boot.bin`, says what it
  found, refuses anything the version does not accept with the reason, and
  then runs the game's own validation (`nsmbw-port --check`).
- **Settings**: the selected version's display settings (single keys of its
  `settings.toml`; everything else in the file is left as it is) and the
  launcher's own switches.
- **Updates**: asks the release source for a newer release and shows a notice
  (`crates/port-update`). See below for what the game still has to do.
- **Controllers**: every screen. D-pad or left stick moves, A confirms, B goes
  back, Y opens Switch Version, X is "use this folder" in the browser, Start
  plays, the bumpers step through versions. Keyboard: arrows or WASD, Enter,
  Esc, Tab, F, Q/E; F11 is fullscreen. The hints at the bottom follow the
  device last used. A controller does nothing while the window is not in
  front.

## Art

Before a game folder is set, everything on screen is drawn by the launcher's
own code (`scene.rs`). Once it is set, the launcher shows the game's own art,
**read from the player's game folder at run time and never stored**:

- the background: the game's own course background models
  (`files/Object/bgA_*.arc`, `bgB_*.arc`), drawn live (`gamebg.rs`);
- the art card: the game's Wii Menu banner (`files/opening.bnr`), played with
  its animations (`banner.rs`);
- the course previews around it (`gameart.rs`).

Which files those are is part of each version's data. No image, logo, font or
sound of the game is in this repository; the type is Windows' own Segoe UI,
read from the system.

## What is kept where

Everything is under the folder the launcher is in (or `--root <folder>`):

| File | What |
|---|---|
| `launcher.toml` | the selected version, Skip Launcher, extra game options per version (`[options]`), the game program when it is not `nsmbw-port.exe` here |
| `update.toml` | whether to look for updates, and where (shared with the game) |
| `versions.toml` | optional: replaces the built-in list of versions |
| `port.toml`, `settings.toml`, `saves\nand`, `nsmbw-port.log` | the original game's: this folder is its working folder, so running `nsmbw-port.exe` here directly plays the same game with the same saves |
| `versions\<id>\…` | the same four for every other version |

## Adding a version

Versions are data: `src/versions.toml` (built in), or a `versions.toml` next
to the launcher. A `[[version]]` gives its name, the discs it accepts (game
ID, revision, files that must or must not be there), whether the port can
play it today (and the reason when not), the art to read from its game
folder, and its theme (colours and the drawn scene). The file's header
explains each field. No code changes are needed.

Newer Super Mario Bros. Wii is listed with `playable = false`: the port's
engine runs only the original USA disc's code, so its Play stays off and
says why. Its folder can already be set (ID `SMNE03` with `files\NewerRes`).

## Updates, and what the game still has to do

`crates/port-update` holds the check: `Config::load(folder)` reads
`update.toml`, `check(&config)` asks the release source (GitHub's "latest
release" JSON, or a file or address holding the same) and compares the tag
with `port_update::VERSION`. Nothing is downloaded or installed: the notice
offers the release's page.

With Skip Launcher on, the launcher is not shown, so it makes the check
after starting the game and, when a release is found, shows itself once at
the next start. For updates not to depend on the launcher at all, the game
should call the same crate itself (the session working on branch `ui`):

1. add `port-update = { path = "../port-update" }` to `crates/frontend`;
2. at start, off the main thread:
   `port_update::Config::load(&port_update::install_dir())`, and when
   `enabled`, `port_update::check(&config)`;
3. on `Status::Available(release)`, show it in the game's own menus (its
   version, `release.notes`, and `release.page` to open).

The release source defaults to this repository's releases
(`port_update::DEFAULT_SOURCE`); `update.toml`'s `source`, or the
environment variable `NSMBW_UPDATE_SOURCE`, replaces it (a local file works,
for testing). `port_update::VERSION` is the crate's own version: raise it
when a release is cut.

## Footprint

The launcher draws 60 frames a second while something is happening, 30 while
it only shows its drifting picture, 2 while another window is in front, and
none while minimised; it hands back the memory it needed only at start; and
it closes when the game starts. Measured on the development PC (in front,
idle, with a game's art loaded): about 4% of one core, 2% of the graphics
card, and 12 to 30 MB of memory in use.

## Testing without hands

As `nsmbw-port` has `--press` and `--shot`:
`nsmbw-launcher --root <folder> --script 30:down,45:a --shot 90:out.png --frames 120`
drives the menus with controller moves at given frames
(`up down left right a b x y start lb rb`) and the mouse
(`point@<x>x<y>`, `click@<x>x<y>`, the window being 720 units tall), saves
the window's picture, and closes. Set `NSMBW_UPDATE_SOURCE` to a local file
for such runs, so that nothing is asked of the network.
`cargo test -p launcher -p port-update` runs the unit tests.
