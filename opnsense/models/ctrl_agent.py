from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.general import General

__NAMESPACE__ = "https://opnsense.org/config"


class CtrlAgent(BaseModel):
    class Meta:
        name = "ctrl_agent"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
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
