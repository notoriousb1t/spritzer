from random import Random
from library.Spritzer import Spritzer
from library.Rom.FileIo import FileIo

if __name__ == "__main__":
    import tkinter as tk
    from tkinter import filedialog

    root = tk.Tk()
    root.withdraw()
    print("Prompting user for their ROM file")
    input_file = filedialog.askopenfilename(filetypes=[("Zelda3", "*.sfc")])
    output_path = input_file.replace(".sfc", "-randomized.sfc")

    local_file = FileIo(input_file)

    spritzer = Spritzer(Random())
    print("Loading from file")

    spritzer.load(local_file.read_byte)
    print("Loaded file")

    print("Enabling Killable Thieves")
    # spritzer.add_killable_thieves()

    print("Enable dungeon palette swap")
    spritzer.add_dungeon_palette_swap()

    print("Enable tileset palette swap")
    # spritzer.add_tileset_swap()

    print("Enabling Shadow Bees")
    # spritzer.add_shadow_bees()

    print("Enabling Mushroom Shuffle")
    spritzer.add_mushroom_shuffle()

    print("Enabling Sprite Shuffle: Simple")
    spritzer.add_sprite_shuffle_simple()

    print("Saving to file")
    spritzer.save(local_file.write_byte)

    local_file.write_to_file(output_path)
    print("Saved to file")
    print("---------------------------------------------------")
    print()
    print(f"Your random adventure begins! {output_path}")
    print()
    print("---------------------------------------------------")
    input("Press Enter to close this window...")
