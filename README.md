![Screenshot from 2022-08-29 10-11-16](https://user-images.githubusercontent.com/77372584/187178819-a0b02458-d273-49c0-b123-5d46fa84f347.png)

# Checkout the game

- [Play on Itch.io](https://park-dev.itch.io/loot-goblin)
- [The top 10 games from Bevy Game Jam 2](https://youtu.be/VBMzaMEOhFI?t=1154)
- [Bevy Jam 2 Results](https://itch.io/jam/bevy-jam-2/results)

# Contributors

- Music by Twitchywhalez [YouTube](https://www.youtube.com/channel/UCSjKBALUTiv8prOCSqdu3xA)
  /[BandCamp](https://twitchywhalez.bandcamp.com/)
- Art by DarkDax [YouTube](https://www.youtube.com/darkdax) and InkeFaux
- Game programming by [@VanGeck](https://github.com/vanGeck), [@Jazzaro](https://github.com/Jazarro)
  , [@frederickjjoubert](https://github.com/frederickjjoubert), [@parK-dev](https://github.com/parK-dev)

# Hotkeys

- `Escape` will back out of the game to the main menu. If you're already on the main menu, it will instantly close the
  game.
- `F11` will toggle between `BorderlessFullscreen` and `Windowed` mode.
- `LShift + Click` will use / equip / consume items.
- `LCtrl + LAlt + Click` will delete items.
- `Drag` items to move them to the combining area.
- `Left-click` the combine button to combine items.
- `Space` when prompted to press it for the dungeon sim to continue.

![image](https://user-images.githubusercontent.com/77372584/187132899-5bfc5d74-efbe-4e23-a9d3-8ab93021d9ae.png)

# Config files

## Adding new config files

- Add it as a struct to `src/config/`.
- Add it to the `src/loading/systems/load_configs()` function to load as a resource.
- Add a ron file with default values to `assets/config/default`.

## Using the configs in systems

You can request it as a resource in systems: `config: Res<GridConfig>,`.

## Overriding configs

Every config file is present as a `ron` file in the `assets/config/default/` directory. If you need to change one of the
constants for yourself, for testing purposes, you can copy the relevant config file to `assets/config/override/` and
change the values to your wishes. Those files wil not be added to source control, so you're not messing with anyone
else's build.

Some suggestions on how to use this:

- Override the `audio.ron` file to turn down the music and sound effects.
- Override the `debug.ron` file to skip past the main menu when testing.
- Override the `log.ron` file to tweak to log filter.
