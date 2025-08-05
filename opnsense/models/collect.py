from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.enable import Enable

__NAMESPACE__ = "https://opnsense.org/config"


class Collect(BaseModel):
    class Meta:
        name = "collect"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enable: Enable = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
