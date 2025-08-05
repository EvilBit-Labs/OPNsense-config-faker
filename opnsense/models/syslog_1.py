from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.destinations import Destinations
from opnsense.models.general import General

__NAMESPACE__ = "https://opnsense.org/config"


class Syslog1(BaseModel):
    class Meta:
        name = "Syslog"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    destinations: Destinations = field(
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
