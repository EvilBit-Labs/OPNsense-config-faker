from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.loopbacks import Loopbacks
from opnsense.models.neighbors import Neighbors
from opnsense.models.vxlans import Vxlans

__NAMESPACE__ = "https://opnsense.org/config"


class Interfaces1(BaseModel):
    class Meta:
        name = "Interfaces"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    loopbacks: Loopbacks = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    neighbors: Neighbors = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    vxlans: Vxlans = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
