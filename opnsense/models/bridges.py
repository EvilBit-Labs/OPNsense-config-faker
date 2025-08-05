from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.bridged import Bridged

__NAMESPACE__ = "https://opnsense.org/config"


class Bridges(BaseModel):
    class Meta:
        name = "bridges"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    bridged: Bridged = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
