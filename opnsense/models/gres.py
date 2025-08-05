from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.gre import Gre

__NAMESPACE__ = "https://opnsense.org/config"


class Gres(BaseModel):
    class Meta:
        name = "gres"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    gre: Gre = field(
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
