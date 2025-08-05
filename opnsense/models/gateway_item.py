from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.defaultgw import Defaultgw
from opnsense.models.descr import Descr
from opnsense.models.fargw import Fargw
from opnsense.models.gateway import Gateway
from opnsense.models.interface import Interface
from opnsense.models.interval import Interval
from opnsense.models.ipprotocol import Ipprotocol
from opnsense.models.monitor_disable import MonitorDisable
from opnsense.models.name import Name
from opnsense.models.weight import Weight

__NAMESPACE__ = "https://opnsense.org/config"


class GatewayItem(BaseModel):
    class Meta:
        name = "gateway_item"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    descr: Descr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    defaultgw: Defaultgw = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ipprotocol: Ipprotocol = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    interface: Interface = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    gateway: Gateway = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    monitor_disable: MonitorDisable = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    name: Name = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    interval: Interval = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    weight: Weight = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    fargw: Fargw = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
