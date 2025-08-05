from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.vlan import Vlan

__NAMESPACE__ = "https://opnsense.org/config"


class Vlans(BaseModel):
    class Meta:
        name = "vlans"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    vlan: Vlan = field(
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
