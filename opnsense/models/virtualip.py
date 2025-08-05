from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.vip import Vip

__NAMESPACE__ = "https://opnsense.org/config"


class Virtualip(BaseModel):
    class Meta:
        name = "virtualip"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    vip: Vip = field(
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
