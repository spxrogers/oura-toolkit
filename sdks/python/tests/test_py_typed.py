"""Tripwire: the PEP 561 ``py.typed`` marker must SHIP in the INSTALLED package (ISSUE C).

setuptools drops marker files from the built wheel unless they are listed in
``[tool.setuptools.package-data]``; a marker that exists only in the source tree does
nothing for downstream type-checkers. ``just sdk-test-py`` pip-installs the distribution
into a fresh venv and runs pytest against ``sdks/python/tests`` (which is NOT on the import
path), so ``import oura_toolkit.*`` here resolves to the INSTALLED copy — this asserts the
marker survived the wheel, not that it exists in git.
"""

from __future__ import annotations

import importlib
from pathlib import Path

import pytest


@pytest.mark.parametrize("module_name", ["oura_toolkit.api", "oura_toolkit.auth"])
def test_py_typed_marker_ships_with_the_installed_package(module_name: str) -> None:
    module = importlib.import_module(module_name)
    assert module.__file__ is not None, f"{module_name} has no __file__"
    marker = Path(module.__file__).parent / "py.typed"
    assert marker.is_file(), (
        f"{module_name} ships no py.typed marker at {marker} — setuptools dropped it "
        "from the wheel (add the package to [tool.setuptools.package-data]); without it "
        "type-checkers silently ignore the package's annotations"
    )
