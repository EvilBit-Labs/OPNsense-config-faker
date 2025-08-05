from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.outbound import Outbound

__NAMESPACE__ = "https://opnsense.org/config"


class Nat(BaseModel):
    class Meta:
        name = "nat"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    outbound: Outbound = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
