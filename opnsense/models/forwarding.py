from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.enabled import Enabled

__NAMESPACE__ = "https://opnsense.org/config"


class Forwarding(BaseModel):
    class Meta:
        name = "forwarding"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enabled: Enabled = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
