from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.captiveportal import Captiveportal
from opnsense.models.cron import Cron
from opnsense.models.dhcrelay import Dhcrelay
from opnsense.models.firewall import Firewall
from opnsense.models.gateways_1 import Gateways1
from opnsense.models.ids import Ids
from opnsense.models.interfaces_1 import Interfaces1
from opnsense.models.ipsec import Ipsec
from opnsense.models.kea import Kea
from opnsense.models.monit import Monit
from opnsense.models.netflow import Netflow
from opnsense.models.open_vpn_1 import OpenVpn1
from opnsense.models.open_vpnexport import OpenVpnexport
from opnsense.models.swanctl import Swanctl
from opnsense.models.syslog_1 import Syslog1
from opnsense.models.traffic_shaper import TrafficShaper
from opnsense.models.trust import Trust
from opnsense.models.unboundplus import Unboundplus
from opnsense.models.wireguard import Wireguard

__NAMESPACE__ = "https://opnsense.org/config"


class Opnsense1(BaseModel):
    class Meta:
        name = "OPNsense"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    captiveportal: Captiveportal | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    cron: Cron | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    dhcrelay: Dhcrelay | None = field(
        default=None,
        metadata={
            "name": "DHCRelay",
            "type": "Element",
            "namespace": "",
        },
    )
    firewall: Firewall | None = field(
        default=None,
        metadata={
            "name": "Firewall",
            "type": "Element",
            "namespace": "",
        },
    )
    netflow: Netflow | None = field(
        default=None,
        metadata={
            "name": "Netflow",
            "type": "Element",
            "namespace": "",
        },
    )
    ids: Ids | None = field(
        default=None,
        metadata={
            "name": "IDS",
            "type": "Element",
            "namespace": "",
        },
    )
    ipsec: Ipsec | None = field(
        default=None,
        metadata={
            "name": "IPsec",
            "type": "Element",
            "namespace": "",
        },
    )
    swanctl: Swanctl | None = field(
        default=None,
        metadata={
            "name": "Swanctl",
            "type": "Element",
            "namespace": "",
        },
    )
    interfaces: Interfaces1 | None = field(
        default=None,
        metadata={
            "name": "Interfaces",
            "type": "Element",
            "namespace": "",
        },
    )
    kea: Kea | None = field(
        default=None,
        metadata={
            "name": "Kea",
            "type": "Element",
            "namespace": "",
        },
    )
    monit: Monit | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    open_vpnexport: OpenVpnexport | None = field(
        default=None,
        metadata={
            "name": "OpenVPNExport",
            "type": "Element",
            "namespace": "",
        },
    )
    open_vpn: OpenVpn1 | None = field(
        default=None,
        metadata={
            "name": "OpenVPN",
            "type": "Element",
            "namespace": "",
        },
    )
    gateways: Gateways1 | None = field(
        default=None,
        metadata={
            "name": "Gateways",
            "type": "Element",
            "namespace": "",
        },
    )
    syslog: Syslog1 | None = field(
        default=None,
        metadata={
            "name": "Syslog",
            "type": "Element",
            "namespace": "",
        },
    )
    traffic_shaper: TrafficShaper | None = field(
        default=None,
        metadata={
            "name": "TrafficShaper",
            "type": "Element",
            "namespace": "",
        },
    )
    trust: Trust | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    unboundplus: Unboundplus | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    wireguard: Wireguard = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
