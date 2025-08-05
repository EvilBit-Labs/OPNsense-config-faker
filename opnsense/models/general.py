from collections.abc import Iterable
from typing import Optional, Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.active_interface import ActiveInterface
from opnsense.models.alert_logrotate import AlertLogrotate
from opnsense.models.alert_save_logs import AlertSaveLogs
from opnsense.models.cacheflush import Cacheflush
from opnsense.models.cipher_string import CipherString
from opnsense.models.ciphersuites import Ciphersuites
from opnsense.models.default_packet_size import DefaultPacketSize
from opnsense.models.detect import Detect
from opnsense.models.disablevpnrules import Disablevpnrules
from opnsense.models.dns64 import Dns64
from opnsense.models.dns64prefix import Dns64Prefix
from opnsense.models.dnssec import Dnssec
from opnsense.models.enable_config_constraints import EnableConfigConstraints
from opnsense.models.enable_legacy_sect import EnableLegacySect
from opnsense.models.enable_wpad import EnableWpad
from opnsense.models.enabled import Enabled
from opnsense.models.eve_log import EveLog
from opnsense.models.eventqueue_path import EventqueuePath
from opnsense.models.eventqueue_slots import EventqueueSlots
from opnsense.models.fetch_crls import FetchCrls
from opnsense.models.fwrules import Fwrules
from opnsense.models.groups import Groups
from opnsense.models.homenet import Homenet
from opnsense.models.http_host import HttpHost
from opnsense.models.http_port import HttpPort
from opnsense.models.httpd_allow import HttpdAllow
from opnsense.models.httpd_enabled import HttpdEnabled
from opnsense.models.httpd_password import HttpdPassword
from opnsense.models.httpd_port import HttpdPort
from opnsense.models.httpd_username import HttpdUsername
from opnsense.models.install_crls import InstallCrls
from opnsense.models.interfaces_2 import Interfaces2
from opnsense.models.interval import Interval
from opnsense.models.ips import Ips
from opnsense.models.local_zone_type import LocalZoneType
from opnsense.models.log_payload import LogPayload
from opnsense.models.logfile import Logfile
from opnsense.models.loglocal import Loglocal
from opnsense.models.mailserver import Mailserver
from opnsense.models.maxfilesize import Maxfilesize
from opnsense.models.maxpreserve import Maxpreserve
from opnsense.models.min_protocol import MinProtocol
from opnsense.models.min_protocol_dtls import MinProtocolDtls
from opnsense.models.mmonit_register_credentials import (
    MmonitRegisterCredentials,
)
from opnsense.models.mmonit_timeout import MmonitTimeout
from opnsense.models.mmonit_url import MmonitUrl
from opnsense.models.mpmalgo import Mpmalgo
from opnsense.models.noarecords import Noarecords
from opnsense.models.noreglladdr6 import Noreglladdr6
from opnsense.models.noregrecords import Noregrecords
from opnsense.models.outgoing_interface import OutgoingInterface
from opnsense.models.passthrough_networks import PassthroughNetworks
from opnsense.models.password import Password
from opnsense.models.port import Port
from opnsense.models.preferred_oldsa import PreferredOldsa
from opnsense.models.promisc import Promisc
from opnsense.models.regdhcp import Regdhcp
from opnsense.models.regdhcpdomain import Regdhcpdomain
from opnsense.models.regdhcpstatic import Regdhcpstatic
from opnsense.models.ssl import Ssl
from opnsense.models.sslverify import Sslverify
from opnsense.models.sslversion import Sslversion
from opnsense.models.startdelay import Startdelay
from opnsense.models.statefile import Statefile
from opnsense.models.stats import Stats
from opnsense.models.store_intermediate_certs import StoreIntermediateCerts
from opnsense.models.syslog_2 import Syslog2
from opnsense.models.syslog_eve import SyslogEve
from opnsense.models.txtsupport import Txtsupport
from opnsense.models.update_cron import UpdateCron
from opnsense.models.username import Username
from opnsense.models.valid_lifetime import ValidLifetime
from opnsense.models.verbosity import Verbosity

__NAMESPACE__ = "https://opnsense.org/config"


class General(BaseModel):
    class Meta:
        name = "general"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    store_intermediate_certs: StoreIntermediateCerts | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    install_crls: InstallCrls | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    fetch_crls: FetchCrls | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    enable_legacy_sect: EnableLegacySect | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    enable_config_constraints: EnableConfigConstraints | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    cipher_string: CipherString | None = field(
        default=None,
        metadata={
            "name": "CipherString",
            "type": "Element",
            "namespace": "",
        },
    )
    ciphersuites: Ciphersuites | None = field(
        default=None,
        metadata={
            "name": "Ciphersuites",
            "type": "Element",
            "namespace": "",
        },
    )
    groups: Groups | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    min_protocol: MinProtocol | None = field(
        default=None,
        metadata={
            "name": "MinProtocol",
            "type": "Element",
            "namespace": "",
        },
    )
    min_protocol_dtls: MinProtocolDtls | None = field(
        default=None,
        metadata={
            "name": "MinProtocol_DTLS",
            "type": "Element",
            "namespace": "",
        },
    )
    enabled: Enabled | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    http_host: HttpHost | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    http_port: HttpPort | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    ips: Ips | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    promisc: Promisc | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    interfaces: Interfaces2 | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    choice: Iterable[
        (
            PreferredOldsa |
            Disablevpnrules |
            PassthroughNetworks |
            Loglocal |
            Maxpreserve |
            Maxfilesize |
            Homenet |
            DefaultPacketSize |
            UpdateCron |
            AlertLogrotate |
            AlertSaveLogs |
            Mpmalgo |
            Detect |
            Syslog2 |
            SyslogEve |
            LogPayload |
            Verbosity |
            EveLog |
            ValidLifetime |
            Fwrules
        )
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "preferred_oldsa",
                    "type": PreferredOldsa,
                    "namespace": "",
                },
                {
                    "name": "disablevpnrules",
                    "type": Disablevpnrules,
                    "namespace": "",
                },
                {
                    "name": "passthrough_networks",
                    "type": PassthroughNetworks,
                    "namespace": "",
                },
                {
                    "name": "loglocal",
                    "type": Loglocal,
                    "namespace": "",
                },
                {
                    "name": "maxpreserve",
                    "type": Maxpreserve,
                    "namespace": "",
                },
                {
                    "name": "maxfilesize",
                    "type": Maxfilesize,
                    "namespace": "",
                },
                {
                    "name": "homenet",
                    "type": Homenet,
                    "namespace": "",
                },
                {
                    "name": "defaultPacketSize",
                    "type": DefaultPacketSize,
                    "namespace": "",
                },
                {
                    "name": "UpdateCron",
                    "type": UpdateCron,
                    "namespace": "",
                },
                {
                    "name": "AlertLogrotate",
                    "type": AlertLogrotate,
                    "namespace": "",
                },
                {
                    "name": "AlertSaveLogs",
                    "type": AlertSaveLogs,
                    "namespace": "",
                },
                {
                    "name": "MPMAlgo",
                    "type": Mpmalgo,
                    "namespace": "",
                },
                {
                    "name": "detect",
                    "type": Detect,
                    "namespace": "",
                },
                {
                    "name": "syslog",
                    "type": Syslog2,
                    "namespace": "",
                },
                {
                    "name": "syslog_eve",
                    "type": SyslogEve,
                    "namespace": "",
                },
                {
                    "name": "LogPayload",
                    "type": LogPayload,
                    "namespace": "",
                },
                {
                    "name": "verbosity",
                    "type": Verbosity,
                    "namespace": "",
                },
                {
                    "name": "eveLog",
                    "type": EveLog,
                    "namespace": "",
                },
                {
                    "name": "valid_lifetime",
                    "type": ValidLifetime,
                    "namespace": "",
                },
                {
                    "name": "fwrules",
                    "type": Fwrules,
                    "namespace": "",
                },
            ),
            "max_occurs": 12,
        },
    )
    interval: Interval | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    startdelay: Startdelay | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    mailserver: Mailserver | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    port: Port | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    choice_1: Iterable[
        (
            Username |
            Password |
            Ssl |
            Sslversion |
            Sslverify |
            Logfile |
            Statefile |
            EventqueuePath |
            EventqueueSlots |
            HttpdEnabled |
            HttpdUsername |
            HttpdPassword |
            HttpdPort |
            HttpdAllow |
            MmonitUrl |
            MmonitTimeout |
            MmonitRegisterCredentials |
            Stats |
            ActiveInterface |
            Dnssec |
            Dns64 |
            Dns64Prefix |
            Noarecords |
            Regdhcp |
            Regdhcpdomain |
            Regdhcpstatic |
            Noreglladdr6 |
            Noregrecords |
            Txtsupport |
            Cacheflush |
            LocalZoneType |
            OutgoingInterface |
            EnableWpad
        )
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "username",
                    "type": Username,
                    "namespace": "",
                },
                {
                    "name": "password",
                    "type": Password,
                    "namespace": "",
                },
                {
                    "name": "ssl",
                    "type": Ssl,
                    "namespace": "",
                },
                {
                    "name": "sslversion",
                    "type": Sslversion,
                    "namespace": "",
                },
                {
                    "name": "sslverify",
                    "type": Sslverify,
                    "namespace": "",
                },
                {
                    "name": "logfile",
                    "type": Logfile,
                    "namespace": "",
                },
                {
                    "name": "statefile",
                    "type": Statefile,
                    "namespace": "",
                },
                {
                    "name": "eventqueuePath",
                    "type": EventqueuePath,
                    "namespace": "",
                },
                {
                    "name": "eventqueueSlots",
                    "type": EventqueueSlots,
                    "namespace": "",
                },
                {
                    "name": "httpdEnabled",
                    "type": HttpdEnabled,
                    "namespace": "",
                },
                {
                    "name": "httpdUsername",
                    "type": HttpdUsername,
                    "namespace": "",
                },
                {
                    "name": "httpdPassword",
                    "type": HttpdPassword,
                    "namespace": "",
                },
                {
                    "name": "httpdPort",
                    "type": HttpdPort,
                    "namespace": "",
                },
                {
                    "name": "httpdAllow",
                    "type": HttpdAllow,
                    "namespace": "",
                },
                {
                    "name": "mmonitUrl",
                    "type": MmonitUrl,
                    "namespace": "",
                },
                {
                    "name": "mmonitTimeout",
                    "type": MmonitTimeout,
                    "namespace": "",
                },
                {
                    "name": "mmonitRegisterCredentials",
                    "type": MmonitRegisterCredentials,
                    "namespace": "",
                },
                {
                    "name": "stats",
                    "type": Stats,
                    "namespace": "",
                },
                {
                    "name": "active_interface",
                    "type": ActiveInterface,
                    "namespace": "",
                },
                {
                    "name": "dnssec",
                    "type": Dnssec,
                    "namespace": "",
                },
                {
                    "name": "dns64",
                    "type": Dns64,
                    "namespace": "",
                },
                {
                    "name": "dns64prefix",
                    "type": Dns64Prefix,
                    "namespace": "",
                },
                {
                    "name": "noarecords",
                    "type": Noarecords,
                    "namespace": "",
                },
                {
                    "name": "regdhcp",
                    "type": Regdhcp,
                    "namespace": "",
                },
                {
                    "name": "regdhcpdomain",
                    "type": Regdhcpdomain,
                    "namespace": "",
                },
                {
                    "name": "regdhcpstatic",
                    "type": Regdhcpstatic,
                    "namespace": "",
                },
                {
                    "name": "noreglladdr6",
                    "type": Noreglladdr6,
                    "namespace": "",
                },
                {
                    "name": "noregrecords",
                    "type": Noregrecords,
                    "namespace": "",
                },
                {
                    "name": "txtsupport",
                    "type": Txtsupport,
                    "namespace": "",
                },
                {
                    "name": "cacheflush",
                    "type": Cacheflush,
                    "namespace": "",
                },
                {
                    "name": "local_zone_type",
                    "type": LocalZoneType,
                    "namespace": "",
                },
                {
                    "name": "outgoing_interface",
                    "type": OutgoingInterface,
                    "namespace": "",
                },
                {
                    "name": "enable_wpad",
                    "type": EnableWpad,
                    "namespace": "",
                },
            ),
            "max_occurs": 17,
        },
    )
    version: str | None = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )
