from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.instances import Instances
from opnsense.models.overwrites import Overwrites
from opnsense.models.static_keys import StaticKeys

__NAMESPACE__ = "https://opnsense.org/config"


class OpenVpn1(BaseModel):
    class Meta:
        name = "OpenVPN"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    overwrites: Overwrites = field(
        metadata={
            "name": "Overwrites",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    instances: Instances = field(
        metadata={
            "name": "Instances",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    static_keys: StaticKeys = field(
        metadata={
            "name": "StaticKeys",
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
