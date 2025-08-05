from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.disablechecksumoffloading import Disablechecksumoffloading
from opnsense.models.disableconsolemenu import Disableconsolemenu
from opnsense.models.disablelargereceiveoffloading import (
    Disablelargereceiveoffloading,
)
from opnsense.models.disablenatreflection import Disablenatreflection
from opnsense.models.disablesegmentationoffloading import (
    Disablesegmentationoffloading,
)
from opnsense.models.disablevlanhwfilter import Disablevlanhwfilter
from opnsense.models.dnsallowoverride import Dnsallowoverride
from opnsense.models.dnsserver import Dnsserver
from opnsense.models.domain import Domain
from opnsense.models.firmware import Firmware
from opnsense.models.group import Group
from opnsense.models.hostname import Hostname
from opnsense.models.interval import Interval
from opnsense.models.ipv6allow import Ipv6Allow
from opnsense.models.language import Language
from opnsense.models.lb_use_sticky import LbUseSticky
from opnsense.models.netflowbackup import Netflowbackup
from opnsense.models.nextgid import Nextgid
from opnsense.models.nextuid import Nextuid
from opnsense.models.optimization import Optimization
from opnsense.models.pf_share_forward import PfShareForward
from opnsense.models.powerd_ac_mode import PowerdAcMode
from opnsense.models.powerd_battery_mode import PowerdBatteryMode
from opnsense.models.powerd_normal_mode import PowerdNormalMode
from opnsense.models.rrdbackup import Rrdbackup
from opnsense.models.timeservers import Timeservers
from opnsense.models.timezone import Timezone
from opnsense.models.user import User
from opnsense.models.usevirtualterminal import Usevirtualterminal
from opnsense.models.webgui import Webgui

__NAMESPACE__ = "https://opnsense.org/config"


class System(BaseModel):
    class Meta:
        name = "system"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    optimization: Optimization = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    hostname: Hostname = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    domain: Domain = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dnsallowoverride: Dnsallowoverride | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    group_element: Group = field(
        metadata={
            "name": "group",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    user: User = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    nextuid: Nextuid = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    nextgid: Nextgid = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    timezone: Timezone = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    timeservers: Timeservers = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    webgui: Webgui = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disablenatreflection: Disablenatreflection = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    usevirtualterminal: Usevirtualterminal = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disableconsolemenu: Disableconsolemenu = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disablevlanhwfilter: Disablevlanhwfilter = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disablechecksumoffloading: Disablechecksumoffloading = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disablesegmentationoffloading: Disablesegmentationoffloading = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disablelargereceiveoffloading: Disablelargereceiveoffloading = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ipv6allow: Ipv6Allow = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    powerd_ac_mode: PowerdAcMode = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    powerd_battery_mode: PowerdBatteryMode = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    powerd_normal_mode: PowerdNormalMode = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    interval: Interval = field(
        metadata={
            "wrapper": "bogons",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pf_share_forward: PfShareForward = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    lb_use_sticky: LbUseSticky = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    group: Group = field(
        metadata={
            "wrapper": "ssh",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    rrdbackup: Rrdbackup = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    netflowbackup: Netflowbackup = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    firmware: Firmware | None = field(
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
    language: Language | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
