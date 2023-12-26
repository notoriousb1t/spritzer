from tkinter import filedialog, messagebox, BooleanVar
from customtkinter import (
    CTk,
    set_appearance_mode,
    set_default_color_theme,
    CENTER,
    CTkButton,
    CTkFrame,
    CTkCheckBox,
)
from library.Spritzer import patch_rom

_PADX = 12
_PADY = 8


class Adjuster:
    def __init__(self) -> None:
        self.app = CTk()

        self.enable_dungeon_palette_shuffle = BooleanVar(self.app, True)
        self.enable_dungeon_tileset_shuffle = BooleanVar(self.app, True)
        self.enable_killable_thieves = BooleanVar(self.app, True)
        self.enable_shadow_bees = BooleanVar(self.app, True)
        self.enable_mushroom_shuffle = BooleanVar(self.app, True)
        self.enable_sprite_shuffle = BooleanVar(self.app, True)

        self.create_window()
        self.create_widgets()
        self.configure()
        self.perform_layout()

        set_appearance_mode("System")
        set_default_color_theme("dark-blue")

    def create_window(self) -> None:
        # create root window
        self.app.title("Spritzer ROM Adjuster")
        self.app.geometry("500x350")
        self.app.minsize(500, 350)
        self.app.maxsize(500, 350)

    def create_widgets(self) -> None:
        self.main = CTkFrame(
            master=self.app,
        )
        self.dungeon_palette_shuffle_checkbox = CTkCheckBox(
            self.main,
            text="Dungeon Palette Shuffle",
            variable=self.enable_dungeon_palette_shuffle,
        )
        self.dungeon_tileset_shuffle_checkbox = CTkCheckBox(
            self.main,
            text="Dungeon Tileset Shuffle",
            variable=self.enable_dungeon_tileset_shuffle,
        )
        self.killable_thieves_checkbox = CTkCheckBox(
            self.main,
            text="Killable Thieves",
            variable=self.enable_killable_thieves,
        )
        self.mushroom_shuffle_checkbox = CTkCheckBox(
            self.main,
            text="Mushroom Shuffle",
            variable=self.enable_mushroom_shuffle,
        )
        self.shadow_bees_checkbox = CTkCheckBox(
            self.main,
            text="Shadow Bees",
            variable=self.enable_shadow_bees,
        )
        self.sprite_shuffle_checkbox = CTkCheckBox(
            self.main,
            text="Sprite Shuffle",
            variable=self.enable_sprite_shuffle,
        )
        self.footer = CTkFrame(
            master=self.app,
        )
        self.button = CTkButton(
            master=self.footer,
            command=self.patch_rom,
        )

    def configure(self) -> None:
        self.button.configure(text="Patch ROM")

    def perform_layout(self) -> None:
        self.app.rowconfigure(1, weight=1)
        self.main.pack(
            expand=True,
            fill="both",
            padx=_PADX,
            pady=_PADY,
        )

        boolean_options = [
            self.dungeon_palette_shuffle_checkbox,
            self.dungeon_tileset_shuffle_checkbox,
            self.killable_thieves_checkbox,
            self.mushroom_shuffle_checkbox,
            self.shadow_bees_checkbox,
            self.sprite_shuffle_checkbox,
        ]
        for index, checkbox in enumerate(boolean_options):
            checkbox.grid(row=index, column=1, sticky="W", pady=_PADY, padx=_PADX/2)

        self.footer.pack(
            fill="x",
            padx=_PADX,
            pady=_PADY,
        )
        self.button.pack(side="right")

    def patch_rom(self) -> None:
        input_path = filedialog.askopenfilename(filetypes=[("Zelda3 JPN", "*.sfc")])
        patch_rom(
            input_path=input_path,
            output_path=input_path,
            dungeon_palette_shuffle=self.enable_dungeon_palette_shuffle.get(),
            dungeon_tileset_shuffle=self.enable_dungeon_tileset_shuffle.get(),
            killable_thieves=self.enable_killable_thieves.get(),
            mushroom_shuffle=self.enable_mushroom_shuffle.get(),
            shadow_bees=self.enable_shadow_bees.get(),
            sprite_shuffle=self.enable_sprite_shuffle.get(),
        )
        message = messagebox.Message(self.app, message="Have a wonderful journey!")
        message.show()

    def start(self) -> None:
        self.app.mainloop()
