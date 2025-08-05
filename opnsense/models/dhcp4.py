from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.general import General
from opnsense.models.ha import Ha
from opnsense.models.ha_peers import HaPeers
from opnsense.models.reservations import Reservations
from opnsense.models.subnets import Subnets

__NAMESPACE__ = "https://opnsense.org/config"


class Dhcp4(BaseModel):
    class Meta:
        name = "dhcp4"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ha: Ha = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    subnets: Subnets = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    reservations: Reservations = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ha_peers: HaPeers = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
