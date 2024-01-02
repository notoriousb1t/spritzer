from string import ascii_uppercase, digits
from math import floor
from customtkinter import (
    CTk,
    set_appearance_mode,
    set_default_color_theme,
    CTkButton,
    CTkLabel,
    CTkFrame,
    CTkCheckBox,
    CTkEntry,
    CTkScrollableFrame,
    CTkComboBox,
)
from random import Random
from tkinter import filedialog, messagebox, BooleanVar, StringVar
from library.Options import Options, DungeonEnemyShuffle, OverworldEnemyShuffle
from library.Patch import patch_file

_PADX = 12
_PADY = 8
_WINDOW = (600, 400)


class Adjuster:
    def __init__(self) -> None:
        self.app = CTk()

        # Generate a random string for the seed.
        random_seed = ''.join(Random().choices(ascii_uppercase + digits, k=16))

        self.seed = StringVar(self.app, value=random_seed)
        self.enable_boss_shuffle = BooleanVar(self.app, False)
        self.enable_dungeon_palette_shuffle = BooleanVar(self.app, True)
        self.dungeon_enemy_shuffle = StringVar(self.app, DungeonEnemyShuffle.SIMPLE)
        self.overworld_enemy_shuffle = StringVar(self.app, OverworldEnemyShuffle.INVERTED)
        self.enable_dungeon_tileset_shuffle = BooleanVar(self.app, False)
        self.enable_killable_thieves = BooleanVar(self.app, False)
        self.enable_shadow_bees = BooleanVar(self.app, True)
        self.enable_mushroom_shuffle = BooleanVar(self.app, True)

        self.create_window()
        self.create_widgets()
        self.configure()
        self.perform_layout()

        set_appearance_mode("System")
        set_default_color_theme("dark-blue")

    def create_window(self) -> None:
        # create root window
        self.app.title("Spritzer ROM Adjuster")
        self.app.eval('tk::PlaceWindow . center')
        self.app.minsize(_WINDOW[0], _WINDOW[1])
        self.app.maxsize(_WINDOW[0], _WINDOW[1])

    def create_widgets(self) -> None:
        self.option_frame = CTkScrollableFrame(
            master=self.app,
        )
        self.dungeon_enemy_shuffle_label = CTkLabel(
            self.option_frame,
            text="Dungeon Entity Shuffle"
        )
        self.dungeon_enemy_shuffle_combobox = CTkComboBox(
            self.option_frame,
            values=list(DungeonEnemyShuffle),
            variable=self.dungeon_enemy_shuffle,
        )
        self.overworld_enemy_shuffle_label = CTkLabel(
            self.option_frame,
            text="Overworld Entity Shuffle"
        )
        self.overworld_enemy_shuffle_combobox = CTkComboBox(
            self.option_frame,
            values=list(OverworldEnemyShuffle),
            variable=self.overworld_enemy_shuffle,
        )
        self.enable_boss_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Boss Shuffle",
            variable=self.enable_boss_shuffle,
        )
        self.dungeon_palette_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Dungeon Palette Shuffle",
            variable=self.enable_dungeon_palette_shuffle,
        )
        self.dungeon_tileset_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Dungeon Tileset Shuffle",
            variable=self.enable_dungeon_tileset_shuffle,
        )
        self.killable_thieves_checkbox = CTkCheckBox(
            self.option_frame,
            text="Killable Thieves",
            variable=self.enable_killable_thieves,
        )
        self.mushroom_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Mushroom Shuffle",
            variable=self.enable_mushroom_shuffle,
        )
        self.shadow_bees_checkbox = CTkCheckBox(
            self.option_frame,
            text="Shadow Bees",
            variable=self.enable_shadow_bees,
        )
        self.footer = CTkFrame(
            master=self.app,
        )
        self.seed_label = CTkLabel(
            self.footer,
            text="Seed (optional)",
        )
        self.seed_entry = CTkEntry(
            self.footer,
            placeholder_text="Default Seed",
            width=600,
            textvariable=self.seed,
        )
        self.button = CTkButton(
            master=self.footer,
            command=self.patch_rom,
        )

    def configure(self) -> None:
        self.button.configure(text="Patch ROM")

    def perform_layout(self) -> None:
        self.app.rowconfigure(1, weight=1)
        self.option_frame.pack(
            expand=True,
            fill="both",
            padx=_PADX,
            pady=_PADY,
        )
        self.option_frame.pack(
            expand=True,
            fill="both",
            padx=_PADX,
            pady=_PADY,
        )

        self.overworld_enemy_shuffle_label.grid(
            row=0,
            column=0,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.overworld_enemy_shuffle_combobox.grid(
            row=0,
            column=1,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )

        self.dungeon_enemy_shuffle_label.grid(
            row=1,
            column=0,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.dungeon_enemy_shuffle_combobox.grid(
            row=1,
            column=1,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )

        boolean_options = [
            self.enable_boss_shuffle_checkbox,
            self.dungeon_palette_shuffle_checkbox,
            self.dungeon_tileset_shuffle_checkbox,
            self.killable_thieves_checkbox,
            self.mushroom_shuffle_checkbox,
            self.shadow_bees_checkbox,
        ]

        # Overcomplicated code that lays checkboxes into columns.
        column_count = 1
        row_offset = 2
        for index, checkbox in enumerate(boolean_options):
            cell_count = floor(len(boolean_options) / column_count)
            row = row_offset + (index % cell_count)
            column = floor(index / cell_count)
            checkbox.grid(
                row=row,
                column=column,
                sticky="W",
                pady=_PADY,
                padx=_PADX / 2,
            )

        self.footer.pack(
            fill="x",
            padx=_PADX,
            pady=_PADY,
        )

        self.footer.grid_columnconfigure(1, weight=1)
        self.seed_label.grid(
            sticky="W",
            row=0,
            column=0,
            padx=(_PADX, 0),
            pady=_PADY,
        )
        self.seed_entry.grid(
            sticky="EW",
            row=0,
            column=1,
            padx=_PADX,
            pady=_PADY,
        )
        self.button.grid(
            sticky="E",
            row=0,
            column=2,
            padx=(0, _PADX),
            pady=_PADY,
        )

    def patch_rom(self) -> None:
        options = Options()
        options.seed = self.seed.get()
        options.boss_shuffle = self.enable_boss_shuffle.get()
        options.dungeon_palette_shuffle = self.enable_dungeon_palette_shuffle.get()
        options.dungeon_tileset_shuffle = self.enable_dungeon_tileset_shuffle.get()
        options.killable_thieves = self.enable_killable_thieves.get()
        options.mushroom_shuffle = self.enable_mushroom_shuffle.get()
        options.shadow_bees = self.enable_shadow_bees.get()
        options.dungeon_enemy_shuffle = self.dungeon_enemy_shuffle.get()
        options.overworld_enemy_shuffle = self.overworld_enemy_shuffle.get()

        input_path = filedialog.askopenfilename(filetypes=[("Zelda3 JPN", "*.sfc")])
        if not input_path:
            return

        patch_file(
            options=options,
            input_path=input_path,
            output_path=input_path,
        )
        message = messagebox.Message(
            self.app, message="ROM Patched -- Have a wonderful journey!"
        )
        message.show()

    def start(self) -> None:
        self.app.mainloop()
