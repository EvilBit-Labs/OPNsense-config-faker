from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.interval import Interval

__NAMESPACE__ = "https://opnsense.org/config"


class Bogons(BaseModel):
    class Meta:
        name = "bogons"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    interval: Interval = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
