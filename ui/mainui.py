# pyright: reportMissingTypeStubs=false, reportUnknownMemberType=false
from math import floor
from string import ascii_uppercase, digits
from random import Random

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
from tkinter import filedialog, messagebox, BooleanVar, StringVar

from library.model.balancing import Balancing
from library.options.options import Options
from library.options.overworld_enemy_shuffle import OverworldEnemyShuffle
from library.options.underworld_enemy_shuffle import UnderworldEnemyShuffle
from library.patcher import patch_file


_PADX = 12
_PADY = 8
_WINDOW: tuple[int, int] = (600, 400)


class MainUi:
    def __init__(self) -> None:
        self.app = CTk()

        # Generate a random string for the seed.
        random_seed: str = "".join(
            Random().choices(
                population=ascii_uppercase + digits,
                k=32,
            )
        )

        self.seed = StringVar(master=self.app, value=random_seed)
        self.overworld_enemy_balancing = StringVar(
            master=self.app, value=Balancing.BALANCED
        )
        self.overworld_enemy_shuffle = StringVar(
            master=self.app, value=OverworldEnemyShuffle.SIMPLE
        )
        self.underworld_balancing = StringVar(master=self.app, value=Balancing.BALANCED)
        self.underworld_enemy_shuffle = StringVar(
            master=self.app, value=UnderworldEnemyShuffle.SIMPLE
        )
        self.underworld_palette_shuffle = BooleanVar(master=self.app, value=False)
        self.underworld_tileset_shuffle = BooleanVar(master=self.app, value=False)
        self.shadow_bees = BooleanVar(master=self.app, value=False)
        self.overlord_shuffle = BooleanVar(master=self.app, value=False)

        self.boss_shuffle = BooleanVar(master=self.app, value=False)
        self.killable_thieves = BooleanVar(master=self.app, value=False)
        self.mushroom_shuffle = BooleanVar(master=self.app, value=False)

        self.create_window()
        self.create_widgets()
        self.configure()
        self.perform_layout()

        set_appearance_mode(mode_string="System")
        set_default_color_theme(color_string="dark-blue")

    def create_window(self) -> None:
        # create root window
        self.app.title(string="Spritzer ROM Adjuster")
        self.app.eval("tk::PlaceWindow . center")
        self.app.minsize(width=_WINDOW[0], height=_WINDOW[1])
        self.app.maxsize(width=_WINDOW[0], height=_WINDOW[1])

    def create_widgets(self) -> None:
        self.option_frame = CTkScrollableFrame(
            master=self.app,
        )
        self.underworld_balancing_label = CTkLabel(
            master=self.option_frame, text="Underworld Balancing"
        )
        self.underworld_balancing_combobox = CTkComboBox(
            master=self.option_frame,
            values=list(Balancing),
            variable=self.underworld_balancing,
        )
        self.underworld_enemy_shuffle_label = CTkLabel(
            master=self.option_frame, text="Underworld Enemy Shuffle"
        )
        self.underworld_enemy_shuffle_combobox = CTkComboBox(
            master=self.option_frame,
            values=list(UnderworldEnemyShuffle),
            variable=self.underworld_enemy_shuffle,
        )
        self.overworld_balancing_label = CTkLabel(
            master=self.option_frame, text="Overworld Balancing"
        )
        self.overworld_balancing_combobox = CTkComboBox(
            master=self.option_frame,
            values=list(Balancing),
            variable=self.overworld_enemy_balancing,
        )
        self.overworld_enemy_shuffle_label = CTkLabel(
            master=self.option_frame, text="Overworld Enemy Shuffle"
        )
        self.overworld_enemy_shuffle_combobox = CTkComboBox(
            master=self.option_frame,
            values=list(OverworldEnemyShuffle),
            variable=self.overworld_enemy_shuffle,
        )
        self.enable_boss_shuffle_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Boss Shuffle (WIP)",
            variable=self.boss_shuffle,
        )
        self.underworld_palette_shuffle_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Underworld Palette Shuffle",
            variable=self.underworld_palette_shuffle,
        )
        self.underworld_tileset_shuffle_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Underworld Tileset Shuffle (WIP)",
            variable=self.underworld_tileset_shuffle,
        )
        self.killable_thieves_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Killable Thieves (WIP)",
            variable=self.killable_thieves,
        )
        self.mushroom_shuffle_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Mushroom Shuffle",
            variable=self.mushroom_shuffle,
        )
        self.shadow_bees_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Shadow Bees",
            variable=self.shadow_bees,
        )
        self.overlord_shuffle_checkbox = CTkCheckBox(
            master=self.option_frame,
            text="Overlord Shuffle (WIP)",
            variable=self.overlord_shuffle
        )
        self.footer = CTkFrame(
            master=self.app,
        )
        self.seed_label = CTkLabel(
            master=self.footer,
            text="Seed (optional)",
        )
        self.seed_entry = CTkEntry(
            master=self.footer,
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
        self.app.rowconfigure(index=1, weight=1)
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

        self.overworld_balancing_label.grid(
            row=0,
            column=0,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.overworld_balancing_combobox.grid(
            row=0,
            column=1,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.overworld_enemy_shuffle_label.grid(
            row=1,
            column=0,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.overworld_enemy_shuffle_combobox.grid(
            row=1,
            column=1,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.underworld_balancing_label.grid(
            row=2,
            column=0,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.underworld_balancing_combobox.grid(
            row=2,
            column=1,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.underworld_enemy_shuffle_label.grid(
            row=3,
            column=0,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )
        self.underworld_enemy_shuffle_combobox.grid(
            row=3,
            column=1,
            sticky="W",
            pady=_PADY,
            padx=_PADX,
        )

        boolean_options: list[CTkCheckBox] = [
            self.underworld_palette_shuffle_checkbox,
            self.mushroom_shuffle_checkbox,
            self.shadow_bees_checkbox,
            self.enable_boss_shuffle_checkbox,
            self.underworld_tileset_shuffle_checkbox,
            self.killable_thieves_checkbox,
            self.overlord_shuffle_checkbox,
        ]

        # Overcomplicated code that lays checkboxes into columns.
        column_count = 1
        row_offset = 4
        for index, checkbox in enumerate(iterable=boolean_options):
            cell_count: int = floor(len(boolean_options) / column_count)
            row: int = row_offset + (index % cell_count)
            column: int = floor(index / cell_count)
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

        self.footer.grid_columnconfigure(index=1, weight=1)
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
        options = Options(seed=self.seed.get())
        options.boss_shuffle = self.boss_shuffle.get()
        options.underworld_palette_shuffle = self.underworld_palette_shuffle.get()
        options.underworld_tileset_shuffle = self.underworld_tileset_shuffle.get()
        options.killable_thieves = self.killable_thieves.get()
        options.mushroom_shuffle = self.mushroom_shuffle.get()
        options.overlord_shuffle = self.overlord_shuffle.get()
        options.shadow_bees = self.shadow_bees.get()
        options.underworld_balancing = Balancing(value=self.underworld_balancing.get())
        options.underworld_enemy_shuffle = UnderworldEnemyShuffle(
            value=self.underworld_enemy_shuffle.get()
        )
        options.overworld_enemy_shuffle = OverworldEnemyShuffle(
            value=self.overworld_enemy_shuffle.get()
        )
        options.overworld_balancing = Balancing(
            value=self.overworld_enemy_balancing.get()
        )

        input_path: str = filedialog.askopenfilename(
            filetypes=[("Zelda3 JPN", "*.sfc")]
        )
        if not input_path:
            return

        patch_file(
            options=options,
            input_path=input_path,
            output_path=input_path,
        )
        message = messagebox.Message(
            master=self.app,
            message="ROM Patched -- Have a wonderful journey!",
        )
        message.show()

    def start(self) -> None:
        self.app.mainloop()
