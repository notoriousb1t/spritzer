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
)
from tkinter import filedialog, messagebox, BooleanVar, StringVar
from library.Patch import patch_file, Options

_PADX = 12
_PADY = 8
_WINDOW = (600, 400)


class Adjuster:
    def __init__(self) -> None:
        self.app = CTk()

        self.seed = StringVar(self.app)
        self.enable_boss_shuffle = BooleanVar(self.app, True)
        self.enable_dungeon_palette_shuffle = BooleanVar(self.app, True)
        self.enable_dungeon_sprite_shuffle = BooleanVar(self.app, True)
        self.enable_dungeon_tileset_shuffle = BooleanVar(self.app, True)
        self.enable_killable_thieves = BooleanVar(self.app, True)
        self.enable_shadow_bees = BooleanVar(self.app, True)
        self.enable_mushroom_shuffle = BooleanVar(self.app, True)
        self.enable_overworld_sprite_shuffle = BooleanVar(self.app, True)

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
            label_text="Options",
            label_anchor="w",
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
        self.dungeon_sprite_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Dungeon Shuffle",
            variable=self.enable_dungeon_sprite_shuffle,
        )
        self.dungeon_tileset_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Dungeon Tileset Shuffle",
            variable=self.enable_dungeon_tileset_shuffle,
        )
        self.overworld_sprite_shuffle_checkbox = CTkCheckBox(
            self.option_frame,
            text="Overworld Shuffle",
            variable=self.enable_overworld_sprite_shuffle,
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

        boolean_options = [
            self.enable_boss_shuffle_checkbox,
            self.dungeon_palette_shuffle_checkbox,
            self.dungeon_sprite_shuffle_checkbox,
            self.dungeon_tileset_shuffle_checkbox,
            self.killable_thieves_checkbox,
            self.mushroom_shuffle_checkbox,
            self.overworld_sprite_shuffle_checkbox,
            self.shadow_bees_checkbox,
        ]

        # Overcomplicated code that lays checkboxes into 2 columns.
        column_count = 2
        for index, checkbox in enumerate(boolean_options):
            cell_count = floor(len(boolean_options) / column_count)
            row = index % cell_count
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
        options.dungeon_sprite_shuffle = self.enable_dungeon_sprite_shuffle.get()
        options.dungeon_tileset_shuffle = self.enable_dungeon_tileset_shuffle.get()
        options.killable_thieves = self.enable_killable_thieves.get()
        options.mushroom_shuffle = self.enable_mushroom_shuffle.get()
        options.overworld_sprite_shuffle = self.enable_overworld_sprite_shuffle.get()
        options.shadow_bees = self.enable_shadow_bees.get()

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
