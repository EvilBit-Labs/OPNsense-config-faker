from collections.abc import Iterable
from typing import Optional, Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.adv_dhcp6_authentication_statement_algorithm import (
    AdvDhcp6AuthenticationStatementAlgorithm,
)
from opnsense.models.adv_dhcp6_authentication_statement_authname import (
    AdvDhcp6AuthenticationStatementAuthname,
)
from opnsense.models.adv_dhcp6_authentication_statement_protocol import (
    AdvDhcp6AuthenticationStatementProtocol,
)
from opnsense.models.adv_dhcp6_authentication_statement_rdm import (
    AdvDhcp6AuthenticationStatementRdm,
)
from opnsense.models.adv_dhcp6_config_advanced import AdvDhcp6ConfigAdvanced
from opnsense.models.adv_dhcp6_config_file_override import (
    AdvDhcp6ConfigFileOverride,
)
from opnsense.models.adv_dhcp6_config_file_override_path import (
    AdvDhcp6ConfigFileOverridePath,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address import (
    AdvDhcp6IdAssocStatementAddress,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_enable import (
    AdvDhcp6IdAssocStatementAddressEnable,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_id import (
    AdvDhcp6IdAssocStatementAddressId,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_pltime import (
    AdvDhcp6IdAssocStatementAddressPltime,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_vltime import (
    AdvDhcp6IdAssocStatementAddressVltime,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix import (
    AdvDhcp6IdAssocStatementPrefix,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_enable import (
    AdvDhcp6IdAssocStatementPrefixEnable,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_id import (
    AdvDhcp6IdAssocStatementPrefixId,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_pltime import (
    AdvDhcp6IdAssocStatementPrefixPltime,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_vltime import (
    AdvDhcp6IdAssocStatementPrefixVltime,
)
from opnsense.models.adv_dhcp6_interface_statement_information_only_enable import (
    AdvDhcp6InterfaceStatementInformationOnlyEnable,
)
from opnsense.models.adv_dhcp6_interface_statement_request_options import (
    AdvDhcp6InterfaceStatementRequestOptions,
)
from opnsense.models.adv_dhcp6_interface_statement_script import (
    AdvDhcp6InterfaceStatementScript,
)
from opnsense.models.adv_dhcp6_interface_statement_send_options import (
    AdvDhcp6InterfaceStatementSendOptions,
)
from opnsense.models.adv_dhcp6_key_info_statement_expire import (
    AdvDhcp6KeyInfoStatementExpire,
)
from opnsense.models.adv_dhcp6_key_info_statement_keyid import (
    AdvDhcp6KeyInfoStatementKeyid,
)
from opnsense.models.adv_dhcp6_key_info_statement_keyname import (
    AdvDhcp6KeyInfoStatementKeyname,
)
from opnsense.models.adv_dhcp6_key_info_statement_realm import (
    AdvDhcp6KeyInfoStatementRealm,
)
from opnsense.models.adv_dhcp6_key_info_statement_secret import (
    AdvDhcp6KeyInfoStatementSecret,
)
from opnsense.models.adv_dhcp6_prefix_interface_statement_sla_len import (
    AdvDhcp6PrefixInterfaceStatementSlaLen,
)
from opnsense.models.adv_dhcp_config_advanced import AdvDhcpConfigAdvanced
from opnsense.models.adv_dhcp_config_file_override import (
    AdvDhcpConfigFileOverride,
)
from opnsense.models.adv_dhcp_config_file_override_path import (
    AdvDhcpConfigFileOverridePath,
)
from opnsense.models.adv_dhcp_option_modifiers import AdvDhcpOptionModifiers
from opnsense.models.adv_dhcp_pt_backoff_cutoff import AdvDhcpPtBackoffCutoff
from opnsense.models.adv_dhcp_pt_initial_interval import (
    AdvDhcpPtInitialInterval,
)
from opnsense.models.adv_dhcp_pt_reboot import AdvDhcpPtReboot
from opnsense.models.adv_dhcp_pt_retry import AdvDhcpPtRetry
from opnsense.models.adv_dhcp_pt_select_timeout import AdvDhcpPtSelectTimeout
from opnsense.models.adv_dhcp_pt_timeout import AdvDhcpPtTimeout
from opnsense.models.adv_dhcp_pt_values import AdvDhcpPtValues
from opnsense.models.adv_dhcp_request_options import AdvDhcpRequestOptions
from opnsense.models.adv_dhcp_required_options import AdvDhcpRequiredOptions
from opnsense.models.adv_dhcp_send_options import AdvDhcpSendOptions
from opnsense.models.alias_address import AliasAddress
from opnsense.models.alias_subnet import AliasSubnet
from opnsense.models.ddnsdomainalgorithm import Ddnsdomainalgorithm
from opnsense.models.descr import Descr
from opnsense.models.dhcp6_ia_pd_len import Dhcp6IaPdLen
from opnsense.models.dhcphostname import Dhcphostname
from opnsense.models.dhcprejectfrom import Dhcprejectfrom
from opnsense.models.dnsserver import Dnsserver
from opnsense.models.enable import Enable
from opnsense.models.failover_peerip import FailoverPeerip
from opnsense.models.gateway import Gateway
from opnsense.models.gatewayv6 import Gatewayv6
from opnsense.models.if_mod import If
from opnsense.models.ipaddr import Ipaddr
from opnsense.models.ipaddrv6 import Ipaddrv6
from opnsense.models.media import Media
from opnsense.models.mediaopt import Mediaopt
from opnsense.models.ntpserver import Ntpserver
from opnsense.models.numberoptions import Numberoptions
from opnsense.models.range import Range
from opnsense.models.spoofmac import Spoofmac
from opnsense.models.staticmap import Staticmap
from opnsense.models.subnet import Subnet
from opnsense.models.subnetv6 import Subnetv6
from opnsense.models.track6_interface import Track6Interface
from opnsense.models.track6_prefix_id import Track6PrefixId
from opnsense.models.winsserver import Winsserver

__NAMESPACE__ = "https://opnsense.org/config"


class Lan(BaseModel):
    class Meta:
        name = "lan"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    choice: Iterable[
        (
            Descr |
            Dhcphostname |
            Enable |
            FailoverPeerip |
            Gateway |
            Gatewayv6 |
            If |
            Ipaddr |
            Ipaddrv6 |
            Media |
            Mediaopt |
            Spoofmac |
            Subnet |
            Subnetv6 |
            AdvDhcpConfigAdvanced |
            AdvDhcpConfigFileOverride |
            AdvDhcpConfigFileOverridePath |
            AdvDhcpOptionModifiers |
            AdvDhcpPtBackoffCutoff |
            AdvDhcpPtInitialInterval |
            AdvDhcpPtReboot |
            AdvDhcpPtRetry |
            AdvDhcpPtSelectTimeout |
            AdvDhcpPtTimeout |
            AdvDhcpPtValues |
            AdvDhcpRequestOptions |
            AdvDhcpRequiredOptions |
            AdvDhcpSendOptions |
            AliasAddress |
            AliasSubnet |
            Dhcprejectfrom
        )
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "descr",
                    "type": Descr,
                    "namespace": "",
                },
                {
                    "name": "dhcphostname",
                    "type": Dhcphostname,
                    "namespace": "",
                },
                {
                    "name": "enable",
                    "type": Enable,
                    "namespace": "",
                },
                {
                    "name": "failover_peerip",
                    "type": FailoverPeerip,
                    "namespace": "",
                },
                {
                    "name": "gateway",
                    "type": Gateway,
                    "namespace": "",
                },
                {
                    "name": "gatewayv6",
                    "type": Gatewayv6,
                    "namespace": "",
                },
                {
                    "name": "if",
                    "type": If,
                    "namespace": "",
                },
                {
                    "name": "ipaddr",
                    "type": Ipaddr,
                    "namespace": "",
                },
                {
                    "name": "ipaddrv6",
                    "type": Ipaddrv6,
                    "namespace": "",
                },
                {
                    "name": "media",
                    "type": Media,
                    "namespace": "",
                },
                {
                    "name": "mediaopt",
                    "type": Mediaopt,
                    "namespace": "",
                },
                {
                    "name": "spoofmac",
                    "type": Spoofmac,
                    "namespace": "",
                },
                {
                    "name": "subnet",
                    "type": Subnet,
                    "namespace": "",
                },
                {
                    "name": "subnetv6",
                    "type": Subnetv6,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_config_advanced",
                    "type": AdvDhcpConfigAdvanced,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_config_file_override",
                    "type": AdvDhcpConfigFileOverride,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_config_file_override_path",
                    "type": AdvDhcpConfigFileOverridePath,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_option_modifiers",
                    "type": AdvDhcpOptionModifiers,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_backoff_cutoff",
                    "type": AdvDhcpPtBackoffCutoff,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_initial_interval",
                    "type": AdvDhcpPtInitialInterval,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_reboot",
                    "type": AdvDhcpPtReboot,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_retry",
                    "type": AdvDhcpPtRetry,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_select_timeout",
                    "type": AdvDhcpPtSelectTimeout,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_timeout",
                    "type": AdvDhcpPtTimeout,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_pt_values",
                    "type": AdvDhcpPtValues,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_request_options",
                    "type": AdvDhcpRequestOptions,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_required_options",
                    "type": AdvDhcpRequiredOptions,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp_send_options",
                    "type": AdvDhcpSendOptions,
                    "namespace": "",
                },
                {
                    "name": "alias-address",
                    "type": AliasAddress,
                    "namespace": "",
                },
                {
                    "name": "alias-subnet",
                    "type": AliasSubnet,
                    "namespace": "",
                },
                {
                    "name": "dhcprejectfrom",
                    "type": Dhcprejectfrom,
                    "namespace": "",
                },
            ),
        },
    )
    choice_1: Iterable[
        (
            Dhcp6IaPdLen |
            AdvDhcp6InterfaceStatementSendOptions |
            AdvDhcp6InterfaceStatementRequestOptions |
            AdvDhcp6InterfaceStatementInformationOnlyEnable |
            AdvDhcp6InterfaceStatementScript |
            AdvDhcp6IdAssocStatementAddressEnable |
            AdvDhcp6IdAssocStatementAddress |
            AdvDhcp6IdAssocStatementAddressId |
            AdvDhcp6IdAssocStatementAddressPltime |
            AdvDhcp6IdAssocStatementAddressVltime |
            AdvDhcp6IdAssocStatementPrefixEnable |
            AdvDhcp6IdAssocStatementPrefix |
            AdvDhcp6IdAssocStatementPrefixId |
            AdvDhcp6IdAssocStatementPrefixPltime |
            AdvDhcp6IdAssocStatementPrefixVltime |
            AdvDhcp6PrefixInterfaceStatementSlaLen |
            AdvDhcp6AuthenticationStatementAuthname |
            AdvDhcp6AuthenticationStatementProtocol |
            AdvDhcp6AuthenticationStatementAlgorithm |
            AdvDhcp6AuthenticationStatementRdm |
            AdvDhcp6KeyInfoStatementKeyname |
            AdvDhcp6KeyInfoStatementRealm |
            AdvDhcp6KeyInfoStatementKeyid |
            AdvDhcp6KeyInfoStatementSecret |
            AdvDhcp6KeyInfoStatementExpire |
            AdvDhcp6ConfigAdvanced |
            AdvDhcp6ConfigFileOverride |
            AdvDhcp6ConfigFileOverridePath |
            Track6Interface |
            Track6PrefixId
        )
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "dhcp6-ia-pd-len",
                    "type": Dhcp6IaPdLen,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_interface_statement_send_options",
                    "type": AdvDhcp6InterfaceStatementSendOptions,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_interface_statement_request_options",
                    "type": AdvDhcp6InterfaceStatementRequestOptions,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_interface_statement_information_only_enable",
                    "type": AdvDhcp6InterfaceStatementInformationOnlyEnable,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_interface_statement_script",
                    "type": AdvDhcp6InterfaceStatementScript,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_address_enable",
                    "type": AdvDhcp6IdAssocStatementAddressEnable,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_address",
                    "type": AdvDhcp6IdAssocStatementAddress,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_address_id",
                    "type": AdvDhcp6IdAssocStatementAddressId,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_address_pltime",
                    "type": AdvDhcp6IdAssocStatementAddressPltime,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_address_vltime",
                    "type": AdvDhcp6IdAssocStatementAddressVltime,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_prefix_enable",
                    "type": AdvDhcp6IdAssocStatementPrefixEnable,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_prefix",
                    "type": AdvDhcp6IdAssocStatementPrefix,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_prefix_id",
                    "type": AdvDhcp6IdAssocStatementPrefixId,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_prefix_pltime",
                    "type": AdvDhcp6IdAssocStatementPrefixPltime,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_id_assoc_statement_prefix_vltime",
                    "type": AdvDhcp6IdAssocStatementPrefixVltime,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_prefix_interface_statement_sla_len",
                    "type": AdvDhcp6PrefixInterfaceStatementSlaLen,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_authentication_statement_authname",
                    "type": AdvDhcp6AuthenticationStatementAuthname,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_authentication_statement_protocol",
                    "type": AdvDhcp6AuthenticationStatementProtocol,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_authentication_statement_algorithm",
                    "type": AdvDhcp6AuthenticationStatementAlgorithm,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_authentication_statement_rdm",
                    "type": AdvDhcp6AuthenticationStatementRdm,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_key_info_statement_keyname",
                    "type": AdvDhcp6KeyInfoStatementKeyname,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_key_info_statement_realm",
                    "type": AdvDhcp6KeyInfoStatementRealm,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_key_info_statement_keyid",
                    "type": AdvDhcp6KeyInfoStatementKeyid,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_key_info_statement_secret",
                    "type": AdvDhcp6KeyInfoStatementSecret,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_key_info_statement_expire",
                    "type": AdvDhcp6KeyInfoStatementExpire,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_config_advanced",
                    "type": AdvDhcp6ConfigAdvanced,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_config_file_override",
                    "type": AdvDhcp6ConfigFileOverride,
                    "namespace": "",
                },
                {
                    "name": "adv_dhcp6_config_file_override_path",
                    "type": AdvDhcp6ConfigFileOverridePath,
                    "namespace": "",
                },
                {
                    "name": "track6-interface",
                    "type": Track6Interface,
                    "namespace": "",
                },
                {
                    "name": "track6-prefix-id",
                    "type": Track6PrefixId,
                    "namespace": "",
                },
            ),
            "max_occurs": 28,
        },
    )
    ddnsdomainalgorithm: Ddnsdomainalgorithm | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    numberoptions: Numberoptions | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    range: Range | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    winsserver: Winsserver | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    dnsserver: Dnsserver | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    ntpserver: Ntpserver | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    staticmap: Staticmap | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
