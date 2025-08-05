from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.templates import Templates
from opnsense.models.zones import Zones

__NAMESPACE__ = "https://opnsense.org/config"


class Captiveportal(BaseModel):
    class Meta:
        name = "captiveportal"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    zones: Zones = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    templates: Templates = field(
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
