from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.description import Description
from opnsense.models.time import Time
from opnsense.models.username import Username

__NAMESPACE__ = "https://opnsense.org/config"


class Revision(BaseModel):
    class Meta:
        name = "revision"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    username: Username = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    time: Time = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    description: Description = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
