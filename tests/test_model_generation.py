"""Unit tests for model-based OPNsense configuration generation."""

from __future__ import annotations

from pathlib import Path
from unittest.mock import Mock

import pytest

from opnsense.factories.interface_factory import InterfaceFactory, InterfaceModel, InterfacesModel
from opnsense.generators.interface_generator import generate_interface_xml_with_models


class TestInterfaceModel:
    """Test InterfaceModel functionality."""

    def test_interface_model_creation(self) -> None:
        """Test creating an InterfaceModel with valid data."""
        interface = InterfaceModel(
            if_field="ix0", descr="WAN", enable=True, ipaddr="11.22.33.44", subnet=29, gateway="WAN_GWv4"
        )
        assert interface.if_field == "ix0"
        assert interface.descr == "WAN"
        assert interface.enable is True
        assert interface.ipaddr == "11.22.33.44"
        assert interface.subnet == 29
        assert interface.gateway == "WAN_GWv4"

    def test_interface_model_to_xml_element(self) -> None:
        """Test converting InterfaceModel to XML element."""
        interface = InterfaceModel(if_field="lagg0", descr="LAN", enable=True, ipaddr="10.1.1.11", subnet=24)
        elem = interface.to_xml_element("lan")

        assert elem.tag == "lan"
        assert elem.find("if").text == "lagg0"
        assert elem.find("descr").text == "LAN"
        assert elem.find("enable").text == "1"
        assert elem.find("ipaddr").text == "10.1.1.11"
        assert elem.find("subnet").text == "24"

    def test_interface_model_to_xml_string(self) -> None:
        """Test converting InterfaceModel to XML string."""
        interface = InterfaceModel(
            if_field="vlan0100", descr="V100_Test", enable=True, ipaddr="192.168.100.1", subnet=24
        )
        xml_str = interface.to_xml("opt1")
        expected = "<opt1><if>vlan0100</if><descr>V100_Test</descr><enable>1</enable><spoofmac /><ipaddr>192.168.100.1</ipaddr><subnet>24</subnet></opt1>"
        assert xml_str == expected


class TestInterfacesModel:
    """Test InterfacesModel functionality."""

    def test_interfaces_model_creation(self) -> None:
        """Test creating an InterfacesModel with multiple interfaces."""
        wan = InterfaceModel(if_field="ix0", descr="WAN", enable=True, ipaddr="11.22.33.44", subnet=29)
        lan = InterfaceModel(if_field="lagg0", descr="LAN", enable=True, ipaddr="10.1.1.11", subnet=24)

        interfaces = InterfacesModel(interfaces={"wan": wan, "lan": lan})

        assert len(interfaces.interfaces) == 2
        assert "wan" in interfaces.interfaces
        assert "lan" in interfaces.interfaces

    def test_interfaces_model_to_xml(self) -> None:
        """Test converting InterfacesModel to XML string."""
        wan = InterfaceModel(if_field="ix0", descr="WAN", enable=True, ipaddr="11.22.33.44", subnet=29)
        lan = InterfaceModel(if_field="lagg0", descr="LAN", enable=True, ipaddr="10.1.1.11", subnet=24)

        interfaces = InterfacesModel(interfaces={"wan": wan, "lan": lan})

        xml_str = interfaces.to_xml()
        assert "<wan>" in xml_str
        assert "<lan>" in xml_str
        assert "ix0" in xml_str
        assert "lagg0" in xml_str


class TestInterfaceFactory:
    """Test InterfaceFactory functionality."""

    def test_interface_factory_creation(self) -> None:
        """Test creating an InterfaceFactory."""
        factory = InterfaceFactory()
        assert isinstance(factory, InterfaceFactory)

    def test_escape_xml_string(self) -> None:
        """Test XML string escaping functionality."""
        factory = InterfaceFactory()

        # Test basic replacements
        assert factory._escape_xml_string("Test-Name") == "Test_Name"
        assert factory._escape_xml_string("Test Name") == "TestName"
        assert factory._escape_xml_string("Test/Name") == "Test_Name"

        # Test German umlauts
        assert factory._escape_xml_string("München") == "Muenchen"
        assert factory._escape_xml_string("Köln") == "Koeln"
        assert factory._escape_xml_string("Straße") == "Strasse"

    def test_create_from_vlan_configs_empty(self) -> None:
        """Test creating interfaces from empty VLAN configs."""
        factory = InterfaceFactory()
        configs = []
        options = {"opt_counter": 1, "firewallNr": 1}

        result = factory.create_from_vlan_configs(configs, options)

        assert isinstance(result, InterfacesModel)
        assert len(result.interfaces) == 2  # wan and lan always present
        assert "wan" in result.interfaces
        assert "lan" in result.interfaces

    def test_create_from_vlan_configs_with_vlans(self) -> None:
        """Test creating interfaces from VLAN configs."""
        factory = InterfaceFactory()

        # Mock VLAN config objects
        config1 = Mock()
        config1.vlan_id = 100
        config1.description = "Test VLAN 1"
        config1.ip_network = "192.168.100.x"

        config2 = Mock()
        config2.vlan_id = 200
        config2.description = "Test VLAN 2"
        config2.ip_network = "192.168.200.x"

        configs = [config1, config2]
        options = {"opt_counter": 1, "firewallNr": 1}

        result = factory.create_from_vlan_configs(configs, options)

        assert isinstance(result, InterfacesModel)
        assert len(result.interfaces) == 4  # wan, lan, opt1, opt2

        # Check WAN interface
        wan = result.interfaces["wan"]
        assert wan.if_field == "ix0"
        assert wan.descr == "WAN"
        assert wan.ipaddr == "11.22.33.44"
        assert wan.subnet == 29

        # Check LAN interface
        lan = result.interfaces["lan"]
        assert lan.if_field == "lagg0"
        assert lan.descr == "LAN"
        assert lan.ipaddr == "10.1.1.11"
        assert lan.subnet == 24

        # Check VLAN interfaces
        opt1 = result.interfaces["opt1"]
        assert opt1.if_field == "vlan0100"
        assert opt1.descr == "V100_TestVLAN1"  # Fixed: spaces removed by _escape_xml_string
        assert opt1.ipaddr == "192.168.100.251"  # 250 + firewallNr(1)

        opt2 = result.interfaces["opt2"]
        assert opt2.if_field == "vlan0200"
        assert opt2.descr == "V200_TestVLAN2"  # Fixed: spaces removed by _escape_xml_string
        assert opt2.ipaddr == "192.168.200.251"

    def test_create_from_vlan_configs_with_options(self) -> None:
        """Test creating interfaces with different options."""
        factory = InterfaceFactory()

        config = Mock()
        config.vlan_id = 100
        config.description = "Test"
        config.ip_network = "192.168.100.x"

        configs = [config]
        options = {"opt_counter": 5, "firewallNr": 3}  # Different values

        result = factory.create_from_vlan_configs(configs, options)

        # Check that opt5 is created (opt_counter = 5)
        opt5 = result.interfaces["opt5"]
        assert opt5.if_field == "vlan0100"
        assert opt5.ipaddr == "192.168.100.253"  # 250 + firewallNr(3)


class TestInterfaceGenerator:
    """Test interface generator functionality."""

    def test_generate_interface_xml_with_models(self, tmp_path: Path) -> None:
        """Test the complete interface XML generation process."""
        output_file = tmp_path / "interfaces.xml"

        # Mock VLAN config
        config = Mock()
        config.vlan_id = 100
        config.description = "Test VLAN"
        config.ip_network = "192.168.100.x"

        configs = [config]
        options = {"opt_counter": 1, "firewallNr": 1}

        generate_interface_xml_with_models(configs, output_file, options)

        assert output_file.exists()

        # Read and verify XML content
        xml_content = output_file.read_text()
        assert "<wan>" in xml_content
        assert "<lan>" in xml_content
        assert "<opt1>" in xml_content
        assert "vlan0100" in xml_content
        assert "192.168.100.251" in xml_content

    def test_generate_interface_xml_empty_configs(self, tmp_path: Path) -> None:
        """Test generating XML with empty VLAN configs."""
        output_file = tmp_path / "empty_interfaces.xml"

        configs = []
        options = {"opt_counter": 1, "firewallNr": 1}

        generate_interface_xml_with_models(configs, output_file, options)

        assert output_file.exists()

        xml_content = output_file.read_text()
        assert "<wan>" in xml_content
        assert "<lan>" in xml_content
        assert "<opt1>" not in xml_content  # No VLAN configs

    def test_generate_interface_xml_multiple_vlans(self, tmp_path: Path) -> None:
        """Test generating XML with multiple VLAN configs."""
        output_file = tmp_path / "multiple_interfaces.xml"

        # Create multiple mock configs
        configs = []
        for i in range(3):
            config = Mock()
            config.vlan_id = 100 + i
            config.description = f"VLAN {i + 1}"
            config.ip_network = f"192.168.{100 + i}.x"
            configs.append(config)

        options = {"opt_counter": 1, "firewallNr": 1}

        generate_interface_xml_with_models(configs, output_file, options)

        assert output_file.exists()

        xml_content = output_file.read_text()
        assert "<opt1>" in xml_content
        assert "<opt2>" in xml_content
        assert "<opt3>" in xml_content
        assert "vlan0100" in xml_content
        assert "vlan0101" in xml_content
        assert "vlan0102" in xml_content


class TestModelValidation:
    """Test model validation and error handling."""

    def test_interface_model_validation(self) -> None:
        """Test that InterfaceModel validates required fields."""
        # Should work with required fields
        interface = InterfaceModel(if_field="ix0", descr="WAN", ipaddr="11.22.33.44")
        assert interface.if_field == "ix0"

        # Should have defaults
        assert interface.enable is True
        assert interface.subnet == 24
        assert interface.spoofmac == ""

    def test_interfaces_model_default_factory(self) -> None:
        """Test that InterfacesModel uses default factory for interfaces dict."""
        interfaces = InterfacesModel()
        assert isinstance(interfaces.interfaces, dict)
        assert len(interfaces.interfaces) == 0

    def test_factory_with_invalid_options(self) -> None:
        """Test factory behavior with invalid options."""
        factory = InterfaceFactory()
        configs = []

        # Test with missing options (should use defaults)
        result = factory.create_from_vlan_configs(configs, {})
        assert isinstance(result, InterfacesModel)

        # Test with None options - should handle gracefully
        with pytest.raises(AttributeError):
            factory.create_from_vlan_configs(configs, None)  # type: ignore
