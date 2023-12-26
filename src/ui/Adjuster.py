from tkinter import filedialog, messagebox
import customtkinter
from library.Spritzer import patch_rom


class Adjuster:
    def __init__(self) -> None:
        self.app = customtkinter.CTk()
        self.create_window()
        self.create_widgets()
        self.perform_layout()

    def create_window(self) -> None:
        # create root window
        customtkinter.set_appearance_mode("dark")
        customtkinter.set_default_color_theme("blue")

        self.app.title("Spritzer ROM Adjuster")
        self.app.geometry("500x350")
        self.app.minsize(500, 350)

    def create_widgets(self) -> None:
        self.button = customtkinter.CTkButton(
            master=self.app,
            text="Patch ROM",
            command=self.patch_rom,
        )

    def perform_layout(self) -> None:
        self.button.place(
            relx=0.5,
            rely=0.5,
            anchor=customtkinter.CENTER,
        )

    def patch_rom(self) -> None:
        input_path = filedialog.askopenfilename(filetypes=[("Zelda3 JPN", "*.sfc")])
        output_path = input_path.replace(".sfc", "-randomized.sfc")
        patch_rom(
            input_path=input_path,
            output_path=output_path,
        )
        message = messagebox.Message(self.app, message="Great success!")
        message.show()

    def start(self) -> None:
        self.app.mainloop()

