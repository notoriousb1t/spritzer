import sys
from cx_Freeze import setup, Executable


# Dependencies are automatically detected, but it might need
# fine tuning.
base = "Win32GUI" if sys.platform == "win32" else None
build_options = {
    "packages": [],
    "excludes": [],
    "build_exe": "../build",
}

executables = [Executable("main.py", base=base, target_name="spritzer-ui")]

setup(
    name="spritzer",
    version="0.1.23",
    description="Zelda3 Randomizer",
    options={"build_exe": build_options},
    executables=executables,
)
