import xml.etree.ElementTree as ET
from abc import ABC, abstractmethod
from typing import Any

from pydantic import BaseModel


class BaseOPNsenseModel(BaseModel):
    class Config:
        extra = "forbid"
        validate_assignment = True

    def to_xml_element(self, tag_name: str) -> ET.Element:
        elem = ET.Element(tag_name)
        for field_name, field_value in self.model_dump().items():
            if field_value is not None and field_value != "":
                if isinstance(field_value, bool):
                    ET.SubElement(elem, field_name).text = "1" if field_value else "0"
                else:
                    ET.SubElement(elem, field_name).text = str(field_value)
        return elem

    def to_xml(self, tag_name: str) -> str:
        elem = self.to_xml_element(tag_name)
        return ET.tostring(elem, encoding="unicode")


class BaseModelFactory(ABC):
    def _escape_xml_string(self, text: str) -> str:
        replacements = {
            "ä": "ae",
            "ö": "oe",
            "ü": "ue",
            "ß": "ss",
            "Ä": "AE",
            "Ö": "OE",
            "Ü": "UE",
            " ": "",
            "-": "_",
            "/": "_",
        }
        for old, new in replacements.items():
            text = text.replace(old, new)
        return text

    @abstractmethod
    def create_from_vlan_configs(self, configs: list[Any], options: dict[str, Any]) -> Any:
        pass
