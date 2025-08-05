from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.ppp import Ppp

__NAMESPACE__ = "https://opnsense.org/config"


class Ppps(BaseModel):
    class Meta:
        name = "ppps"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    ppp: Ppp = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
