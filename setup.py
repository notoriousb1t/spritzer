# pyright: reportMissingTypeStubs=false, reportUnknownVariableType=false, reportUnknownArgumentType=false
import sys
from cx_Freeze import setup, Executable


# Dependencies are automatically detected, but it might need
# fine tuning.
base = "Win32GUI" if sys.platform == "win32" else None

executables: list[Executable] = [
    Executable(
        script="./src/main.py",
        base=base,
        target_name="spritzer-ui",
    )
]

setup(
    name="spritzer",
    version="0.1.30",
    description="Zelda3 Randomizer",
    options={
        "build_exe": {
            "packages": [],
            "excludes": [],
            "build_exe": "./build",
        }
    },
    executables=executables,
)
