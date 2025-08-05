"""Interface factory for model-based generation."""

from __future__ import annotations

import xml.etree.ElementTree as ET
from typing import Any

from pydantic import Field

from opnsense.factories.base import BaseModelFactory, BaseOPNsenseModel


class InterfaceModel(BaseOPNsenseModel):
    if_field: str
    descr: str
    enable: bool = True
    spoofmac: str = ""
    ipaddr: str
    subnet: int = 24
    blockpriv: bool | None = None
    blockbogons: bool | None = None
    gateway: str | None = None

    def to_xml_element(self, tag_name: str) -> ET.Element:
        elem = ET.Element(tag_name)
        if self.if_field:
            ET.SubElement(elem, "if").text = self.if_field
        if self.descr:
            ET.SubElement(elem, "descr").text = self.descr
        if self.enable:
            ET.SubElement(elem, "enable").text = "1"
        if self.spoofmac == "":
            ET.SubElement(elem, "spoofmac")
        if self.ipaddr:
            ET.SubElement(elem, "ipaddr").text = self.ipaddr
        if self.subnet:
            ET.SubElement(elem, "subnet").text = str(self.subnet)
        if self.blockpriv:
            ET.SubElement(elem, "blockpriv").text = "1"
        if self.blockbogons:
            ET.SubElement(elem, "blockbogons").text = "1"
        if self.gateway:
            ET.SubElement(elem, "gateway").text = self.gateway
        return elem


class InterfacesModel(BaseOPNsenseModel):
    interfaces: dict[str, InterfaceModel] = Field(default_factory=dict)

    def to_xml(self, tag_name: str = "interfaces") -> str:
        xml_parts: list[str] = []
        for interface_name, interface in self.interfaces.items():
            elem = interface.to_xml_element(str(interface_name))
            xml_str = ET.tostring(elem, encoding="unicode")
            xml_parts.append(xml_str)
        return "\n".join(xml_parts)


class InterfaceFactory(BaseModelFactory):
    def create_from_vlan_configs(self, configs: list[Any], options: dict[str, Any]) -> InterfacesModel:
        opt_counter = int(options.get("opt_counter", 1))
        firewall_number = int(options.get("firewallNr", 1))
        ip_suffix = 250 + firewall_number
        interfaces_dict: dict[str, InterfaceModel] = {}
        interfaces_dict["wan"] = InterfaceModel(
            if_field="ix0",
            descr="WAN",
            enable=True,
            blockpriv=True,
            blockbogons=True,
            ipaddr="11.22.33.44",
            subnet=29,
            gateway="WAN_GWv4",
        )
        interfaces_dict["lan"] = InterfaceModel(
            if_field="lagg0", enable=True, ipaddr="10.1.1.11", subnet=24, descr="LAN"
        )
        for i, config in enumerate(configs):
            opt_key = f"opt{opt_counter + i}"
            ip_address = config.ip_network.replace(".x", f".{ip_suffix}")
            interfaces_dict[opt_key] = InterfaceModel(
                if_field=f"vlan0{config.vlan_id}",
                descr=f"V{config.vlan_id}_{self._escape_xml_string(config.description)}",
                enable=True,
                spoofmac="",
                ipaddr=ip_address,
                subnet=24,
            )
        return InterfacesModel(interfaces=interfaces_dict)
