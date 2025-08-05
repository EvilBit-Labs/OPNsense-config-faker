from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.lagg import Lagg

__NAMESPACE__ = "https://opnsense.org/config"


class Laggs(BaseModel):
    class Meta:
        name = "laggs"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    lagg: Lagg = field(
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
