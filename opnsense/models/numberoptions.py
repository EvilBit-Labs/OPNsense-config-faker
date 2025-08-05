from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.item import Item

__NAMESPACE__ = "https://opnsense.org/config"


class Numberoptions(BaseModel):
    class Meta:
        name = "numberoptions"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    item: Item = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
