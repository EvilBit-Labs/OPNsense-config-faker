"""Model-based interface generator."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from opnsense.factories.interface_factory import InterfaceFactory


def generate_interface_xml_with_models(configs: list[Any], output_file: Path, options: dict[str, Any]) -> None:
    factory = InterfaceFactory()
    interface_models = factory.create_from_vlan_configs(configs, options)
    xml_content = interface_models.to_xml()
    output_file.write_text(xml_content)
