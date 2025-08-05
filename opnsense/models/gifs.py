from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.gif import Gif

__NAMESPACE__ = "https://opnsense.org/config"


class Gifs(BaseModel):
    class Meta:
        name = "gifs"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    gif: Gif = field(
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
