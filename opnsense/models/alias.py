from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.aliases import Aliases
from opnsense.models.url import Url

__NAMESPACE__ = "https://opnsense.org/config"


class Alias(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    url: Url = field(
        metadata={
            "wrapper": "geoip",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    aliases: Aliases = field(
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
