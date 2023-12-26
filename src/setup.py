from cx_Freeze import setup, Executable
import sys


# Dependencies are automatically detected, but it might need
# fine tuning.
base = "Win32GUI" if sys.platform == "win32" else None
build_options = {
    "packages": [],
    "excludes": [],
    "build_exe": "..\dist",
}

executables = [Executable("main.py", base=base, target_name="spritzer-ui")]

setup(
    name="Spritzer",
    version="0.1.15",
    description="Zelda3 Randomizer",
    options={"build_exe": build_options},
    executables=executables,
)
