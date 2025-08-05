from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.enable import Enable

__NAMESPACE__ = "https://opnsense.org/config"


class Rrd(BaseModel):
    class Meta:
        name = "rrd"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enable: Enable = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
