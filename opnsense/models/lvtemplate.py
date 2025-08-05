from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.templates import Templates

__NAMESPACE__ = "https://opnsense.org/config"


class Lvtemplate(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    templates: Templates = field(
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
