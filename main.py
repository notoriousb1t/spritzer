
from random import Random
from library.Spritzer import Spritzer
from library.Rom.LocalRom import LocalRom

if __name__ == "__main__":
    import tkinter as tk
    from tkinter import filedialog
    
    root = tk.Tk()
    root.withdraw()

    input_file = filedialog.askopenfilename(filetypes = [('Zelda3', '*.sfc')])
    output_path = input_file.replace('.sfc', '-randomized.sfc')

    local_rom = LocalRom(input_file)
    spritzer = Spritzer(Random())
    spritzer.load(local_rom.read_byte)
    
    spritzer.enable_killable_thieves()
    spritzer.enable_shadow_bees()
    spritzer.enable_sprite_shuffle_simple()
    spritzer.save(local_rom.write_byte)
    
    local_rom.write_to_file(output_path)
    print(f'Your random adventure begins! {output_path}')