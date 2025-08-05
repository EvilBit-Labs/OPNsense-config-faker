from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.general import General

__NAMESPACE__ = "https://opnsense.org/config"


class Trust(BaseModel):
    class Meta:
        name = "trust"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
