from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.prefer import Prefer

__NAMESPACE__ = "https://opnsense.org/config"


class Ntpd(BaseModel):
    class Meta:
        name = "ntpd"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    prefer: Prefer = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
