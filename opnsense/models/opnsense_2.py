from collections.abc import Iterable
from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.bridges import Bridges
from opnsense.models.ca import Ca
from opnsense.models.cert import Cert
from opnsense.models.dhcpd import Dhcpd
from opnsense.models.dhcpdv6 import Dhcpdv6
from opnsense.models.enable import Enable
from opnsense.models.gateways_2 import Gateways2
from opnsense.models.gifs import Gifs
from opnsense.models.gres import Gres
from opnsense.models.hasync import Hasync
from opnsense.models.ifgroups import Ifgroups
from opnsense.models.interfaces_2 import Interfaces2
from opnsense.models.item import Item
from opnsense.models.laggs import Laggs
from opnsense.models.load_balancer import LoadBalancer
from opnsense.models.openvpn_2 import Openvpn2
from opnsense.models.opnsense_1 import Opnsense1
from opnsense.models.outbound import Outbound
from opnsense.models.ppps import Ppps
from opnsense.models.prefer import Prefer
from opnsense.models.revision import Revision
from opnsense.models.rule import Rule
from opnsense.models.snmpd import Snmpd
from opnsense.models.staticroutes import Staticroutes
from opnsense.models.system import System
from opnsense.models.theme import Theme
from opnsense.models.trigger_initial_wizard import TriggerInitialWizard
from opnsense.models.unbound import Unbound
from opnsense.models.virtualip import Virtualip
from opnsense.models.vlans import Vlans
from opnsense.models.widgets import Widgets
from opnsense.models.wireless import Wireless

__NAMESPACE__ = "https://opnsense.org/config"


class Opnsense2(BaseModel):
    class Meta:
        name = "opnsense"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    trigger_initial_wizard: TriggerInitialWizard | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    theme: Theme = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    item: Iterable[Item] = field(
        default_factory=list,
        metadata={
            "wrapper": "sysctl",
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
    system: System = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    interfaces: Interfaces2 = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dhcpd: Dhcpd = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    unbound: Unbound | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    snmpd: Snmpd = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    outbound: Outbound = field(
        metadata={
            "wrapper": "nat",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    rule: Iterable[Rule] = field(
        default_factory=list,
        metadata={
            "wrapper": "filter",
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
    enable: Enable = field(
        metadata={
            "wrapper": "rrd",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    load_balancer: LoadBalancer | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    prefer: Prefer = field(
        metadata={
            "wrapper": "ntpd",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    widgets: Widgets | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    revision: Revision | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    gateways: Gateways2 | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    opnsense: Opnsense1 | None = field(
        default=None,
        metadata={
            "name": "OPNsense",
            "type": "Element",
            "namespace": "",
        },
    )
    hasync: Hasync | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    ifgroups: Ifgroups | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    gifs: Gifs | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    gres: Gres | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    laggs: Laggs | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    virtualip: Virtualip | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    vlans: Vlans | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    openvpn: Openvpn2 | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    staticroutes: Staticroutes | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    bridges: Bridges | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    ppps: Ppps | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    wireless: Wireless | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    ca: Ca | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    dhcpdv6: Dhcpdv6 | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    cert: Cert | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
