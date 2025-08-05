from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.url import Url

__NAMESPACE__ = "https://opnsense.org/config"


class Geoip(BaseModel):
    class Meta:
        name = "geoip"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    url: Url = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
