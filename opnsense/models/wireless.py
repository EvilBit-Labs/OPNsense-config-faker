from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.clone import Clone

__NAMESPACE__ = "https://opnsense.org/config"


class Wireless(BaseModel):
    class Meta:
        name = "wireless"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    clone: Clone = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
